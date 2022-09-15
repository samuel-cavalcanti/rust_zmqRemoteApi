use ciborium::cbor;
use ciborium::value::Value;

pub trait RawRequest {
    fn to_raw_request(&self) -> Vec<u8>;
}

#[derive(Debug)]
pub struct ZmqRequest {
    pub function_name: String,
    pub args: Vec<Value>,
}

impl ZmqRequest {
    pub fn remote_api_info(object: String) -> ZmqRequest {
        ZmqRequest {
            function_name: String::from("zmqRemoteApi.info"),
            args: vec![cbor!(object).unwrap()],
        }
    }

    pub fn step(uuid: String) -> ZmqRequest {
        ZmqRequest {
            function_name: "step".to_string(),
            args: vec![cbor!(uuid).unwrap()],
        }
    }

    pub fn set_stepping(enable: bool, uuid: String) -> ZmqRequest {
        ZmqRequest {
            function_name: "setStepping".to_string(),
            args: vec![cbor!(enable).unwrap(), cbor!(uuid).unwrap()],
        }
    }
}

impl RawRequest for ZmqRequest {
    fn to_raw_request(&self) -> Vec<u8> {
        let enconded = cbor!({"func"=> self.function_name, "args"=> self.args}).unwrap();

        log::trace!("sending: {:?}", enconded);

        let mut bytes = Vec::new();
        ciborium::ser::into_writer(&enconded, &mut bytes).unwrap();

        bytes
    }
}
