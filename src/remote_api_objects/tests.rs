use super::remote_api_objects_const;
use super::RemoteAPIObjects;
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
