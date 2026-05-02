use serde::Serialize;
use serde_bytes::ByteBuf;
use std::collections::BTreeMap;

#[derive(Debug, Serialize)]
pub struct ResponsePayload {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    // Erlang binary — emit raw bytes (no UTF-8 lossy decode) so gzipped
    // sitemaps / images / any non-text payload survive the NIF boundary intact.
    pub body: ByteBuf,
    pub remote_address: String,
    pub diagnostics: BTreeMap<String, serde_json::Value>,
}
