use super::remote_api_objects_const;
use super::RemoteAPIObjects;
use crate::log_utils;
use crate::remote_api_client::RemoteApiClientInterface;
use serde_json::{json, Value as JsonValue};
use std::cell::RefCell;
use zmq::Error;

struct MockRemoteAPIClient {
    payload: RefCell<Vec<u8>>,
    result: RefCell<JsonValue>,
}

impl MockRemoteAPIClient {
    fn get_payload(&self) -> Vec<u8> {
        self.payload.borrow_mut().clone()
    }
}

impl RemoteApiClientInterface for MockRemoteAPIClient {
    fn send_raw_request(&self, request: Vec<u8>) -> Result<JsonValue, Error> {
        *self.payload.borrow_mut() = request;
        let result = self.result.clone().into_inner();
        Ok(result)
    }
}

fn assert_payload(client: &MockRemoteAPIClient, payload: Vec<u8>) {
    assert_eq!(client.get_payload(), payload)
}
macro_rules! assert_payload {
    ($client:ident,$payload:literal) => {
        assert_eq!($client.get_payload(), $payload.to_vec());
    };
}

#[test]
fn test_get_simulation_time_functions() -> Result<(), zmq::Error> {
    let client = MockRemoteAPIClient {
        payload: RefCell::new(vec![]),
        result: RefCell::new(json!({"ret":[1],"success":true})),
    };
    let sim = RemoteAPIObjects::new(&client);

    sim.start_simulation()?;
    assert_payload! {client,b"\xa2dfuncssim.startSimulationdargs\x80"};

    sim.stop_simulation()?;
    assert_payload! {client,b"\xa2dfuncrsim.stopSimulationdargs\x80"};

    *client.result.borrow_mut() = json!({"ret":[1.23],"success":true});

    let time = sim.get_simulation_time()?;

    assert_payload! {client,b"\xa2dfuncusim.getSimulationTimedargs\x80"};

    assert_eq!(time, 1.23);

    Ok(())
}

#[test]
fn test_simple_test_functions() -> Result<(), zmq::Error> {
    let client = MockRemoteAPIClient {
        payload: RefCell::new(vec![]),
        result: RefCell::new(json!({"ret":[1],"success":true})),
    };
    let sim = RemoteAPIObjects::new(&client);

    sim.get_int32_param(26)?;
    assert_payload!(client, b"\xa2dfuncqsim.getInt32Paramdargs\x81\x18\x1a");

    sim.set_int32_param(26, 0)?;
    assert_payload!(client, b"\xa2dfuncqsim.setInt32Paramdargs\x82\x18\x1a\x00");

    sim.create_dummy(0.01)?;
    assert_payload!(
        client,
        b"\xa2dfuncosim.createDummydargs\x81\xfb?\x84z\xe1G\xae\x14{"
    );

    let state = sim.get_simulation_state()?;
    assert_eq!(state, 1);
    assert_payload!(client, b"\xa2dfuncvsim.getSimulationStatedargs\x80");

    sim.add_log(
        remote_api_objects_const::VERBOSITY_SCRIPTINFOS,
        String::from(
            "Simulation time: 0.00 [s] (simulation running synchronously to client, i.e. stepped)",
        ),
    )?;
    assert_payload!(client,b"\xa2dfuncjsim.addLogdargs\x82\x19\x01\xc2xTSimulation time: 0.00 [s] (simulation running synchronously to client, i.e. stepped)");

    sim.remove_object(222)?;
    assert_payload!(client, b"\xa2dfuncpsim.removeObjectdargs\x81\x18\xde");

    Ok(())
}

#[test]
fn test_p_controller_functions() -> Result<(), zmq::Error> {
    let client = MockRemoteAPIClient {
        payload: RefCell::new(vec![]),
        result: RefCell::new(json!({"ret":[1],"success":true})),
    };
    let sim = RemoteAPIObjects::new(&client);

    let id = sim.get_object("/Cuboid[0]/joint".to_string(), None)?;
    assert_eq!(id, 1);
    assert_payload!(client, b"\xa2dfuncmsim.getObjectdargs\x81p/Cuboid[0]/joint");

    *client.result.borrow_mut() = json!({"ret":[1.2],"success":true});
    let pos = sim.get_joint_position(9)?;
    assert_eq!(pos, 1.2);
    assert_payload!(client, b"\xa2dfunctsim.getJointPositiondargs\x81\t");

    *client.result.borrow_mut() = json!({"ret":[1],"success":true});
    sim.set_joint_target_velocity(9, 6.283185307179586, None, None)?;
    assert_payload!(
        client,
        b"\xa2dfuncx\x1asim.setJointTargetVelocitydargs\x82\t\xfb@\x19!\xfbTD-\x18"
    );

    sim.set_joint_max_force(9, 100.0)?;
    assert_payload!(client, b"\xa2dfunctsim.setJointMaxForcedargs\x82\t\xf9V@");

    Ok(())
}

#[test]
fn test_synchronous_image_transmission_functions() -> Result<(), zmq::Error> {
    let image = include_bytes!("../../assets/image.bin").to_vec();
    assert_eq!(image.len(), 196608);

    let expected_payload = include_bytes!("../../assets/cbor_image.bin").to_vec();
    assert_eq!(expected_payload.len(), 196649);

    let client = MockRemoteAPIClient {
        payload: RefCell::new(vec![]),
        result: RefCell::new(json!({"ret":[1],"success":true})),
    };
    let sim = RemoteAPIObjects::new(&client);

    sim.set_vision_sensor_img(22, image, None, None, None)?;
    assert_payload(&client, expected_payload);

    Ok(())
}

#[test]
fn test_send_ik_movement_sequence_mov_functions() -> Result<(), zmq::Error> {
    env_logger::init();

    let client = MockRemoteAPIClient {
        payload: RefCell::new(vec![]),
        result: RefCell::new(json!({"ret":[1],"success":true})),
    };

    let sim = RemoteAPIObjects::new(&client);

    let handle_id = sim.get_object(String::from("/LBR4p"), None)?;
    assert_eq!(handle_id, 1);

    assert_payload!(client, b"\xa2dfuncmsim.getObjectdargs\x81f/LBR4p");

    let script_handle = sim.get_script(1, Some(13), None)?;
    assert_eq!(script_handle, 1);
    assert_payload!(client, b"\xa2dfuncmsim.getScriptdargs\x82\x01\r");

    *client.result.borrow_mut() = serde_json::json!(json!({"ret":{},"success":true}));
    let signal = sim.get_string_signal(String::from("/LBR4p_executedMovId"))?;
    assert_payload!(
        client,
        b"\xa2dfuncssim.getStringSignaldargs\x81t/LBR4p_executedMovId"
    );
    assert_eq!(signal.is_empty(), true);

    *client.result.borrow_mut() = json!({"ret":["ready"],"success":true});
    let signal = sim.get_string_signal(String::from("/LBR4p_executedMovId"))?;
    assert_payload!(
        client,
        b"\xa2dfuncssim.getStringSignaldargs\x81t/LBR4p_executedMovId"
    );
    assert_eq!(signal, "ready");

    *client.result.borrow_mut() = json!({"success": true,"ret": [[0.0051022060215473175, -7.424468640238047e-05, 1.072655200958252, 3.102603295701556e-05, 3.6248049582354724e-05, -0.0001559544907649979, 1], [-0.0008084774017333984, -0.5228924751281738, -0.0008318424224853516, 1.0469286441802979, 0.0001537799835205078, -0.5236260890960693, -6.67572021484375e-06]]});
    let json =
        sim.call_script_function(String::from("remoteApi_getPoseAndConfig"), 2050006, None)?;
    assert_payload!(client,b"\xa2dfuncvsim.callScriptFunctiondargs\x82x\x1aremoteApi_getPoseAndConfig\x1a\x00\x1fG\xd6");
    assert_eq!(
        json,
        json! {[[0.0051022060215473175, -7.424468640238047e-05, 1.072655200958252, 3.102603295701556e-05, 3.6248049582354724e-05, -0.0001559544907649979, 1], [-0.0008084774017333984, -0.5228924751281738, -0.0008318424224853516, 1.0469286441802979, 0.0001537799835205078, -0.5236260890960693, -6.67572021484375e-06]]}
    );

    *client.result.borrow_mut() = json! {{"ret":[{}],"success":true}};
    let json_arg = json!( {"id": "movSeq1", "type": "mov", "targetPose": [0, 0, 0.85, 0, 0, 0, 1], "maxVel": 0.1, "maxAccel": 0.01});
    let json_out = sim.call_script_function(
        String::from("remoteApi_movementDataFunction"),
        2050006,
        Some(json_arg),
    )?;
    let payload = client.get_payload();
    let decoded: ciborium::value::Value = ciborium::de::from_reader(payload.as_slice()).unwrap();

    let json = serde_json::json!(decoded);
    println!("Json response: {}", json);
    println!("{}", log_utils::to_byte_array_string(&payload));

    assert_payload!(client,b"\xa2dfuncvsim.callScriptFunctiondargs\x83x\x1eremoteApi_movementDataFunction\x1a\x00\x1fG\xd6\xa5bidgmovSeq1hmaxAccel\xfb?\x84z\xe1G\xae\x14{fmaxVel\xfb?\xb9\x99\x99\x99\x99\x99\x9ajtargetPose\x87\x00\x00\xfb?\xeb333333\x00\x00\x00\x01dtypecmov");
    assert_eq!(json_out, json!({}));

    Ok(())
}
