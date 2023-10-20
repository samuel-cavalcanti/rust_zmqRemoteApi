use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
/// Coppeleia's server can return 3 kinds of reponse:
/// An error: ErrorResponse
/// The Function Response: FunctionResponse
/// An Object: ObjectResponse
/// More information see the
/// [PROTOCOL](https://github.com/CoppeliaRobotics/zmqRemoteApi/blob/master/PROTOCOL.md)
pub enum ZmqResponse {
    Error(ErrorResponse),
    Object(ObjectResponse),
    Function(FunctionResponse),
}
#[derive(Deserialize, Debug)]
/// When an Error occurs in the simulation, the sever will send an ErrorResponse
pub struct ErrorResponse {
    #[serde(rename = "err")]
    pub message: String,
}
#[derive(Deserialize, Debug)]
/// When the server sends data to client, like a object handle id, or the value of any sensors
/// the  server will send ObjectResponse
pub struct ObjectResponse {
    pub ret: Value,
}

#[derive(Deserialize, Debug)]
/// When an Callback is passed or the server wants to the client wait, then a FunctionResponse
/// is Returned
pub struct FunctionResponse {
    #[serde(rename = "func")]
    pub function_name: String,
    pub args: Value,
}
