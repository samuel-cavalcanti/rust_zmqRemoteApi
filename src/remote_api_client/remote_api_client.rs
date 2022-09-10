use ciborium::value::Value as CborValue;
use serde_json::Value as JsonValue;
use uuid::Uuid;
use zmq::Error;

use super::{cbor_to_json, get_object::GetObject, set_stepping::SetStepping};
use crate::{
    log_utils, remote_api_client::get_step::GetStep, remote_api_requests::RawRequest,
    RemoteApiClientParams,
};

const ZMQ_RECV_FLAG_NONE: i32 = 0;

pub struct RemoteApiClient {
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

        let context = zmq::Context::new();

        let rpc_socket = context.socket(zmq::SocketType::REQ)?;
        let cnt_socket = context.socket(zmq::SocketType::SUB)?;

        rpc_socket.connect(&rpc_address)?;

        cnt_socket.set_subscribe(b"")?;
        cnt_socket.set_conflate(true)?;
        cnt_socket.connect(&cnt_address)?;

        Ok(RemoteApiClient {
            id: Uuid::new_v4(),
            rpc_socket: rpc_socket,
            cnt_socket,
        })
    }

    pub fn call(&self, payload: Vec<u8>) -> Result<Vec<u8>, Error> {
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
        log::trace!("receive: {}", json);

        Ok(json)
    }

    pub fn get_object(&self, name: String) -> Result<JsonValue, Error> {
        let request = GetObject { name };

        self.send_request(request)
    }

    pub fn set_stepping(&self, enable: bool) -> Result<JsonValue, Error> {
        let request = SetStepping {
            enable,
            uuid: self.id.to_string(),
        };

        self.send_request(request)
    }

    pub fn step(&self, wait: bool) -> Result<(), Error> {
        if wait {
            self.get_step_count(false)?;
            let request = GetStep {
                uuid: self.id.to_string(),
            };
            self.send_request(request)?;
            self.get_step_count(true)?;
        }

        Ok(())
    }
    fn get_step_count(&self, wait: bool) -> Result<(), Error> {
        let flag = match wait {
            true => ZMQ_RECV_FLAG_NONE,
            false => zmq::DONTWAIT,
        };

        let mut message = zmq::Message::new();
        self.cnt_socket.recv(&mut message, flag)?;

        let message = (*message).to_vec();

        Self::log_step_count(message);

        Ok(())
    }

    fn log_step_count(message: Vec<u8>) {
        const size: usize = 4;
        if message.len() == size {
            let mut slice: [u8; 4] = [0, 0, 0, 0];
            slice.copy_from_slice(&message);
            let counter = i32::from_be_bytes(slice);
            log::debug!("Step count: {}", counter);
        } else {
            log::warn!(
                "Step count message have more than 4 bytes, msg: {:?}",
                message
            );
        }
    }
}
