use zmq_remote_api::{
    RawRequest, RemoteAPIError, RemoteApiClientInterface, RemoteApiClientParams, ZmqRequest,
};

use ciborium::cbor;

fn main() -> Result<(), RemoteAPIError> {
    // use the env variable RUST_LOG="trace" or RUST_LOG="debug" to observe the zmq communication
    env_logger::init();

    let client = zmq_remote_api::RemoteApiClient::new(RemoteApiClientParams {
        host: "localhost".to_string(),
        ..RemoteApiClientParams::default()
    })?;

    let request = ZmqRequest {
        function_name: "sim.getStringSignal".to_string(),
        args: vec![cbor!("testSignal").unwrap()],
    };

    let json = client.send_raw_request(request.to_raw_request())?;

    println!("json: {json}");

    Ok(())
}
