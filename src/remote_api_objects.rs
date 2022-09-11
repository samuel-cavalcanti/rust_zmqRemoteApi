use crate::{zmq_requests, RemoteApiClient};
use serde_json::Value;
pub struct RemoteAPIObjects<'a> {
    client: &'a RemoteApiClient,
}

impl<'a> RemoteAPIObjects<'a> {
    pub fn new(client: &RemoteApiClient) -> RemoteAPIObjects {
        RemoteAPIObjects { client }
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

    pub fn start_simulation(&self) -> Result<i64, zmq::Error> {
        let request = zmq_requests::ZmqRequest::start_simulation();

        let result = self.client.send_request(request)?;

        if let Err(error) = Self::is_success(&result) {
            panic!("error: {}", error)
        }

        Ok(result["ret"].as_array().unwrap()[0].as_i64().unwrap())
    }

    pub fn stop_simulation(&self) -> Result<i64, zmq::Error> {
        let request = zmq_requests::ZmqRequest::stop_simulation();

        let result = self.client.send_request(request)?;

        if let Err(error) = Self::is_success(&result) {
            panic!("error: {}", error)
        }

        Ok(result["ret"].as_array().unwrap()[0].as_i64().unwrap())
    }

    pub fn get_simulation_time(&self) -> Result<f64, zmq::Error> {
        let request = zmq_requests::ZmqRequest::get_simulation_time();

        let result = self.client.send_request(request)?;

        if let Err(error) = Self::is_success(&result) {
            panic!("error: {}", error)
        }

        Ok(result["ret"].as_array().unwrap()[0].as_f64().unwrap())
    }
}
