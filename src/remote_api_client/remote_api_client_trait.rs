use serde_json::Value as JsonValue;
use zmq::Error;

pub trait RemoteApiClientInterface {
    fn send_raw_request(&self, request: Vec<u8>) -> Result<JsonValue, Error>;
}
