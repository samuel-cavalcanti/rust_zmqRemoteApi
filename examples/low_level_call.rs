use coppeliasim_zmq_remote_api::{
    RawRequest, RemoteAPIError, RemoteApiClient, RemoteApiClientInterface, RemoteApiClientParams,
    ZmqRequest,
};

use ciborium::cbor;

fn main() -> Result<(), RemoteAPIError> {
    // use the env variable RUST_LOG="trace" or RUST_LOG="debug" to observe the zmq communication
    env_logger::init();

    let client = RemoteApiClient::new(RemoteApiClientParams {
        host: "localhost".to_string(),
        ..RemoteApiClientParams::default()
    })?;

    let request = ZmqRequest {
        uuid: client.get_uuid(),
        func: "sim.getStringSignal".to_string(),
        args: vec![cbor!("testSignal").unwrap()],
        ver: coppeliasim_zmq_remote_api::VERSION,
        lang: coppeliasim_zmq_remote_api::LANG.into(),
        args_l: 1,
    };

    let json = client.send_raw_request(request.to_raw_request())?;

    println!("json: {json}");

    Ok(())
}
