use super::Sim;
use super::SimIK;
use super::*;
use crate::remote_api_objects::mocks::MockRemoteAPIClient;
use crate::sim;
use crate::RemoteApiClientInterface;
use serde_json::json;

use crate::remote_api_objects::connection_error::RemoteAPIError;

#[test]
fn test_get_simulation_time_functions() -> Result<(), RemoteAPIError> {
    let mut sim = MockRemoteAPIClient::new_sucess();
    sim.uuid = "773fb818-718b-4337-ba83-33a13209a3a1".into();

    sim.sim_start_simulation()?;
    assert_payload! {sim,b"\xa6dfuncssim.startSimulationdargs\x80duuidx$773fb818-718b-4337-ba83-33a13209a3a1cver\x02dlangdrusteargsL\x00"};

    sim.sim_stop_simulation()?;
    assert_payload! {sim,b"\xa6dfuncrsim.stopSimulationdargs\x80duuidx$773fb818-718b-4337-ba83-33a13209a3a1cver\x02dlangdrusteargsL\x00"};

    *sim.result.borrow_mut() = json!({"ret":[1.23],"success":true});

    let time = sim.sim_get_simulation_time()?;

    assert_payload! {sim,b"\xa6dfuncusim.getSimulationTimedargs\x80duuidx$773fb818-718b-4337-ba83-33a13209a3a1cver\x02dlangdrusteargsL\x00"};

    assert_eq!(time, 1.23);

    Ok(())
}
#[test]
fn test_simple_test_functions() -> Result<(), RemoteAPIError> {
    let sim = MockRemoteAPIClient::new_sucess();

    sim.sim_get_int32_param(26)?;
    assert_payload!(sim, b"\xa6dfuncqsim.getInt32Paramdargs\x81\x18\x1aduuidx$8a7e3cf4-ae84-4b29-9af3-1e87930b7971cver\x02dlangdrusteargsL\x01");

    sim.sim_set_int32_param(26, 0)?;
    assert_payload!(sim, b"\xa6dfuncqsim.setInt32Paramdargs\x82\x18\x1a\x00duuidx$8a7e3cf4-ae84-4b29-9af3-1e87930b7971cver\x02dlangdrusteargsL\x02");

    sim.sim_create_dummy(0.01)?;
    assert_payload!(
        sim,
       b"\xa6dfuncosim.createDummydargs\x81\xfb?\x84z\xe1G\xae\x14{duuidx$8a7e3cf4-ae84-4b29-9af3-1e87930b7971cver\x02dlangdrusteargsL\x01" 
    );

    let state = sim.sim_get_simulation_state()?;
    assert_eq!(state, 1);
    assert_payload!(sim, b"\xa6dfuncvsim.getSimulationStatedargs\x80duuidx$8a7e3cf4-ae84-4b29-9af3-1e87930b7971cver\x02dlangdrusteargsL\x00");

    sim.sim_add_log(
        VERBOSITY_SCRIPTINFOS,
        String::from(
            "Simulation time: 0.00 [s] (simulation running asynchronously  to client, i.e. stepped)",
        ),
    )?;
    assert_payload!(sim,b"\xa6dfuncjsim.addLogdargs\x82\x19\x01\xc2xVSimulation time: 0.00 [s] (simulation running asynchronously  to client, i.e. stepped)duuidx$8a7e3cf4-ae84-4b29-9af3-1e87930b7971cver\x02dlangdrusteargsL\x02");

    let objects = (465..515).collect::<Vec<i64>>();
    sim.sim_remove_objects(objects)?;
    assert_payload!(sim, b"\xa6dfuncqsim.removeObjectsdargs\x81\x982\x19\x01\xd1\x19\x01\xd2\x19\x01\xd3\x19\x01\xd4\x19\x01\xd5\x19\x01\xd6\x19\x01\xd7\x19\x01\xd8\x19\x01\xd9\x19\x01\xda\x19\x01\xdb\x19\x01\xdc\x19\x01\xdd\x19\x01\xde\x19\x01\xdf\x19\x01\xe0\x19\x01\xe1\x19\x01\xe2\x19\x01\xe3\x19\x01\xe4\x19\x01\xe5\x19\x01\xe6\x19\x01\xe7\x19\x01\xe8\x19\x01\xe9\x19\x01\xea\x19\x01\xeb\x19\x01\xec\x19\x01\xed\x19\x01\xee\x19\x01\xef\x19\x01\xf0\x19\x01\xf1\x19\x01\xf2\x19\x01\xf3\x19\x01\xf4\x19\x01\xf5\x19\x01\xf6\x19\x01\xf7\x19\x01\xf8\x19\x01\xf9\x19\x01\xfa\x19\x01\xfb\x19\x01\xfc\x19\x01\xfd\x19\x01\xfe\x19\x01\xff\x19\x02\x00\x19\x02\x01\x19\x02\x02duuidx$8a7e3cf4-ae84-4b29-9af3-1e87930b7971cver\x02dlangdrusteargsL\x01");

    Ok(())
}

#[test]
fn test_p_controller_functions() -> Result<(), RemoteAPIError> {
    let mut sim = MockRemoteAPIClient::new_sucess();
    sim.uuid = "9ea91dac-2753-4033-af92-3d0832b898fa".to_string();

    let id = sim.sim_get_object("/Cuboid[0]/joint".to_string(), None)?;
    assert_eq!(id, 1);
    assert_payload!(sim,b"\xa6dfuncmsim.getObjectdargs\x81p/Cuboid[0]/jointduuidx$9ea91dac-2753-4033-af92-3d0832b898facver\x02dlangdrusteargsL\x01");

    *sim.result.borrow_mut() = json!({"ret":[1.2],"success":true});
    let pos = sim.sim_get_joint_position(9)?;
    assert_eq!(pos, 1.2);
    assert_payload!(sim, b"\xa6dfunctsim.getJointPositiondargs\x81\tduuidx$9ea91dac-2753-4033-af92-3d0832b898facver\x02dlangdrusteargsL\x01"
);

    *sim.result.borrow_mut() = json!({"ret":[1],"success":true});
    sim.sim_set_joint_target_velocity(9, std::f64::consts::TAU, None)?;
    assert_payload!(
        sim,
b"\xa6dfuncx\x1asim.setJointTargetVelocitydargs\x82\t\xfb@\x19!\xfbTD-\x18duuidx$9ea91dac-2753-4033-af92-3d0832b898facver\x02dlangdrusteargsL\x02"
    );

    sim.sim_set_joint_max_force(9, 100.0)?;
    assert_payload!(sim, b"\xa6dfunctsim.setJointMaxForcedargs\x82\t\xf9V@duuidx$9ea91dac-2753-4033-af92-3d0832b898facver\x02dlangdrusteargsL\x02");

    Ok(())
}

#[test]
fn test_synchronous_image_transmission_functions() -> Result<(), RemoteAPIError> {
    let mut sim = MockRemoteAPIClient::new_sucess();
    sim.uuid = "330c0929-05e6-4749-8212-18d0dec5b1e2".to_string();

    let vision_sensor_handle = 11;
    sim.sim_get_vision_sensor_img(vision_sensor_handle, None, None, None, None)?;
    assert_payload!(sim,b"\xa6dfuncvsim.getVisionSensorImgdargs\x81\x0bduuidx$330c0929-05e6-4749-8212-18d0dec5b1e2cver\x02dlangdrusteargsL\x01");

    let img = [0, 0, 0, 0, 0, 0, 0, 0, 0].to_vec();

    sim.sim_set_vision_sensor_img(vision_sensor_handle, img.clone(), None, None, None)?;

    assert_payload!(sim, b"\xa6dfuncvsim.setVisionSensorImgdargs\x82\x0bI\x00\x00\x00\x00\x00\x00\x00\x00\x00duuidx$330c0929-05e6-4749-8212-18d0dec5b1e2cver\x02dlangdrusteargsL\x02");
    Ok(())
}

#[test]
fn test_send_ik_movement_sequence_mov_functions() -> Result<(), RemoteAPIError> {
    env_logger::init();

    let mut sim = MockRemoteAPIClient::new_sucess();
    sim.uuid = "782cad7d-4493-42f2-a715-69f740f26ea9".to_string();

    let handle_id = sim.sim_get_object(String::from("/LBR4p"), None)?;
    assert_eq!(handle_id, 1);

    assert_payload!(sim, b"\xa6dfuncmsim.getObjectdargs\x81f/LBR4pduuidx$782cad7d-4493-42f2-a715-69f740f26ea9cver\x02dlangdrusteargsL\x01");

    let script_handle = sim.sim_get_script(1, Some(13), None)?;
    assert_eq!(script_handle, 1);
    assert_payload!(sim, b"\xa6dfuncmsim.getScriptdargs\x82\x01\rduuidx$782cad7d-4493-42f2-a715-69f740f26ea9cver\x02dlangdrusteargsL\x02");

    *sim.result.borrow_mut() = serde_json::json!(json!({"ret":[[114,101,97,100,121]]}));
    let signal = sim.sim_get_string_signal(String::from("/LBR4p_executedMovId"))?;
    assert_payload!(
        sim,
        b"\xa6dfuncssim.getStringSignaldargs\x81t/LBR4p_executedMovIdduuidx$782cad7d-4493-42f2-a715-69f740f26ea9cver\x02dlangdrusteargsL\x01"
    );

    let signal = String::from_utf8(signal.unwrap()).unwrap();
    assert_eq!(signal, "ready".to_string());

    let script_handle = 1010073;

    *sim.result.borrow_mut() = json!({"ret":[[0.005610920649735013,-0.00021765310913694194,1.0726233629223674,0.00015426587889780238,0.00035320923886966716,0.0002262267625328107,0.999999900133357],[-0.00016239643955495708,-0.523748620585974,6.142930608632469e-10,1.047150871028287,0.00003561749575364814,-0.523602489795632,-3.7685468967652014e-8]]});
    let _json = sim.sim_call_script_function(
        String::from("remoteApi_getPoseAndConfig"),
        script_handle,
        None,
    )?;

    assert_payload!(sim,b"\xa6dfuncvsim.callScriptFunctiondargs\x82x\x1aremoteApi_getPoseAndConfig\x1a\x00\x0fi\x99duuidx$782cad7d-4493-42f2-a715-69f740f26ea9cver\x02dlangdrusteargsL\x02");

    let json_arg = json!( {"id": "movSeq1", "type": "mov", "targetPose": [0, 0, 0.85, 0, 0, 0, 1], "maxVel": 0.1, "maxAccel": 0.01});
    let _json_out = sim.sim_call_script_function(
        String::from("remoteApi_movementDataFunction"),
        script_handle,
        Some(json_arg),
    )?;

    assert_payload!(sim,b"\xa6dfuncvsim.callScriptFunctiondargs\x83x\x1eremoteApi_movementDataFunction\x1a\x00\x0fi\x99\xa5bidgmovSeq1hmaxAccel\xfb?\x84z\xe1G\xae\x14{fmaxVel\xfb?\xb9\x99\x99\x99\x99\x99\x9ajtargetPose\x87\x00\x00\xfb?\xeb333333\x00\x00\x00\x01dtypecmovduuidx$782cad7d-4493-42f2-a715-69f740f26ea9cver\x02dlangdrusteargsL\x03");

    Ok(())
}

#[test]
fn test_get_step() {
    let mut sim = MockRemoteAPIClient::new_sucess();
    sim.uuid = "773fb818-718b-4337-ba83-33a13209a3a1".into();

    sim.sim_step().unwrap();
    assert_payload! {sim,b"\xa6dfunchsim.stepdargs\x80duuidx$773fb818-718b-4337-ba83-33a13209a3a1cver\x02dlangdrusteargsL\x00"};
}

#[test]
fn test_set_stepping() {
    let mut sim = MockRemoteAPIClient::new_sucess();
    sim.uuid = "773fb818-718b-4337-ba83-33a13209a3a1".into();

    sim.sim_set_stepping(true).unwrap();
    assert_payload!(sim,b"\xa6dfuncosim.setSteppingdargs\x81\xf5duuidx$773fb818-718b-4337-ba83-33a13209a3a1cver\x02dlangdrusteargsL\x01");

    sim.sim_set_stepping(false).unwrap();
    assert_payload! {sim,b"\xa6dfuncosim.setSteppingdargs\x81\xf4duuidx$773fb818-718b-4337-ba83-33a13209a3a1cver\x02dlangdrusteargsL\x01"};
}

#[test]
fn test_sim_ik_example() {
    let mut sim = MockRemoteAPIClient::new_sucess();
    sim.uuid = "fdea473e-ef12-43e0-bd41-8248ea153e17".into();

    sim.require(Plugin::SimIK).unwrap();
    assert_payload!(sim,b"\xa6dfunctzmqRemoteApi.requiredargs\x81esimIKduuidx$fdea473e-ef12-43e0-bd41-8248ea153e17cver\x02dlangdrusteargsL\x01");

    sim.sim_ik_create_environment(None).unwrap();
    assert_payload!(sim,b"\xa6dfuncwsimIK.createEnvironmentdargs\x80duuidx$fdea473e-ef12-43e0-bd41-8248ea153e17cver\x02dlangdrusteargsL\x00");

    let env = 15;
    sim.sim_ik_create_group(env, None).unwrap();
    assert_payload!(sim,b"\xa6dfuncqsimIK.createGroupdargs\x81\x0fduuidx$fdea473e-ef12-43e0-bd41-8248ea153e17cver\x02dlangdrusteargsL\x01");

    let ik_group = 2030003;

    sim.sim_ik_set_group_calculation(env, ik_group, sim::IK_PSEUDO_INVERSE_METHOD, 0.0, 6)
        .unwrap();
    assert_payload!(sim,b"\xa6dfuncx\x19simIK.setGroupCalculationdargs\x85\x0f\x1a\x00\x1e\xf9\xb3\x00\xf9\x00\x00\x06duuidx$fdea473e-ef12-43e0-bd41-8248ea153e17cver\x02dlangdrusteargsL\x05");
    let tip = 12;
    let target = 3;
    let base = 1;

    let constraint_pos = sim::IK_X_CONSTRAINT | sim::IK_Y_CONSTRAINT | sim::IK_Z_CONSTRAINT;
    let constraint_ori = sim::IK_ALPHA_BETA_CONSTRAINT | sim::IK_GAMMA_CONSTRAINT;
    let constraint_pose = constraint_pos | constraint_ori;

    sim.sim_ik_add_element_from_scene(env, ik_group, base, tip, target, constraint_pose)
        .unwrap();

    assert_payload!(sim, b"\xa6dfuncx\x19simIK.addElementFromScenedargs\x86\x0f\x1a\x00\x1e\xf9\xb3\x01\x0c\x03\x18\x1fduuidx$fdea473e-ef12-43e0-bd41-8248ea153e17cver\x02dlangdrusteargsL\x06");

    sim.sim_ik_handle_group(env, ik_group, Some(json!({"syncWorlds" : true})))
        .unwrap();
    assert_payload!(sim, b"\xa6dfuncqsimIK.handleGroupdargs\x83\x0f\x1a\x00\x1e\xf9\xb3\xa1jsyncWorlds\xf5duuidx$fdea473e-ef12-43e0-bd41-8248ea153e17cver\x02dlangdrusteargsL\x03");
}
