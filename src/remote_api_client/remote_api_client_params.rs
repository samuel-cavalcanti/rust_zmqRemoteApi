/// The Socket connection Parameters, in most of the cases the default is Okay to use
/// If the simulation is running in another pc, make sure the ip is correct
pub struct RemoteApiClientParams {
    /// the Ip address, the default is "localhost"
    pub host: String,
    /// The socket port, the default is 23000
    pub rpc_port: usize,
}

impl Default for RemoteApiClientParams {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            rpc_port: 23000,
        }
    }
}
