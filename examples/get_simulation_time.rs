use zmqRemoteApi::{RemoteAPIObjects, RemoteApiClientParams};

/*
    Example based on Example.cpp
*/

fn main() -> Result<(), zmq::Error> {
    env_logger::init();

    let client = zmqRemoteApi::RemoteApiClient::new(RemoteApiClientParams {
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
