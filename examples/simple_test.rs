use std::rc::Rc;
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

    // Rc means Reference counter, is a smart pointer that counter the number of references
    let client = Rc::new(client);
    let sim = Sim::new(client.clone());

    // When simulation is not running, ZMQ message handling could be a bit
    // slow, since the idle loop runs at 8 Hz by default. So let's make
    // sure that the idle loop runs at full speed for this program:

    let default_idle_fps = sim.get_int32_param(sim::INTPARAM_IDLE_FPS)?;
    sim.set_int32_param(sim::INTPARAM_IDLE_FPS, 0)?;

    // Create a few dummies and set their positions:
    let handles: Vec<i64> = (0..50).map(|_| sim.create_dummy(0.01).unwrap()).collect();

    for (i, h) in handles.iter().enumerate() {
        let i = i as f64;
        sim.set_object_position(*h, sim::HANDLE_WORLD, vec![0.01 * i, 0.01 * i, 0.01 * i])?;
    }

    sim.start_simulation()?;
    // Run a simulation in asynchronous mode:
    let mut time = sim.get_simulation_time()?;
    while time < 3.0 {
        time = sim.get_simulation_time()?;

        println!("Simulation time: {time:.2} [s] (simulation running asynchronously  to client, i.e. non-stepped)", time = time);
    }

    sim.stop_simulation()?;
    // if you need to make sure we really stopped:
    while sim.get_simulation_state()? != sim::SIMULATION_STOPPED {
        std::thread::sleep(std::time::Duration::from_secs_f64(0.1))
    }

    client.set_stepping(true)?;
    sim.start_simulation()?;

    // Run a simulation in stepping mode:
    let mut time = sim.get_simulation_time()?;
    while time < 3.0 {
        time = sim.get_simulation_time()?;

        let message = format!("Simulation time: {time:.2} [s] (simulation running asynchronously  to client, i.e. stepped)", time = time);
        println!("{}", message);
        sim.add_log(sim::VERBOSITY_SCRIPTINFOS, message)?;
        client.step(true)?; //triggers next simulation step
    }
    sim.stop_simulation()?;

    //Remove the dummies created earlier:
    for h in handles {
        sim.remove_object(h)?;
    }

    //Restore the original idle loop frequency:
    sim.set_int32_param(sim::INTPARAM_IDLE_FPS, default_idle_fps)?;
    println!("Program ended");

    Ok(())
}
