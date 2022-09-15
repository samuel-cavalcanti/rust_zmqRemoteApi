use zmq_remote_api::{remote_api_objects_const, RemoteAPIObjects, RemoteApiClientParams};

/*
    Example based on SimpleTest.py
*/

fn main() -> Result<(), zmq::Error> {
    println!("Program started");

    let client = zmq_remote_api::RemoteApiClient::new(RemoteApiClientParams {
        host: "localhost".to_string(),
        ..RemoteApiClientParams::default()
    })?;

    let sim = RemoteAPIObjects::new(&client);

    let _default_idle_fps = sim.get_int32_param(remote_api_objects_const::intparam_idle_fps)?;

    Ok(())
}
