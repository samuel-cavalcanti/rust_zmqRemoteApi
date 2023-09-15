use zmq_remote_api::{sim, sim::Sim, RemoteAPIError, RemoteApiClientParams};

/*
    Example based on SimpleTest.py

# Make sure to have the add-on "ZMQ remote API"
# running in CoppeliaSim
#
# All CoppeliaSim commands will run in blocking mode (block
# until a reply from CoppeliaSim is received).

*/

fn main() -> Result<(), RemoteAPIError> {
    // use the env variable RUST_LOG="trace" or RUST_LOG="debug" to observe the zmq communication
    env_logger::init();

    println!("Program started");

    let client = zmq_remote_api::RemoteApiClient::new(RemoteApiClientParams {
        host: "localhost".to_string(),
        ..RemoteApiClientParams::default()
    })?;


    // When simulation is not running, ZMQ message handling could be a bit
    // slow, since the idle loop runs at 8 Hz by default. So let's make
    // sure that the idle loop runs at full speed for this program:

    let default_idle_fps = client.get_int32_param(sim::INTPARAM_IDLE_FPS)?;
    client.set_int32_param(sim::INTPARAM_IDLE_FPS, 0)?;

    // Create a few dummies and set their positions:
    let handles: Vec<i64> = (0..50).map(|_| client.create_dummy(0.01).unwrap()).collect();

    for (i, h) in handles.iter().enumerate() {
        let i = i as f64;
        client.set_object_position(*h, sim::HANDLE_WORLD, vec![0.01 * i, 0.01 * i, 0.01 * i])?;
    }

    client.start_simulation()?;
    // Run a simulation in asynchronous mode:
    let mut time = client.get_simulation_time()?;
    while time < 3.0 {
        time = client.get_simulation_time()?;

        println!("Simulation time: {time:.2} [s] (simulation running asynchronously  to client, i.e. non-stepped)", time = time);
    }

    client.stop_simulation()?;
    // if you need to make sure we really stopped:
    while client.get_simulation_state()? != sim::SIMULATION_STOPPED {
        std::thread::sleep(std::time::Duration::from_secs_f64(0.1))
    }

    client.set_stepping(true)?;
    client.start_simulation()?;

    // Run a simulation in stepping mode:
    let mut time = client.get_simulation_time()?;
    while time < 3.0 {
        time = client.get_simulation_time()?;

        let message = format!("Simulation time: {time:.2} [s] (simulation running asynchronously  to client, i.e. stepped)", time = time);
        println!("{}", message);
        client.add_log(sim::VERBOSITY_SCRIPTINFOS, message)?;
        client.step(true)?; //triggers next simulation step
    }
    client.stop_simulation()?;

    //Remove the dummies created earlier:
    for h in handles {
        client.remove_object(h)?;
    }

    //Restore the original idle loop frequency:
    client.set_int32_param(sim::INTPARAM_IDLE_FPS, default_idle_fps)?;
    println!("Program ended");

    Ok(())
}
