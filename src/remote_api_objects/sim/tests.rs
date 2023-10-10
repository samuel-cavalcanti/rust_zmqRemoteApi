use super::Sim;
use super::*;
use crate::remote_api_objects::mocks::{assert_payload, MockRemoteAPIClient};
use serde_json::json;

use crate::remote_api_objects::connection_error::RemoteAPIError;

#[test]
fn test_get_simulation_time_functions() -> Result<(), RemoteAPIError> {
    let sim = MockRemoteAPIClient::new_sucess();

    sim.sim_start_simulation()?;
    assert_payload! {sim,b"\xa5dfuncssim.startSimulationdargs\x80duuidx$8a7e3cf4-ae84-4b29-9af3-1e87930b7971cver\x02dlangdrust"};

    sim.sim_stop_simulation()?;
    assert_payload! {sim,b"\xa5dfuncrsim.stopSimulationdargs\x80duuidx$8a7e3cf4-ae84-4b29-9af3-1e87930b7971cver\x02dlangdrust"};

    *sim.result.borrow_mut() = json!({"ret":[1.23],"success":true});

    let time = sim.sim_get_simulation_time()?;

    assert_payload! {sim,b"\xa5dfuncusim.getSimulationTimedargs\x80duuidx$8a7e3cf4-ae84-4b29-9af3-1e87930b7971cver\x02dlangdrust"};

    assert_eq!(time, 1.23);

    Ok(())
}
#[test]
fn test_simple_test_functions() -> Result<(), RemoteAPIError> {
    let sim = MockRemoteAPIClient::new_sucess();

    sim.sim_get_int32_param(26)?;
    assert_payload!(sim, b"\xa5dfuncqsim.getInt32Paramdargs\x81\x18\x1aduuidx$8a7e3cf4-ae84-4b29-9af3-1e87930b7971cver\x02dlangdrust");

    sim.sim_set_int32_param(26, 0)?;
    assert_payload!(sim, b"\xa5dfuncqsim.setInt32Paramdargs\x82\x18\x1a\x00duuidx$8a7e3cf4-ae84-4b29-9af3-1e87930b7971cver\x02dlangdrust");

    sim.sim_create_dummy(0.01)?;
    assert_payload!(
        sim,
       b"\xa5dfuncosim.createDummydargs\x81\xfb?\x84z\xe1G\xae\x14{duuidx$8a7e3cf4-ae84-4b29-9af3-1e87930b7971cver\x02dlangdrust" 
    );

    let state = sim.sim_get_simulation_state()?;
    assert_eq!(state, 1);
    assert_payload!(sim, b"\xa5dfuncvsim.getSimulationStatedargs\x80duuidx$8a7e3cf4-ae84-4b29-9af3-1e87930b7971cver\x02dlangdrust");

    sim.sim_add_log(
        VERBOSITY_SCRIPTINFOS,
        String::from(
            "Simulation time: 0.00 [s] (simulation running asynchronously  to client, i.e. stepped)",
        ),
    )?;
    assert_payload!(sim,b"\xa5dfuncjsim.addLogdargs\x82\x19\x01\xc2xVSimulation time: 0.00 [s] (simulation running asynchronously  to client, i.e. stepped)duuidx$8a7e3cf4-ae84-4b29-9af3-1e87930b7971cver\x02dlangdrust");

    let objects = (465..515).collect::<Vec<i64>>();
    sim.sim_remove_objects(objects)?;
    assert_payload!(sim, b"\xa5dfuncqsim.removeObjectsdargs\x81\x982\x19\x01\xd1\x19\x01\xd2\x19\x01\xd3\x19\x01\xd4\x19\x01\xd5\x19\x01\xd6\x19\x01\xd7\x19\x01\xd8\x19\x01\xd9\x19\x01\xda\x19\x01\xdb\x19\x01\xdc\x19\x01\xdd\x19\x01\xde\x19\x01\xdf\x19\x01\xe0\x19\x01\xe1\x19\x01\xe2\x19\x01\xe3\x19\x01\xe4\x19\x01\xe5\x19\x01\xe6\x19\x01\xe7\x19\x01\xe8\x19\x01\xe9\x19\x01\xea\x19\x01\xeb\x19\x01\xec\x19\x01\xed\x19\x01\xee\x19\x01\xef\x19\x01\xf0\x19\x01\xf1\x19\x01\xf2\x19\x01\xf3\x19\x01\xf4\x19\x01\xf5\x19\x01\xf6\x19\x01\xf7\x19\x01\xf8\x19\x01\xf9\x19\x01\xfa\x19\x01\xfb\x19\x01\xfc\x19\x01\xfd\x19\x01\xfe\x19\x01\xff\x19\x02\x00\x19\x02\x01\x19\x02\x02duuidx$8a7e3cf4-ae84-4b29-9af3-1e87930b7971cver\x02dlangdrust");

    Ok(())
}

#[test]
fn test_p_controller_functions() -> Result<(), RemoteAPIError> {
    let mut sim = MockRemoteAPIClient::new_sucess();
    sim.uuid = "92e1fde9-2d2c-4258-aafc-06d89efb8a7a".to_string();

    let id = sim.sim_get_object("/Cuboid[0]/joint".to_string(), None)?;
    assert_eq!(id, 1);
    assert_payload!(sim,b"\xa5dfuncmsim.getObjectdargs\x81p/Cuboid[0]/jointduuidx$92e1fde9-2d2c-4258-aafc-06d89efb8a7acver\x02dlangdrust");

    *sim.result.borrow_mut() = json!({"ret":[1.2],"success":true});
    let pos = sim.sim_get_joint_position(9)?;
    assert_eq!(pos, 1.2);
    assert_payload!(sim, b"\xa5dfunctsim.getJointPositiondargs\x81\tduuidx$92e1fde9-2d2c-4258-aafc-06d89efb8a7acver\x02dlangdrust"
);

    *sim.result.borrow_mut() = json!({"ret":[1],"success":true});
    sim.sim_set_joint_target_velocity(9, std::f64::consts::TAU, None)?;
    assert_payload!(
        sim,
b"\xa5dfuncx\x1asim.setJointTargetVelocitydargs\x82\t\xfb@\x19!\xfbTD-\x18duuidx$92e1fde9-2d2c-4258-aafc-06d89efb8a7acver\x02dlangdrust"
    );

    sim.sim_set_joint_max_force(9, 100.0)?;
    assert_payload!(sim, b"\xa5dfunctsim.setJointMaxForcedargs\x82\t\xf9V@duuidx$92e1fde9-2d2c-4258-aafc-06d89efb8a7acver\x02dlangdrust");

    Ok(())
}

#[test]
fn test_synchronous_image_transmission_functions() -> Result<(), RemoteAPIError> {
    let image = include_bytes!("../../../assets/image.bin").to_vec();
    assert_eq!(image.len(), 196608);

    let expected_payload = include_bytes!("../../../assets/cbor_image.bin").to_vec();
    assert_eq!(expected_payload.len(), 196649);

    let sim = MockRemoteAPIClient::new_sucess();

    sim.sim_set_vision_sensor_img(22, image, None, None, None)?;
    assert_payload(&sim, expected_payload);

    Ok(())
}

#[test]
fn test_send_ik_movement_sequence_mov_functions() -> Result<(), RemoteAPIError> {
    env_logger::init();

    let mut sim = MockRemoteAPIClient::new_sucess();
    sim.uuid = "782cad7d-4493-42f2-a715-69f740f26ea9".to_string();

    let handle_id = sim.sim_get_object(String::from("/LBR4p"), None)?;
    assert_eq!(handle_id, 1);

    assert_payload!(sim, b"\xa5dfuncmsim.getObjectdargs\x81f/LBR4pduuidx$782cad7d-4493-42f2-a715-69f740f26ea9cver\x02dlangdrust");

    let script_handle = sim.sim_get_script(1, Some(13), None)?;
    assert_eq!(script_handle, 1);
    assert_payload!(sim, b"\xa5dfuncmsim.getScriptdargs\x82\x01\rduuidx$782cad7d-4493-42f2-a715-69f740f26ea9cver\x02dlangdrust");

    *sim.result.borrow_mut() = serde_json::json!(json!({"ret":[[114,101,97,100,121]]}));
    let signal = sim.sim_get_string_signal(String::from("/LBR4p_executedMovId"))?;
    assert_payload!(
        sim,
        b"\xa5dfuncssim.getStringSignaldargs\x81t/LBR4p_executedMovIdduuidx$782cad7d-4493-42f2-a715-69f740f26ea9cver\x02dlangdrust"
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

    assert_payload!(sim,b"\xa5dfuncvsim.callScriptFunctiondargs\x82x\x1aremoteApi_getPoseAndConfig\x1a\x00\x0fi\x99duuidx$782cad7d-4493-42f2-a715-69f740f26ea9cver\x02dlangdrust");

    let json_arg = json!( {"id": "movSeq1", "type": "mov", "targetPose": [0, 0, 0.85, 0, 0, 0, 1], "maxVel": 0.1, "maxAccel": 0.01});
    let _json_out = sim.sim_call_script_function(
        String::from("remoteApi_movementDataFunction"),
        script_handle,
        Some(json_arg),
    )?;

    assert_payload!(sim,b"\xa5dfuncvsim.callScriptFunctiondargs\x83x\x1eremoteApi_movementDataFunction\x1a\x00\x0fi\x99\xa5bidgmovSeq1hmaxAccel\xfb?\x84z\xe1G\xae\x14{fmaxVel\xfb?\xb9\x99\x99\x99\x99\x99\x9ajtargetPose\x87\x00\x00\xfb?\xeb333333\x00\x00\x00\x01dtypecmovduuidx$782cad7d-4493-42f2-a715-69f740f26ea9cver\x02dlangdrust");

    Ok(())
}

#[test]
fn test_get_step() {
    let sim = MockRemoteAPIClient::new_sucess();

    sim.sim_step().unwrap();
    assert_payload! {sim,b"\xa5dfunchsim.stepdargs\x80duuidx$8a7e3cf4-ae84-4b29-9af3-1e87930b7971cver\x02dlangdrust"};
}

#[test]
fn test_set_stepping() {
    let sim = MockRemoteAPIClient::new_sucess();

    sim.sim_set_stepping(true).unwrap();
    assert_payload!(sim,b"\xa5dfuncosim.setSteppingdargs\x81\xf5duuidx$8a7e3cf4-ae84-4b29-9af3-1e87930b7971cver\x02dlangdrust");

    sim.sim_set_stepping(false).unwrap();
    assert_payload! {sim,b"\xa5dfuncosim.setSteppingdargs\x81\xf4duuidx$8a7e3cf4-ae84-4b29-9af3-1e87930b7971cver\x02dlangdrust"};
}
