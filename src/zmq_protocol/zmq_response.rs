use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum ZmqResponse {
    Error(ErrorResponse),
    Object(ObjectResponse),
    Function(FunctionResponse),
}
//#[derive(Deserialize, Debug)]
//pub struct ArrayResponse {
//    #[serde(rename = "ret")]
//    pub array: Vec<Value>,
//}
#[derive(Deserialize, Debug)]
pub struct ErrorResponse {
    #[serde(rename = "err")]
    pub message: String,
}
#[derive(Deserialize, Debug)]
pub struct ObjectResponse {
    pub ret: Value,
}

#[derive(Deserialize, Debug)]
pub struct FunctionResponse {
    #[serde(rename = "func")]
    pub function_name: String,
    pub args: Value,
}
