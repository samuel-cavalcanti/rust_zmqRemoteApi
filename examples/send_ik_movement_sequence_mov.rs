use zmq_remote_api::serde_json::json;
use zmq_remote_api::{sim, sim::Sim, RemoteAPIError, RemoteApiClient, RemoteApiClientParams};
/*
    Example based on sendIkMovementSequence-mov.py

     Make sure to have CoppeliaSim running, with followig scene loaded:

    scenes/messaging/ikMovementViaRemoteApi.ttt

    Do not launch simulation, then run this script
*/

fn wait_for_movement_executed<S: Sim>(
    id: String,
    sim: &S,
    signal_name: String,
) -> Result<(), RemoteAPIError> {
    let mut string = sim.get_string_signal(signal_name.clone())?;
    while string != id {
        string = sim.get_string_signal(signal_name.clone())?;
    }

    Ok(())
}

fn main() -> Result<(), RemoteAPIError> {
    // use the env variable RUST_LOG="trace" or RUST_LOG="debug" to observe the zmq communication
    env_logger::init();

    let client = RemoteApiClient::new(RemoteApiClientParams {
        host: "localhost".to_string(),
        ..RemoteApiClientParams::default()
    })?;

    println!("Program started");

    let target_arm = "/LBR4p".to_string();

    let signal_name = format!("{}_executedMovId", target_arm);

    let arm_handle = client.get_object(target_arm, None)?;
    let script_handle = client.get_script(sim::SCRIPTTYPE_CHILDSCRIPT, Some(arm_handle), None)?;

    client.start_simulation()?;

    println!("Wait until ready");
    wait_for_movement_executed("ready".to_string(), &client, signal_name.clone())?;

    println!("Get initial pose");
    let json = client.call_script_function(
        String::from("remoteApi_getPoseAndConfig"),
        script_handle,
        None,
    )?;

    let (initial_pose, _initial_config): (Vec<f64>, Vec<f64>) =
        serde_json::from_value(json).unwrap();

    println!("Send first movement sequence");
    const MAX_VEL: f64 = 0.1;
    const MAX_ACCEL: f64 = 0.01;

    let movement_data = json!({"id": "movSeq1", "type": "mov", "targetPose": [0, 0, 0.85, 0, 0, 0, 1], "maxVel": MAX_VEL, "maxAccel":MAX_ACCEL});

    let _json = client.call_script_function(
        String::from("remoteApi_movementDataFunction"),
        script_handle,
        Some(movement_data),
    )?;

    println!("Execute first movement sequence");
    let _json = client.call_script_function(
        String::from("remoteApi_executeMovement"),
        script_handle,
        Some(json!("movSeq1")),
    )?;

    println!("Wait until above movement sequence finished executing");
    wait_for_movement_executed("movSeq1".to_string(), &client, signal_name.clone())?;

    println!("Send second and third movement sequence, where third one should execute immediately after the second one");
    let target_pose = vec![
        0.0,
        0.0,
        0.85,
        -0.7071068883,
        -6.252754758e-08,
        -8.940695295e-08,
        -0.7071067691,
    ];

    let movement_data = json!({"id": "movSeq2", "type": "mov", "targetPose": target_pose, "maxVel": MAX_VEL, "maxAccel": MAX_ACCEL});

    let _json = client.call_script_function(
        String::from("remoteApi_movementDataFunction"),
        script_handle,
        Some(movement_data),
    )?;

    let movement_data = json!({"id": "movSeq3", "type": "mov", "targetPose": initial_pose, "maxVel": MAX_VEL, "maxAccel": MAX_ACCEL});

    let _json = client.call_script_function(
        String::from("remoteApi_movementDataFunction"),
        script_handle,
        Some(movement_data),
    )?;

    println!("Execute second and third movement sequence");

    client.call_script_function(
        String::from("remoteApi_executeMovement"),
        script_handle,
        Some(json!("movSeq2")),
    )?;
    client.call_script_function(
        String::from("remoteApi_executeMovement"),
        script_handle,
        Some(json!("movSeq3")),
    )?;

    println!("Wait until above 2 movement sequences finished executing");

    wait_for_movement_executed("movSeq3".to_string(), &client, signal_name.clone())?;

    client.stop_simulation()?;

    println!("Program ended");
    Ok(())
}
