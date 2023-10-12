use serde_json::json;
use zmq_remote_api::{
    sim,
    sim::{Sim, SimIK},
    RemoteAPIError, RemoteApiClientInterface, RemoteApiClientParams,
};

/*
#
# Example based in scenes/tutorials/InverseKinematics/redundantRobot.ttt
# Make sure to have CoppeliaSim running, with following scene loaded:
#
# <rust_zmq_remote_api>scenes/redundantRobot.ttt
#
# Do not launch simulation, but run this script
*/

fn main() -> Result<(), RemoteAPIError> {
    // use the env variable RUST_LOG="trace" or RUST_LOG="debug" to observe the zmq communication
    env_logger::init();

    println!("Program started");

    let client = zmq_remote_api::RemoteApiClient::new(RemoteApiClientParams {
        host: "localhost".to_string(),
        ..RemoteApiClientParams::default()
    })?;

    client.require(sim::Module::SimIK)?;

    // When simulation is not running, ZMQ message handling could be a bit
    // slow, since the idle loop runs at 8 Hz by default. So let's make
    // sure that the idle loop runs at full speed for this program:

    let default_idle_fps = client.sim_get_int32_param(sim::INTPARAM_IDLE_FPS)?;
    client.sim_set_int32_param(sim::INTPARAM_IDLE_FPS, 0)?;

    let tip = client.sim_get_object("./redundantRob_tip".into(), None)?;
    let target = client.sim_get_object("./redundantRob_target".into(), None)?;
    let base = client.sim_get_object("./redundantRobot".into(), None)?;

    // Create an IK enviroment
    let ik_env = client.sim_ik_create_environment(None)?;

    // create an IK group
    let ik_group = client.sim_ik_create_group(ik_env, None)?;

    // set its resolution method to undamped
    client.sim_ik_set_group_calculation(ik_env, ik_group, sim::IK_PSEUDO_INVERSE_METHOD, 0.0, 6)?;

    let constraint_pos = sim::IK_X_CONSTRAINT | sim::IK_Y_CONSTRAINT | sim::IK_Z_CONSTRAINT;
    let constraint_ori = sim::IK_ALPHA_BETA_CONSTRAINT | sim::IK_GAMMA_CONSTRAINT;
    let constraint_pose = constraint_pos | constraint_ori;
    //  create an IK element based on the scene content
    client.sim_ik_add_element_from_scene(ik_env, ik_group, base, tip, target, constraint_pose)?;

    // create another IK group
    let ik_group_damped = client.sim_ik_create_group(ik_env, None)?;
    client.sim_ik_set_group_calculation(
        ik_env,
        ik_group_damped,
        sim::IK_DAMPED_LEAST_SQUARES_METHOD,
        1.0,
        99,
    )?;

    // create an IK element based on the scene content:
    client.sim_ik_add_element_from_scene(
        ik_env,
        ik_group_damped,
        base,
        tip,
        target,
        constraint_pose,
    )?;

    client.sim_start_simulation()?;

    while client.sim_get_simulation_state()? != sim::SIMULATION_ADVANCING_RUNNING {
        std::thread::sleep(std::time::Duration::from_secs_f64(0.1))
    }

    while client.sim_get_simulation_state()? == sim::SIMULATION_ADVANCING_RUNNING {
        // try to solve with the undamped method:
        let (status, _, _) =
            client.sim_ik_handle_group(ik_env, ik_group, Some(json!({"syncWorlds" : true})))?;

        if status != sim::IKRESULT_SUCCESS {
            // try to solve with the damped method:
            client.sim_ik_handle_group(
                ik_env,
                ik_group_damped,
                Some(json!({"syncWorlds" : true})),
            )?;
        }
    }

    // if you need to make sure we really stopped:
    while client.sim_get_simulation_state()? != sim::SIMULATION_STOPPED {
        std::thread::sleep(std::time::Duration::from_secs_f64(0.1))
    }

    client.sim_ik_erase_environment(ik_env)?;
    client.sim_set_int32_param(sim::INTPARAM_IDLE_FPS, default_idle_fps)?;
    println!("Program ended");

    Ok(())
}
