use zmq_remote_api::{RemoteAPIObjects, RemoteApiClientParams};

/*
    Example based on Example.cpp
*/

fn main() -> Result<(), zmq::Error> {
    // use the env variable RUST_LOG="trace" or RUST_LOG="debug" to observe the zmq communication
    env_logger::init();

    let client = zmq_remote_api::RemoteApiClient::new(RemoteApiClientParams {
        host: "localhost".to_string(),
        ..RemoteApiClientParams::default()
    })?;

    let sim = RemoteAPIObjects::new(&client);

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
