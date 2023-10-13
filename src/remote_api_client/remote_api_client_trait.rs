use serde_json::Value as JsonValue;

use crate::{sim::Module, RawRequest, RemoteAPIError, ZmqRequest, ZmqResponse};

pub trait RemoteApiClientInterface {
    fn send_raw_request(&self, request: Vec<u8>) -> Result<JsonValue, RemoteAPIError>;
    fn get_uuid(&self) -> String;

    fn require(&self, module: Module) -> Result<(), RemoteAPIError> {
        let name = match module {
            Module::SimIK => "simIK".into(),
        };

        let require_req = ZmqRequest::require_request(name, self.get_uuid());
        self.send_request(require_req)?;

        Ok(())
    }

    fn send_request(&self, request: ZmqRequest) -> Result<JsonValue, crate::RemoteAPIError> {
        log::trace!("request: {request:?}");
        let json = self.send_raw_request(request.to_raw_request())?;

        log::trace!("json response: {json:?}");

        let response: ZmqResponse = serde_json::from_value(json).unwrap();


        match response {
            ZmqResponse::Error(e) => {
                log::error!("Error: {}", e.message);
                Err(RemoteAPIError::new(e.message))
            }
            ZmqResponse::Object(obj) => Ok(obj.ret),
            ZmqResponse::Function(fun) => {
                if fun.function_name == "_*wait*_" {
                    let wait_request = ZmqRequest::wait_request(self.get_uuid());
                    self.send_request(wait_request)
                } else {
                    Ok(serde_json::json!([]))
                }
            }
        }
    }
}
