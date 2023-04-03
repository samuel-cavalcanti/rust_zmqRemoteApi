use std::sync::Arc;
use zmq_remote_api::{sim::Sim, RemoteAPIError, RemoteApiClientParams};

/*
    Example based on Example.cpp
*/

fn main() -> Result<(), RemoteAPIError> {
    // use the env variable RUST_LOG="trace" or RUST_LOG="debug" to observe the zmq communication
    env_logger::init();

    let client = zmq_remote_api::RemoteApiClient::new(RemoteApiClientParams {
        host: "localhost".to_string(),
        ..RemoteApiClientParams::default()
    })?;

    // Arc means Atomic reference counter, is a smart pointer that counter the number of references
    let client = Arc::new(client);
    let sim = Sim::new(client.clone());

    client.set_stepping(true)?;

    sim.start_simulation()?;

    let mut time = sim.get_simulation_time()?;

    while time < 3.0 {
        println!("Simulation time: {:.3} [s]", time);
        client.step(true)?;
        time = sim.get_simulation_time()?;
    }

    sim.stop_simulation()?;

    Ok(())
}
