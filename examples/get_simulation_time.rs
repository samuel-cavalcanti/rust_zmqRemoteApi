use coppeliasim_zmq_remote_api::{
    sim::Sim, RemoteAPIError, RemoteApiClient, RemoteApiClientParams,
};

/*
    Example based on Example.cpp
*/

fn main() -> Result<(), RemoteAPIError> {
    // use the env variable RUST_LOG="trace" or RUST_LOG="debug" to observe the zmq communication
    env_logger::init();

    let client = RemoteApiClient::new(RemoteApiClientParams {
        host: "localhost".to_string(),
        ..RemoteApiClientParams::default()
    })?;

    client.sim_set_stepping(true)?;

    client.sim_start_simulation()?;

    let mut time = client.sim_get_simulation_time()?;

    while time < 3.0 {
        println!("Simulation time: {:.3} [s]", time);
        client.sim_step()?;
        time = client.sim_get_simulation_time()?;
    }

    client.sim_stop_simulation()?;

    Ok(())
}
