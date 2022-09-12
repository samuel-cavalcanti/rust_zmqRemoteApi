pub struct RemoteApiClientParams {
    pub host: String,
    pub rpc_port: usize,
    pub cnt_port: usize,
}

impl Default for RemoteApiClientParams {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            rpc_port: 23000,
            cnt_port: 23001,
        }
    }
}
