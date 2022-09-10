use crate::remote_api_requests::{RawRequest, RemoteApiRequest};

use ciborium::{cbor, value::Value};
pub struct SetStepping {
    pub enable: bool,
    pub uuid: String,
}

impl RemoteApiRequest for SetStepping {
    fn function_name(&self) -> String {
        "setStepping".to_string()
    }

    fn args(&self) -> Vec<ciborium::value::Value> {
        let enable = cbor!(self.enable).unwrap();
        let uuid = cbor!(self.uuid).unwrap();
        vec![enable, uuid]
    }
}

#[test]
fn test_set_stepping() {
    let request = SetStepping {
        enable: true,
        uuid: String::from("a9bb7126-0d1f-4474-8801-078c094dcee9"),
    };
    let bytes =
        b"\xa2dfuncksetSteppingdargs\x82\xf5x$a9bb7126-0d1f-4474-8801-078c094dcee9".to_vec();

    assert_eq!(bytes, request.to_raw_request());
}
