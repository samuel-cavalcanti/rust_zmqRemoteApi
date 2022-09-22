
#[derive(Debug)]
pub struct RemoteAPIError {
    message: String,
}

impl RemoteAPIError {
    pub fn new(message: String) -> RemoteAPIError {
        RemoteAPIError {
            message,
        }
    }


    pub fn show(&self)->String{
        self.message.clone()
    }
}


impl From<zmq::Error> for RemoteAPIError {
    fn from(e: zmq::Error) -> Self {
        
        RemoteAPIError { message: e.to_string() }
    }
}
