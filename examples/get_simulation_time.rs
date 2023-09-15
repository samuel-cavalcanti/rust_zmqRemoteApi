use std::rc::Rc;
use zmq_remote_api::{sim::Sim, RemoteAPIError, RemoteApiClientParams};

/*
    Example based on Example.cpp
*/

fn main() -> Result<(), RemoteAPIError> {
    // use the env variable RUST_LOG="trace" or RUST_LOG="debug" to observe the zmq communication
    env_logger::init();

    let sim = zmq_remote_api::RemoteApiClient::new(RemoteApiClientParams {
        host: "localhost".to_string(),
        ..RemoteApiClientParams::default()
    })?;


    sim.set_stepping(true)?;

    sim.start_simulation()?;

    let mut time = sim.get_simulation_time()?;

    while time < 3.0 {
        println!("Simulation time: {:.3} [s]", time);
        sim.step(true)?;
        time = sim.get_simulation_time()?;
    }

    sim.stop_simulation()?;

    Ok(())
}
