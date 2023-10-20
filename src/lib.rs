//! # ZMQ Remote API for CoppeliaSim, Rust implementation
//! The ZMQ Remote API requires the [ZMQ plugin](https://github.com/CoppeliaRobotics/simExtZMQ).
//!
//! the Current Plugins support is
//! - [SimIK](https://www.coppeliarobotics.com/helpFiles/en/simIK.htm)
//! - [The Remote API](https://www.coppeliarobotics.com/helpFiles/index.html)
//!
//! Make sure that you are using the same version of the Client and CoppeliaSim, see the branchs of the repository
//! [rust_zmqRemoteApi](https://github.com/samuel-cavalcanti/rust_zmqRemoteApi)
//!
//! ## Get started
//!
//! ```no_run
//! use coppeliasim_zmq_remote_api::{sim, sim::Sim, RemoteAPIError, RemoteApiClientParams,RemoteApiClient};
//!
//! fn main() -> Result<(), RemoteAPIError> {
//!  let  client = RemoteApiClient::new(RemoteApiClientParams::default())?;
//!
//!   Ok(())
//! }
//!
//! ```
//! Or See an example in
//! [examples](https://github.com/samuel-cavalcanti/rust_zmqRemoteApi/tree/main/examples)
//!
mod log_utils;
mod remote_api_client;
mod remote_api_objects;
mod zmq_protocol;

pub use remote_api_client::{RemoteApiClient, RemoteApiClientInterface, RemoteApiClientParams};
pub use remote_api_objects::sim;
pub use zmq_protocol::{ErrorResponse, FunctionResponse, ObjectResponse, ZmqResponse};
pub use zmq_protocol::{RawRequest, ZmqRequest, LANG, VERSION};

pub use remote_api_objects::connection_error::RemoteAPIError;
/// You will need to use Json to communicate with the the ZMQ server
pub use serde_json;
