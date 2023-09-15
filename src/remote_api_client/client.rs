use ciborium::value::Value as CborValue;
use serde_json::Value as JsonValue;
use uuid::Uuid;

use crate::remote_api_client::RemoteApiClientInterface;
use crate::sim::Sim;
use crate::sim_ik::SimIk;
use crate::zmq_requests::{RawRequest, ZmqRequest};
use crate::{log_utils, RemoteAPIError, RemoteApiClientParams};

const ZMQ_RECV_FLAG_NONE: i32 = 0;

pub struct RemoteApiClient {
    rpc_socket: zmq::Socket,
    cnt_socket: zmq::Socket,
    id: Uuid,
}

impl RemoteApiClient {
    pub fn new(params: RemoteApiClientParams) -> Result<RemoteApiClient, RemoteAPIError> {
        let rpc_address = format!(
            "tcp://{host}:{port}",
            host = params.host,
            port = params.rpc_port
        );
        let cnt_address = format!(
            "tcp://{host}:{port}",
            host = params.host,
            port = params.cnt_port
        );

        log::debug!(
            "connecting on  rpc_address: {} cnt_address: {}",
            rpc_address,
            cnt_address
        );

        let context = zmq::Context::new();

        let rpc_socket = Self::convert_result(context.socket(zmq::SocketType::REQ))?;

        let cnt_socket = Self::convert_result(context.socket(zmq::SocketType::SUB))?;
        cnt_socket.set_subscribe(b"")?;
        cnt_socket.set_conflate(true)?;

        rpc_socket.connect(&rpc_address)?;
        cnt_socket.connect(&cnt_address)?;

        Ok(RemoteApiClient {
            id: Uuid::new_v4(),
            rpc_socket,
            cnt_socket,
        })
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

    // pub fn send_request<R: RawRequest>(&self, request: R) -> Result<JsonValue, Error> {
    //     let payload = request.to_raw_request();
    //
    //     let result = self.call(payload)?;
    //
    //     let decoded: CborValue = ciborium::de::from_reader(result.as_slice()).unwrap();
    //
    //     let json = serde_json::json!(decoded);
    //     log::trace!("Json response: {}", json);
    //
    //     Ok(json)
    // }

    pub fn get_object_client(&self, name: String) -> Result<JsonValue, RemoteAPIError> {
        let request = ZmqRequest::remote_api_info(name);

        self.send_raw_request(request.to_raw_request())
    }

    pub fn set_stepping(&self, enable: bool) -> Result<(), RemoteAPIError> {
        let request = ZmqRequest::set_stepping(enable, self.id.to_string());

        self.send_raw_request(request.to_raw_request())?;

        Ok(())
    }

    pub fn step(&self, wait: bool) -> Result<(), RemoteAPIError> {
        let request = ZmqRequest::step(self.id.to_string());

        match wait {
            true => {
                self.get_step_count(false)?;
                self.send_raw_request(request.to_raw_request())?;
                self.get_step_count(true)?;
            }
            false => {
                self.send_raw_request(request.to_raw_request())?;
            }
        }

        Ok(())
    }
    fn get_step_count(&self, wait: bool) -> Result<(), RemoteAPIError> {
        let flag = match wait {
            true => ZMQ_RECV_FLAG_NONE,
            false => zmq::DONTWAIT,
        };
        log::trace!("receive a message for cnt socket");

        let mut message = zmq::Message::new();
        let _ = self.cnt_socket.recv(&mut message, flag);

        let message = (*message).to_vec();

        Self::log_step_count(message);

        Ok(())
    }

    fn log_step_count(message: Vec<u8>) {
        const SIZE: usize = 4;
        if message.len() == SIZE {
            let mut slice: [u8; SIZE] = [0, 0, 0, 0];
            slice.copy_from_slice(&message);
            let counter = i32::from_be_bytes(slice);
            log::debug!("step count: {}", counter);
        } else {
            log::warn!(
                "Step count message doesn't have 4 bytes, msg: {:?}, size: {}",
                message,
                message.len()
            );
        }
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
}

impl RemoteApiClientInterface for &RemoteApiClient {
    fn send_raw_request(&self, request: Vec<u8>) -> Result<JsonValue, RemoteAPIError> {
        let result = self.call(request)?;

        let decoded: CborValue = ciborium::de::from_reader(result.as_slice()).unwrap();

        let json = serde_json::json!(decoded);
        log::trace!("Json response: {}", json);

        Ok(json)
    }
}

impl Sim for RemoteApiClient {}
impl Sim for &RemoteApiClient {}

impl SimIk for RemoteApiClient {}
impl SimIk for &RemoteApiClient {}
