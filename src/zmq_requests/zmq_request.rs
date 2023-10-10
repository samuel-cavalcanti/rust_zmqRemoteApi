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
}

impl ZmqRequest {
    pub fn remote_api_info(object: String, uuid: String) -> ZmqRequest {
        ZmqRequest {
            uuid,
            func: "zmqRemoteApi.info".into(),
            args: vec![cbor!(object).unwrap()],
            ver: VERSION,
            lang: LANG.into(),
        }
    }
    pub fn wait_request(uuid: String) -> ZmqRequest {
        ZmqRequest {
            uuid,
            func: "_*executed*_".into(),
            args: vec![],
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
