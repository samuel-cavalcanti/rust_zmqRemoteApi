
use crate::remote_api_requests::{ RemoteApiRequest, RawRequest};

use ciborium::{cbor};
pub struct GetStep {
    pub uuid: String,
}

impl RemoteApiRequest for GetStep {
    fn function_name(&self) -> String {
        "step".to_string()
    }

    fn args(&self) -> Vec<ciborium::value::Value> {
        let value = cbor!(self.uuid).unwrap();
        vec![value]
    }
}

#[test]
fn test_get_sim() {
    let request = GetStep {
        uuid: "2b2d55e0-576c-4dce-86c5-c3b7a3df0d73".to_string(),
    };
    let bytes = b"\xa2dfuncdstepdargs\x81x$2b2d55e0-576c-4dce-86c5-c3b7a3df0d73".to_vec();
    assert_eq!(bytes, request.to_raw_request());
}