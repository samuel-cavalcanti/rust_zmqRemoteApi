use std::cell::RefCell;

use crate::{
    remote_api_client::RemoteApiClientInterface,
    sim::{Sim, SimIK},
    RemoteAPIError,
};
use serde_json::{json, Value as JsonValue};

pub struct MockRemoteAPIClient {
    pub payload: RefCell<Vec<u8>>,
    pub result: RefCell<JsonValue>,
    pub uuid: String,
}

impl MockRemoteAPIClient {
    pub fn new_sucess() -> MockRemoteAPIClient {
        MockRemoteAPIClient {
            payload: RefCell::new(vec![]),
            result: RefCell::new(json!({"ret":[1]})),
            uuid: "8a7e3cf4-ae84-4b29-9af3-1e87930b7971".into(),
        }
    }
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

    fn get_uuid(&self) -> String {
        self.uuid.clone()
    }

    fn get_callback(&self, _function_name: &str) -> Option<&Box<dyn Fn(JsonValue) -> JsonValue>> {
        None
    }
}

impl Sim for MockRemoteAPIClient {}
impl SimIK for MockRemoteAPIClient {}

macro_rules! assert_payload {
    ($client:ident,$payload:literal) => {
        assert_eq!(
            $client.get_payload(),
            $payload.to_vec(),
            "payload: {}",
            crate::log_utils::to_byte_array_string(&$client.get_payload())
        );
    };
}
