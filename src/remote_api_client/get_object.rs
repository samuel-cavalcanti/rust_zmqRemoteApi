
use crate::remote_api_requests::{ RemoteApiRequest, RawRequest};

use ciborium::{cbor};
pub struct GetObject {
    pub name: String,
}

impl RemoteApiRequest for GetObject {
    fn function_name(&self) -> String {
        "zmqRemoteApi.info".to_string()
    }

    fn args(&self) -> Vec<ciborium::value::Value> {
        let value = cbor!(self.name).unwrap();
        vec![value]
    }
}

#[test]
fn test_get_sim() {
    let request = GetObject {
        name: "sim".to_string(),
    };
    let bytes = b"\xa2dfuncqzmqRemoteApi.infodargs\x81csim".to_vec();
    assert_eq!(bytes, request.to_raw_request());
}