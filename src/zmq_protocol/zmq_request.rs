use ciborium::{cbor, value::Value};
use serde::Serialize;

pub trait RawRequest {
    fn to_raw_request(&self) -> Vec<u8>;
}

pub const VERSION: i32 = 2;
pub const LANG: &str = "rust";

#[derive(Debug, Serialize)]
pub struct ZmqRequest {
    pub func: String,
    pub args: Vec<Value>,
    pub uuid: String,
    pub ver: i32,
    pub lang: String,

    #[serde(rename = "argsL")]
    pub args_l: usize,
}

impl ZmqRequest {
    pub fn remote_api_info(object: String, uuid: String) -> ZmqRequest {
        ZmqRequest {
            uuid,
            func: "zmqRemoteApi.info".into(),
            args: vec![cbor!(object).unwrap()],
            ver: VERSION,
            lang: LANG.into(),
            args_l: 1,
        }
    }
    pub fn executed_request(uuid: String, args: Vec<Value>) -> ZmqRequest {
        ZmqRequest {
            uuid,
            func: "_*executed*_".into(),
            args_l: args.len(),
            args,
            ver: VERSION,
            lang: LANG.into(),
        }
    }

    pub fn end_request(uuid: String) -> ZmqRequest {
        ZmqRequest {
            uuid,
            func: "_*end*_".into(),
            args: vec![],
            ver: VERSION,
            lang: LANG.into(),
            args_l: 0,
        }
    }

    pub fn require_request(name: String, uuid: String) -> ZmqRequest {
        ZmqRequest {
            uuid,
            func: "zmqRemoteApi.require".into(),
            args: vec![cbor!(name).unwrap()],
            ver: VERSION,
            lang: LANG.into(),
            args_l: 1,
        }
    }
}

impl RawRequest for ZmqRequest {
    fn to_raw_request(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        ciborium::ser::into_writer(self, &mut bytes).unwrap();

        bytes
    }
}
