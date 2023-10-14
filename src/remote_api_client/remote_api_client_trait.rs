use serde_json::Value as JsonValue;

use crate::{sim::Module, RawRequest, RemoteAPIError, ZmqRequest, ZmqResponse};

pub trait RemoteApiClientInterface {
    fn send_raw_request(&self, request: Vec<u8>) -> Result<JsonValue, RemoteAPIError>;
    fn get_uuid(&self) -> String;
    fn get_callback(&self, function_name: &str) -> Option<&Box<dyn Fn(JsonValue) -> JsonValue>>;

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
                let empty_req = ZmqRequest::executed_request(self.get_uuid(), vec![]);
                let req = if fun.function_name == "_*wait*_" {
                    empty_req
                } else {
                    let function = self.get_callback(&fun.function_name);
                    match function {
                        Some(function) => {
                            let output = function(fun.args);
                            let encoded = ciborium::cbor!(output).unwrap();
                            if let ciborium::Value::Array(vec) = encoded {
                                ZmqRequest::executed_request(self.get_uuid(), vec)
                            } else {
                                ZmqRequest::executed_request(self.get_uuid(), vec![encoded])
                            }
                        }
                        None => empty_req,
                    }
                };

                self.send_request(req)
            }
        }
    }
}
