use std::collections::HashMap;

use ciborium::value::Value as CborValue;
use serde_json::Value as JsonValue;
use uuid::Uuid;

use crate::remote_api_client::RemoteApiClientInterface;
use crate::sim::Sim;
use crate::sim::SimIK;
use crate::zmq_protocol::{RawRequest, ZmqRequest};
use crate::{log_utils, RemoteAPIError, RemoteApiClientParams};

const ZMQ_RECV_FLAG_NONE: i32 = 0;

pub struct RemoteApiClient {
    rpc_socket: zmq::Socket,
    end_point: String,
    id: Uuid,
    callbacks: HashMap<String, Box<dyn Fn(JsonValue) -> JsonValue>>,
}

impl RemoteApiClient {
    pub fn new(params: RemoteApiClientParams) -> Result<RemoteApiClient, RemoteAPIError> {
        let rpc_address = format!(
            "tcp://{host}:{port}",
            host = params.host,
            port = params.rpc_port
        );

        log::debug!("connecting on  rpc_address: {} ", rpc_address,);

        let context = zmq::Context::new();

        let rpc_socket = Self::convert_result(context.socket(zmq::SocketType::REQ))?;

        rpc_socket.connect(&rpc_address)?;

        Ok(RemoteApiClient {
            id: Uuid::new_v4(),
            rpc_socket,
            end_point: rpc_address,
            callbacks: HashMap::new(),
        })
    }

    pub fn register_callback(&mut self, name: String, func: Box<dyn Fn(JsonValue) -> JsonValue>) {
        self.callbacks.insert(name, func);
    }

    fn convert_result<T>(result: Result<T, zmq::Error>) -> Result<T, RemoteAPIError> {
        match result {
            Ok(t) => Ok(t),
            Err(e) => Err(RemoteAPIError::from(e)),
        }
    }

    pub fn call(&self, payload: Vec<u8>) -> Result<Vec<u8>, RemoteAPIError> {
        log::trace!("Raw request: {}", log_utils::to_byte_array_string(&payload));

        self.rpc_socket.send(payload, zmq::DONTWAIT)?;

        let mut message = zmq::Message::new();

        self.rpc_socket.recv(&mut message, ZMQ_RECV_FLAG_NONE)?;

        let message = (*message).to_vec();

        log::trace!(
            "Raw response: {:?}",
            log_utils::to_byte_array_string(&message)
        );

        Ok(message)
    }

    pub fn get_object(&self, name: String) -> Result<JsonValue, RemoteAPIError> {
        let request = ZmqRequest::remote_api_info(name, self.get_uuid());

        self.send_raw_request(request.to_raw_request())
    }
}

impl RemoteApiClientInterface for RemoteApiClient {
    fn send_raw_request(&self, request: Vec<u8>) -> Result<JsonValue, RemoteAPIError> {
        let result = self.call(request)?;

        let decoded: CborValue = ciborium::de::from_reader(result.as_slice()).unwrap();

        let json = serde_json::json!(decoded);
        log::trace!("Json response: {}", json);

        Ok(json)
    }

    fn get_uuid(&self) -> String {
        self.id.to_string()
    }

    fn get_callback(&self, function_name: &str) -> Option<&Box<dyn Fn(JsonValue) -> JsonValue>> {
        self.callbacks.get(function_name)
    }
}

impl RemoteApiClientInterface for &RemoteApiClient {
    fn send_raw_request(&self, request: Vec<u8>) -> Result<JsonValue, RemoteAPIError> {
        let result = self.call(request)?;

        let decoded: CborValue = ciborium::de::from_reader(result.as_slice()).unwrap();

        let json = serde_json::json!(decoded);
        log::trace!("Json response: {}", json);

        Ok(json)
    }

    fn get_uuid(&self) -> String {
        self.id.to_string()
    }

    fn get_callback(&self, function_name: &str) -> Option<&Box<dyn Fn(JsonValue) -> JsonValue>> {
        self.callbacks.get(function_name)
    }
}
impl Drop for RemoteApiClient {
    fn drop(&mut self) {
        let end_request = ZmqRequest::end_request(self.get_uuid());

        log::debug!("Remote Client dropped");
        self.rpc_socket
            .send(end_request.to_raw_request(), zmq::DONTWAIT)
            .unwrap();
        self.rpc_socket.disconnect(&self.end_point).unwrap();
    }
}

impl Sim for RemoteApiClient {}
impl Sim for &RemoteApiClient {}

impl SimIK for RemoteApiClient {}
impl SimIK for &RemoteApiClient {}
