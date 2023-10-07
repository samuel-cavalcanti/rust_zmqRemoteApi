use serde_json::Value as JsonValue;

use crate::{RemoteAPIError, ZmqRequest, RawRequest};

pub trait RemoteApiClientInterface {
    fn send_raw_request(&self, request: Vec<u8>) -> Result<JsonValue, RemoteAPIError>;
    fn get_uuid(&self) -> String;

    fn send_request(
        &self,
        request: ZmqRequest,
    ) -> Result<serde_json::Value, crate::RemoteAPIError> {
        log::trace!("request: {request:?}");
        let mut json = self.send_raw_request(request.to_raw_request())?;

        if let Some(err) = json.get("err") {
            let error_msg = err.as_str().unwrap().to_string();
            return Err(
                crate::remote_api_objects::connection_error::RemoteAPIError::new(error_msg),
            );
        }

        while let Some(func) = json.get("func") {
            let function_name = func.as_str().unwrap().to_string();

            if function_name == "_*wait*_" {
                let wait_request = crate::zmq_requests::ZmqRequest::wait_request(self.get_uuid());

                json = self.send_raw_request(wait_request.to_raw_request())?
            }
        }

        Ok(json["ret"].to_owned())
    }
}
