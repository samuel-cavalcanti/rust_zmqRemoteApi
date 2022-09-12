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
    ( $(($rust_fn:ident,$function_name:literal $(, ( $($arg_id:ident : $arg_type:ty),+ ) )?  )),+ $(,)? ) => { //$(($()? ))?
        $(
            pub fn $rust_fn($( $($arg_id:$arg_type),* )*) -> ZmqRequest {
                let mut args:Vec<Value> = Vec::new();

                $($(args.push(cbor!($arg_id).unwrap());)*)*

                ZmqRequest {
                    function_name: format!("sim.{}",$function_name),
                    args,
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
            (create_environment,"createEnvironment"),
            (add_drawing_object_item,"addDrawingObjectItem",(drawing_object_handle:i64,item_data:Vec<f64>)),
            (add_force,"addForce",(shape_handle:i64,position:Vec<f64>,force:Vec<f64>)),
            (add_item_to_collection,"addItemToCollection",(collection_handle:i64,what:i64,object_handle:i64,options:i64)),
            (add_log,"addLog",(verbosity_level:i64,log_message:String)),
            (add_particle_object_item,"addParticleObjectItem",(particle_object_handle:i64,item_data:Vec<f64>)),
            (add_script,"addScript",(add_script:i64)),
            (alpha_beta_gamma_to_yaw_pitch_roll,"alphaBetaGammaToYawPitchRoll",(alpha:f64, beta:f64,gamma:f64)),
            (associate_script_with_object,"associateScriptWithObject",(script_handle:i64, object_handle:i64)),
            (auxiliary_console_close,"auxiliaryConsoleClose",(console_handle:i64)),
            (auxiliary_console_print,"auxiliaryConsolePrint",(console_handle:i64,text:String)),
            (auxiliary_console_show,"auxiliaryConsoleShow",(console_handle:i64,show_state:bool)),
            (build_matrix,"buildMatrix",(position:Vec<f64>,euler_angles:Vec<f64>)),
            (build_matrix_q,"buildMatrixQ",(position:Vec<f64>,quaternion:Vec<f64>)),
            (check_collision,"checkCollision",(entity1_handle:i64,entity2_handle:i64)),
            (check_collision_ex,"checkCollisionEx",(entity1_handle:i64,entity2_handle:i64)),
            (check_octree_point_occupancy,"checkOctreePointOccupancy",(octree_handle:i64,options:i64,points:Vec<f64>)),
            (check_proximity_sensor,"checkProximitySensor",(sensor_handle:i64,entity_handle:i64)),
            (check_proximity_sensor_ex,"checkProximitySensorEx",(sensor_handle:i64,entity_handle:i64,mode:i64,threshold:f64,max_angle:f64)),
            (check_proximity_sensor_ex2,"checkProximitySensorEx2",(sensor_handle:i64,vertices:Vec<f64>,item_type:i64,item_count:i64,mode:i64,threshold:f64,max_angle:f64)),
            (check_vision_sensor,"checkVisionSensor",(sensor_handle:i64,entity_handle:i64)),
            (check_vision_sensor_ex,"checkVisionSensorEx",(sensor_handle:i64,entity_handle:i64,return_image:bool)),
            (clear_double_signal,"clearDoubleSignal",(signal_name:String)),
            (clear_float_signal,"clearFloatSignal",(signal_name:String)),
            (clear_int32_signal,"clearInt32Signal",(signal_name:String)),
            (clear_string_signal,"clearStringSignal",(signal_name:String)),
            (combine_rgb_images,"combineRgbImages",(img1:Vec<u8>,img1_res:Vec<i64>,img2:Vec<u8>,img2_res:Vec<i64>,operation:i64)),
            (compute_mass_and_inertia,"computeMassAndInertia",(shape_handle:i64,density:f64)),
            (convex_decompose,"convexDecompose",(shape_handle:i64,options:i64,int_params:Vec<i64>,float_params:Vec<i64>)),
            (copy_table,"copyTable",(original:Vec<serde_json::Value>)),
            (create_dummy,"createDummy",(size:f64)),
            (create_force_sensor,"createForceSensor",(options:i64,int_params:Vec<i64>,float_params:Vec<f64>)),
            (create_heightfield_shape,"createHeightfieldShape",(options:i64,shading_angle:f64,x_point_count:i64,y_point_count:i64,x_size:f64,heights:Vec<f64>)),
            (create_mesh_shape,"createMeshShape",(options:i64,shading_angle:f64,vertices:Vec<f64>,indices:Vec<i64>)),
            (create_octree,"createOctree",(voxel_size:f64,options:i64,point_size:f64)),
            (create_point_cloud,"createPointCloud",(max_voxel_size:f64,max_pt_cnt_per_voxel:i64,options:i64,point_size:f64)),
            (create_proximity_sensor,"createProximitySensor",(sensor_type:i64,sub_type:i64,options:i64,int_params:Vec<i64>,float_params:Vec<f64>)),
            (create_vision_sensor,"createVisionSensor",(options:i64,int_params:Vec<i64>,float_params:Vec<f64>)),
            (destroy_collection,"destroyCollection",(collection_handle:i64)),
            (destroy_graph_curve,"destroyGraphCurve",(graph_handle:i64,curve_id:i64)),
            (execute_script_string,"executeScriptString",(string_at_script_name:i64,script_handle_or_type:i64)),
            (export_mesh,"exportMesh",(file_format:i64,path_and_filename:String,options:i64,scaling_factor:i64,vertices:Vec<f64>,indices:Vec<i64>)),
            (floating_view_add,"floatingViewAdd",(pos_x:f64,pos_y:f64,size_x:f64,size_y:f64,options:i64)),
            (floating_view_remove,"floatingViewRemove",(floating_view_handle:i64)),
            (get_api_func,"getApiFunc",(script_handle_or_type:i64,api_word:String)),
            (get_api_info,"getApiInfo",(script_handle_or_type:i64,api_word:String)),
            (get_array_param,"getArrayParam",(parameter:i64)),
            (get_bool_param,"getBoolParam",(parameter:i64)),
            (get_closest_pos_on_path,"getClosestPosOnPath",(path:Vec<f64>,path_lengths:Vec<f64>,abs_pt:Vec<f64>)),
            (get_collection_objects,"getCollectionObjects",(collection_handle:i64)),


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
