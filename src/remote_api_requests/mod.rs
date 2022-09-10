use ciborium::cbor;
use ciborium::value::Value;
pub trait RawRequest {
    fn to_raw_request(&self) -> Vec<u8>;
}

pub trait RemoteApiRequest {
    fn function_name(&self) -> String;
    fn args(&self) -> Vec<Value>;
}

impl<T> RawRequest for T
where
    T: RemoteApiRequest,
{
    fn to_raw_request(&self) -> Vec<u8> {
        let enconded = cbor!({"func"=> self.function_name(), "args"=> self.args()}).unwrap();

        log::trace!("sending: {:?}", enconded);

        let mut bytes = Vec::new();
        ciborium::ser::into_writer(&enconded, &mut bytes).unwrap();

        bytes
    }
}
