use ciborium::cbor;
use ciborium::value::Value;

pub trait RawRequest {
    fn to_raw_request(&self) -> Vec<u8>;
}

pub struct ZmqRequest {
    pub function_name: String,
    pub args: Vec<Value>,
}

macro_rules! requests {
    ( $(($rust_fn:tt,$function_name:literal)),+ ) => {
        $(
            pub fn $rust_fn() -> ZmqRequest {
                ZmqRequest {
                    function_name: format!("sim.{}",$function_name),
                    args: vec![],
                }
        }
    )*
    };
}

impl ZmqRequest {
    requests! {
            (start_simulation,"startSimulation"),
            (stop_simulation,"stopSimulation"),
            (switch_thread,"switchThread"),
            (quit_simulator,"quitSimulator"),
            (pause_simulation,"pauseSimulation"),
            (handle_joint_motion,"handleJointMotion"),
            (handle_simulation_start,"handleSimulationStart"),
            (handle_sensing_start,"handleSensingStart"),
            (get_user_variables,"getUserVariables"),
            (get_thread_switch_timing, "getThreadSwitchTiming"),
            (get_thread_switch_allowed,"getThreadSwitchAllowed"),
            (get_thread_id,"getThreadId"),
            (get_thread_exist_request,"getThreadExistRequest"),
            (get_thread_automatic_switch,"getThreadAutomaticSwitch"),
            (get_system_time,"getSystemTime"),
            (get_simulation_time_step,"getSimulationTimeStep"),
            (get_simulation_time,"getSimulationTime"),
            (get_simulation_state,"getSimulationState"),
            (get_simulator_message,"getSimulatorMessage"),
            (get_real_time_simulation,"getRealTimeSimulation"),
            (get_persistent_data_tags,"getPersistentDataTags"),
            (get_page,"getPage"),
            (get_object_selection,"getObjectSelection"),
            (get_genesis_events,"getGenesisEvents"),
            (get_navigation_mode,"getNavigationMode"),
            (announce_scene_content_change,"announceSceneContentChange"),
            (build_identity_matrix,"buildIdentityMatrix"),
            (close_scene,"closeScene"),
            (create_environment,"createEnvironment")
    }

    pub fn remote_api_info(object: String) -> ZmqRequest {
        ZmqRequest {
            function_name: String::from("zmqRemoteApi.info"),
            args: vec![cbor!(object).unwrap()],
        }
    }

    pub fn step(uuid: String) -> ZmqRequest {
        ZmqRequest {
            function_name: "step".to_string(),
            args: vec![cbor!(uuid).unwrap()],
        }
    }

    pub fn set_stepping(enable: bool, uuid: String) -> ZmqRequest {
        ZmqRequest {
            function_name: "setStepping".to_string(),
            args: vec![cbor!(enable).unwrap(), cbor!(uuid).unwrap()],
        }
    }
}

impl RawRequest for ZmqRequest {
    fn to_raw_request(&self) -> Vec<u8> {
        let enconded = cbor!({"func"=> self.function_name, "args"=> self.args}).unwrap();

        log::trace!("sending: {:?}", enconded);

        let mut bytes = Vec::new();
        ciborium::ser::into_writer(&enconded, &mut bytes).unwrap();

        bytes
    }
}
