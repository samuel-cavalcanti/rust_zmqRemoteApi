#[derive(Debug)]
/// RemoteAPIError will be retured if the client was unable to send our receive messages
pub struct RemoteAPIError {
    message: String,
}

impl RemoteAPIError {
    pub fn new(message: String) -> RemoteAPIError {
        RemoteAPIError { message }
    }

    pub fn show(&self) -> String {
        self.message.clone()
    }
}

impl From<zmq::Error> for RemoteAPIError {
    fn from(e: zmq::Error) -> Self {
        RemoteAPIError {
            message: e.to_string(),
        }
    }
}
