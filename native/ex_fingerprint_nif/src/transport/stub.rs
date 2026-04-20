use crate::error::RequestError;
use crate::profile::{self, BrowserProfile};
use crate::request::RequestPayload;
use crate::response::ResponsePayload;
use crate::transport::emulation;
use crate::transport::plan;
use crate::transport::BrowserTransport;
use serde_json::Value;
use std::fs;
use std::collections::BTreeMap;
use std::time::Duration;
use tokio::runtime::Builder as RuntimeBuilder;
use wreq::Client;
use wreq::tls::trust::CertStore;

pub struct StubTransport;

impl BrowserTransport for StubTransport {
    fn dispatch(&self, payload: RequestPayload) -> Result<ResponsePayload, RequestError> {
        let plan = plan::build(&payload)?;
        let profile = payload
            .profile_data
            .clone()
            .or_else(|| profile::resolve(&payload.profile))
            .ok_or_else(|| RequestError::UnsupportedProfile(payload.profile.clone()))?;

        let runtime = RuntimeBuilder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|err| RequestError::Io(err.to_string()))?;

        runtime.block_on(dispatch_async(&plan, &profile, &payload))
    }
}

async fn dispatch_async(
    plan: &plan::RequestPlan,
    profile: &BrowserProfile,
    payload: &RequestPayload,
) -> Result<ResponsePayload, RequestError> {
    let emulation = emulation::build(plan, profile)?;
    let client = client_from_payload(payload)?;

    let method = http::Method::from_bytes(uppercase_method(&plan.method).as_bytes())
        .map_err(|err| RequestError::Protocol(err.to_string()))?;

    let mut request = client
        .request(method, plan.url.as_str())
        .emulation(emulation)
        .timeout(Duration::from_secs(45));

    if let Some(body) = payload.body.as_ref() {
        request = request.body(body.clone());
    }

    let response = request
        .send()
        .await
        .map_err(|err| RequestError::Protocol(err.to_string()))?;

    let status = response.status().as_u16();
    let version = response.version();
    let remote_address = response
        .remote_addr()
        .map(|addr| addr.to_string())
        .unwrap_or_default();
    let headers = response
        .headers()
        .iter()
        .map(|(name, value)| {
            (
                name.as_str().to_ascii_lowercase(),
                value.to_str().unwrap_or_default().to_string(),
            )
        })
        .collect::<Vec<_>>();
    let body = response
        .text()
        .await
        .map_err(|err| RequestError::Protocol(err.to_string()))?;

    Ok(ResponsePayload {
        status,
        headers,
        body,
        remote_address,
        diagnostics: diagnostics(plan, payload, profile, version),
    })
}

fn client_from_payload(payload: &RequestPayload) -> Result<Client, RequestError> {
    let mut builder = Client::builder()
        .connect_timeout(Duration::from_secs(15))
        .no_proxy();

    if let Some(path) = metadata_string(&payload.metadata, "ca_cert_file") {
        let pem = fs::read(path).map_err(|err| RequestError::Io(err.to_string()))?;
        let store =
            CertStore::from_pem_stack(&pem).map_err(|err| RequestError::Protocol(err.to_string()))?;
        builder = builder.tls_cert_store(store);
    } else if let Some(pem) = metadata_string(&payload.metadata, "ca_cert_pem") {
        let store = CertStore::from_pem_stack(pem.as_bytes())
            .map_err(|err| RequestError::Protocol(err.to_string()))?;
        builder = builder.tls_cert_store(store);
    }

    if let Some(enabled) = metadata_bool(&payload.metadata, "tls_cert_verification") {
        builder = builder.tls_cert_verification(enabled);
    }

    if let Some(enabled) = metadata_bool(&payload.metadata, "tls_verify_hostname") {
        builder = builder.tls_verify_hostname(enabled);
    }

    builder
        .build()
        .map_err(|err| RequestError::Protocol(err.to_string()))
}

fn diagnostics(
    plan: &plan::RequestPlan,
    payload: &RequestPayload,
    profile: &BrowserProfile,
    version: http::Version,
) -> BTreeMap<String, Value> {
    let mut diagnostics = BTreeMap::new();

    diagnostics.insert(
        "transport".to_string(),
        Value::String("wreq_boringssl".to_string()),
    );
    diagnostics.insert("profile_id".to_string(), Value::String(plan.profile_id.clone()));
    diagnostics.insert(
        "profile_version".to_string(),
        Value::String(profile.version.to_string()),
    );
    diagnostics.insert(
        "profile_alpn".to_string(),
        Value::Array(plan.alpn.iter().cloned().map(Value::String).collect()),
    );
    diagnostics.insert(
        "http_version".to_string(),
        Value::String(emulation::version_name(version).to_string()),
    );
    diagnostics.insert(
        "negotiated_alpn".to_string(),
        Value::String(emulation::protocol_name(version).to_string()),
    );
    diagnostics.insert(
        "pseudo_header_order".to_string(),
        Value::Array(
            plan.pseudo_header_order
                .iter()
                .cloned()
                .map(Value::String)
                .collect(),
        ),
    );
    diagnostics.insert(
        "http2_settings".to_string(),
        Value::Array(
            plan.http2_settings
                .iter()
                .map(|(name, value)| {
                    Value::Array(vec![
                        Value::String(name.clone()),
                        Value::Number((*value as u64).into()),
                    ])
                })
                .collect(),
        ),
    );
    diagnostics.insert(
        "request_header_count".to_string(),
        Value::Number((plan.headers.len() as u64).into()),
    );
    diagnostics.insert("has_body".to_string(), Value::Bool(payload.body.is_some()));
    diagnostics.insert(
        "proxy_tunnel_requested".to_string(),
        Value::Bool(payload.proxy_tunnel.is_some()),
    );
    diagnostics.insert(
        "metadata_keys".to_string(),
        Value::Array(
            payload
                .metadata
                .keys()
                .cloned()
                .map(Value::String)
                .collect(),
        ),
    );

    diagnostics
}

fn uppercase_method(method: &str) -> String {
    method.to_ascii_uppercase()
}

fn metadata_string<'a>(metadata: &'a BTreeMap<String, Value>, key: &str) -> Option<&'a str> {
    metadata.get(key)?.as_str()
}

fn metadata_bool(metadata: &BTreeMap<String, Value>, key: &str) -> Option<bool> {
    metadata.get(key)?.as_bool()
}
