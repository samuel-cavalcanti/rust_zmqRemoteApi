use std::f64::consts::PI;

use zmq_remote_api::{sim::Sim, RemoteAPIError, RemoteApiClient, RemoteApiClientParams};

/*

based on pController.py

# Make sure to have CoppeliaSim running, with following scene loaded:
#
# scenes/messaging/pControllerViaRemoteApi.ttt
#
# Do not launch simulation, but run this script

*/

fn main() -> Result<(), RemoteAPIError> {
    // use the env variable RUST_LOG="trace" or RUST_LOG="debug" to observe the zmq communication
    env_logger::init();

    println!("Program started");

    let client = RemoteApiClient::new(RemoteApiClientParams {
        host: "localhost".to_string(),
        ..RemoteApiClientParams::default()
    })?;

    let joint_handle = client.sim_get_object("/Cuboid[0]/joint".to_string(), None)?;
    let mut join_angle = client.sim_get_joint_position(joint_handle)?;
    client.sim_set_joint_target_velocity(joint_handle, 360.0 * PI, None)?;

    /*
       enable the stepping mode on the client, that means
       the simulation waits the trigger: client.step()

       triggers next simulation step
    */
    client.sim_set_stepping(true)?;

    client.sim_start_simulation()?;

    move_to_angle(
        45.0 * PI / 180.0,
        &mut join_angle,
        &client,
        &client,
        &joint_handle,
    )?;

    move_to_angle(
        90.0 * PI / 180.0,
        &mut join_angle,
        &client,
        &client,
        &joint_handle,
    )?;

    move_to_angle(
        -89.0 * PI / 180.0,
        &mut join_angle,
        &client,
        &client,
        &joint_handle,
    )?;

    move_to_angle(
        0.0 * PI / 180.0,
        &mut join_angle,
        &client,
        &client,
        &joint_handle,
    )?;

    client.sim_stop_simulation()?;

    println!("Program ended");
    Ok(())
}

const MAX_FORCE: f64 = 100.0;

fn move_to_angle<S: Sim>(
    target_angle: f64,
    join_angle: &mut f64,
    sim: &S,
    client: &RemoteApiClient,
    joint_handle: &i64,
) -> Result<(), RemoteAPIError> {
    while (target_angle - *join_angle).abs() > 0.1 * PI / 180.0 {
        let velocity = compute_target_velocity(target_angle, *join_angle);
        sim.sim_set_joint_target_velocity(*joint_handle, velocity, None)?;
        sim.sim_set_joint_target_force(*joint_handle, MAX_FORCE, None)?;
        client.sim_step()?;
        *join_angle = sim.sim_get_joint_position(*joint_handle)?;
    }

    Ok(())
}

const PID_P: f64 = 0.1;
const DYN_STEP_SIZE: f64 = 0.005;
const VEL_UPPER_LIMIT: f64 = 360.0 * PI / 180.0;

fn compute_target_velocity(target_angle: f64, join_angle: f64) -> f64 {
    let error_value = target_angle - join_angle;
    let sin = error_value.sin();
    let cos = error_value.cos();
    let error_value = sin.atan2(cos);

    let ctrl = error_value * PID_P;

    /*
         Calculate the velocity needed to reach the position
         in one dynamic time step:
    */

    let velocity = ctrl / DYN_STEP_SIZE;

    if velocity > VEL_UPPER_LIMIT {
        return VEL_UPPER_LIMIT;
    }

    if velocity < -VEL_UPPER_LIMIT {
        return -VEL_UPPER_LIMIT;
    }

    velocity
}
