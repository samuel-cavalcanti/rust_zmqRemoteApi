use ciborium::{value::Value as CborValue};
use serde_json::Value as JsonValue;
use uuid::Uuid;
use zmq::Error;

use super::cbor_to_json;
use crate::zmq_requests::{RawRequest, ZmqRequest};
use crate::{log_utils, RemoteApiClientParams};

const ZMQ_RECV_FLAG_NONE: i32 = 0;

pub struct RemoteApiClient {
    context: zmq::Context,
    rpc_socket: zmq::Socket,
    cnt_socket: zmq::Socket,
    id: Uuid,
}

impl RemoteApiClient {
    pub fn new(params: RemoteApiClientParams) -> Result<RemoteApiClient, Error> {
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

        let rpc_socket = context.socket(zmq::SocketType::REQ)?;

        let cnt_socket = context.socket(zmq::SocketType::SUB)?;
        cnt_socket.set_subscribe(b"")?;
        cnt_socket.set_conflate(true)?;

        rpc_socket.connect(&rpc_address)?;
        cnt_socket.connect(&cnt_address)?;

        Ok(RemoteApiClient {
            context,
            id: Uuid::new_v4(),
            rpc_socket,
            cnt_socket,
        })
    }

    fn call(&self, payload: Vec<u8>) -> Result<Vec<u8>, Error> {
        log::debug!(
            "Raw request: {:?}",
            log_utils::bytes_to_hex_format(&payload)
        );

        self.rpc_socket.send(payload, zmq::DONTWAIT)?;

        let mut message = zmq::Message::new();

        self.rpc_socket.recv(&mut message, ZMQ_RECV_FLAG_NONE)?;

        let message = (*message).to_vec();

        log::debug!(
            "Raw response: {:?}",
            log_utils::bytes_to_hex_format(&message)
        );

        Ok(message)
    }

    pub fn send_request<R: RawRequest>(&self, request: R) -> Result<JsonValue, Error> {
        let payload = request.to_raw_request();

        let result = self.call(payload)?;

        let decoded: CborValue = ciborium::de::from_reader(result.as_slice()).unwrap();

        let json = cbor_to_json::cbor_to_json(decoded);
        log::trace!("Json response: {}", json);

        Ok(json)
    }

    pub fn get_object(&self, name: String) -> Result<JsonValue, Error> {
        let request = ZmqRequest::remote_api_info(name);

        self.send_request(request)
    }

    pub fn set_stepping(&self, enable: bool) -> Result<(), Error> {
        let request = ZmqRequest::set_stepping(enable, self.id.to_string());

        self.send_request(request)?;

        Ok(())
    }

    pub fn step(&self, wait: bool) -> Result<(), Error> {
        let request = ZmqRequest::step(self.id.to_string());

        match wait {
            true => {
                self.get_step_count(false)?;
                self.send_request(request)?;
                self.get_step_count(true)?;
            }
            false => {
                self.send_request(request)?;
            }
        }

        Ok(())
    }
    fn get_step_count(&self, wait: bool) -> Result<(), Error> {
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
