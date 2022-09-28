use std::cell::RefCell;

use crate::{remote_api_client::RemoteApiClientInterface, RemoteAPIError};
use serde_json::Value as JsonValue;

pub struct MockRemoteAPIClient {
    pub payload: RefCell<Vec<u8>>,
    pub result: RefCell<JsonValue>,
}

impl MockRemoteAPIClient {
    pub fn get_payload(&self) -> Vec<u8> {
        self.payload.borrow_mut().clone()
    }
}

impl RemoteApiClientInterface for MockRemoteAPIClient {
    fn send_raw_request(&self, request: Vec<u8>) -> Result<JsonValue, RemoteAPIError> {
        *self.payload.borrow_mut() = request;
        let result = self.result.clone().into_inner();
        Ok(result)
    }
}

pub fn assert_payload(client: &MockRemoteAPIClient, payload: Vec<u8>) {
    assert_eq!(client.get_payload(), payload)
}
macro_rules! assert_payload {
    ($client:ident,$payload:literal) => {
        assert_eq!($client.get_payload(), $payload.to_vec());
    };
}
