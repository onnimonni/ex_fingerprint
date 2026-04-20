pub mod emulation;
pub mod plan;
pub mod stub;

use crate::error::RequestError;
use crate::request::RequestPayload;
use crate::response::ResponsePayload;

pub trait BrowserTransport {
    fn dispatch(&self, payload: RequestPayload) -> Result<ResponsePayload, RequestError>;
}

pub fn dispatch(payload: RequestPayload) -> Result<ResponsePayload, RequestError> {
    let transport = stub::StubTransport;
    transport.dispatch(payload)
}
