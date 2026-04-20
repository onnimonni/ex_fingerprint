use crate::error::RequestError;
use crate::profile::{resolve, BrowserProfile};
use crate::request::RequestPayload;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct RequestPlan {
    pub method: String,
    pub url: String,
    pub profile_id: String,
    pub headers: Vec<(String, String)>,
    pub alpn: Vec<String>,
    pub pseudo_header_order: Vec<String>,
    pub http2_settings: Vec<(String, u32)>,
    pub proxy_tunnel: Option<String>,
}

pub fn build(payload: &RequestPayload) -> Result<RequestPlan, RequestError> {
    let profile = payload
        .profile_data
        .clone()
        .or_else(|| resolve(&payload.profile))
        .ok_or_else(|| RequestError::UnsupportedProfile(payload.profile.clone()))?;

    Ok(RequestPlan {
        method: payload.method.clone(),
        url: payload.url.clone(),
        profile_id: profile.id.clone(),
        headers: effective_headers(&profile, &payload.headers),
        alpn: profile.tls.alpn.clone(),
        pseudo_header_order: profile
            .http2
            .pseudo_header_order
            .iter()
            .cloned()
            .collect(),
        http2_settings: profile
            .http2
            .settings
            .iter()
            .map(|(name, value)| (name.clone(), *value))
            .collect(),
        proxy_tunnel: payload.proxy_tunnel.clone(),
    })
}

fn effective_headers(
    profile: &BrowserProfile,
    request_headers: &[(String, String)],
) -> Vec<(String, String)> {
    let mut merged: Vec<(String, String)> = profile
        .headers
        .iter()
        .cloned()
        .collect();

    for (name, value) in request_headers {
        let lname = name.to_ascii_lowercase();
        if let Some(existing) = merged.iter_mut().find(|(k, _)| k == &lname) {
            existing.1 = value.clone();
        } else {
            merged.push((lname, value.clone()));
        }
    }

    merged
}
