use crate::remote_api_client::RemoteApiClientInterface;
use std::rc::Rc;

use crate::zmq_requests::RawRequest;
use serde_json::Value;

pub struct SimIk {
    client: Rc<dyn RemoteApiClientInterface>,
}

impl SimIk {
    pub fn new(client: Rc<dyn RemoteApiClientInterface>) -> SimIk {
        SimIk { client }
    }

    fn is_success(json: &Value) -> Result<(), String> {
        if let Some(Value::Bool(success)) = json.get("success") {
            if *success {
                Ok(())
            } else {
                Err(Self::get_error(json))
            }
        } else {
            Err(Self::get_error(json))
        }
    }

    fn get_error(json: &Value) -> String {
        if let Some(Value::String(error)) = json.get("error") {
            error.clone()
        } else {
            "unknown error".to_string()
        }
    }
}

include!("../../../c_transpiler/assets/sim_ik.rs");
