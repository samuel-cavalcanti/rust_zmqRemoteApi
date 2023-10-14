use zmq_remote_api::{sim, sim::Sim, RemoteAPIError, RemoteApiClientParams};
/*
    Example based on SimpleTest.py

# Make sure to have the add-on "ZMQ remote API"
# running in CoppeliaSim
#
# All CoppeliaSim commands will run in blocking mode (block
# until a reply from CoppeliaSim is received).

*/

fn callback(_json: serde_json::Value) -> serde_json::Value {
    println!("from fn callback !!");
    serde_json::json!({})
}

fn main() -> Result<(), RemoteAPIError> {
    // use the env variable RUST_LOG="trace" or RUST_LOG="debug" to observe the zmq communication
    env_logger::init();

    println!("Program started");

    let mut client = zmq_remote_api::RemoteApiClient::new(RemoteApiClientParams {
        host: "localhost".to_string(),
        ..RemoteApiClientParams::default()
    })?;
    let fn_cb = "fn CB";
    client.register_callback(fn_cb.into(), Box::new(callback));

    // When simulation is not running, ZMQ message handling could be a bit
    // slow, since the idle loop runs at 8 Hz by default. So let's make
    // sure that the idle loop runs at full speed for this program:

    let default_idle_fps = client.sim_get_int32_param(sim::INTPARAM_IDLE_FPS)?;
    client.sim_set_int32_param(sim::INTPARAM_IDLE_FPS, 0)?;

    // Create a few dummies and set their positions:
    let handles: Vec<i64> = (0..50)
        .map(|_| client.sim_create_dummy(0.01).unwrap())
        .collect();
    let closure_callback = move |_json: serde_json::Value| -> serde_json::Value {
        println!("from fn closure callback !!");
        serde_json::json!({})
    };

    let closure_cb = "closure CB";
    client.register_callback(closure_cb.into(), Box::new(closure_callback));

    for (i, h) in handles.iter().enumerate() {
        let i = i as f64;
        let pos = vec![0.01 * i, 0.01 * i, 0.01 * i];
        client.sim_set_object_position(*h, pos, Some(sim::HANDLE_WORLD))?;
    }

    client.sim_start_simulation()?;
    // Run a simulation in asynchronous mode:
    let mut time = client.sim_get_simulation_time()?;
    while time < 3.0 {
        time = client.sim_get_simulation_time()?;

        println!("Simulation time: {time:.2} [s] (simulation running asynchronously  to client, i.e. non-stepped)", time = time);
    }

    client.sim_test_cb(10, format!("{fn_cb}@func"), 20)?;
    client.sim_test_cb(10, format!("{closure_cb}@func"), 20)?;

    client.sim_stop_simulation()?;
    // if you need to make sure we really stopped:
    while client.sim_get_simulation_state()? != sim::SIMULATION_STOPPED {
        std::thread::sleep(std::time::Duration::from_secs_f64(0.1))
    }

    client.sim_start_simulation()?;

    // Run a simulation in stepping mode:
    let mut time = client.sim_get_simulation_time()?;
    while time < 3.0 {
        time = client.sim_get_simulation_time()?;

        let message = format!("Simulation time: {time:.2} [s] (simulation running asynchronously  to client, i.e. stepped)", time = time);
        println!("{}", message);
        client.sim_add_log(sim::VERBOSITY_SCRIPTINFOS, message)?;
    }
    client.sim_stop_simulation()?;

    //Remove the dummies created earlier:
    client.sim_remove_objects(handles)?;

    //Restore the original idle loop frequency:
    client.sim_set_int32_param(sim::INTPARAM_IDLE_FPS, default_idle_fps)?;
    println!("Program ended");

    Ok(())
}
