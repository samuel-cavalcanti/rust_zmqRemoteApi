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


    ( $(
        ($rust_fn:ident,$function_name:literal $(, ( $($arg_id:ident : $arg_type:ty),+ )  )? $(, opt( $($opt_arg_id:ident : $opt_arg_type:ty),+   ) )?  $(->$return_type:ty)? )
    ),+ $(,)? ) => { 
        $(
            pub fn $rust_fn( $( $($arg_id:$arg_type,)*  )* $( $($opt_arg_id:Option<$opt_arg_type>,)*  )*   ) -> ZmqRequest {//

                let mut _brk = false;
                let mut args = vec![$($(cbor!($arg_id).unwrap(),)*)*];

                

                $(
                    $(
                        if let Some(option) = $opt_arg_id{
                            let option = cbor!(option).unwrap();
                            if _brk{
                                panic!("no gaps allowed");
                            }
                            args.push(option);

                        }
                        else{
                            _brk = true;
                        }
                    )*
                )*



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

                /*
                    DEPRECATED START
                */
                (get_joint_max_force,"get_joint_max_force", (joint_handle:i64)->f64),
                (set_joint_max_force,"setJointMaxForce", (joint_handle:i64,force_or_torque:f64)),
                (create_pure_shape,"createPureShape", (primitive_type:i64,options:i64,size:Vec<f64>,mass:f64), opt(precision:Vec<i64>) -> i64 ),
                (remove_object,"removeObject", (object_handle:i64)),
                (get_vision_sensor_depth_buffer,"getVisionSensorDepthBuffer",(sensor_handle:i64),opt(pos:Vec<i64>,size:Vec<i64>)->(Vec<u8>,Vec<i64>)),
                (get_vision_sensor_char_image,"getVisionSensorCharImage",(sensor_handle:i64),opt(pos:Vec<i64>,size:Vec<i64>)->(Vec<u8>,Vec<i64>)),
                (set_vision_sensor_char_image,"setVisionSensorCharImage", (sensor_handle:i64,image:Vec<u8>)),
                /*
                    DEPRECATED END
                */
                (add_drawing_object, "addDrawingObject", (object_type:i64,size:f64,duplicate_tolerance:f64,parent_object_handle:i64,max_item_count:i64),opt(ambient_diffuse:Vec<f64>,reserved:Vec<f64>,specular:Vec<f64>,emission:Vec<f64>)-> i64 ),
                (add_drawing_object_item,"addDrawingObjectItem",(drawing_object_handle:i64,item_data:Vec<f64>)),
                (add_force,"addForce",(shape_handle:i64,position:Vec<f64>,force:Vec<f64>)),
                (add_force_and_torque,"addForceAndTorque",(shape_handle:i64),opt(force:Vec<f64>,torque:Vec<f64>)),
                (add_graph_curve,"addGraphCurve",(graph_handle:i64,curve_name:String,dim:i64,stream_ids:Vec<i64>,default_values:Vec<i64>,unit_str:String),opt(options:i64,color:Vec<f64>,curve_width:i64)->i64),
                (add_graph_stream,"addGraphStream",(graph_handle:i64,stream_name:String,unit:String),opt(options:i64,color:Vec<f64>,cyclic_range:f64)->i64),
                (add_item_to_collection,"addItemToCollection",(collection_handle:i64,what:i64,object_handle:i64,options:i64)),
                (add_log,"addLog",(verbosity_level:i64,log_message:String)),
                (add_particle_object, "addParticleObject", (object_type:i64,size:f64,density:f64,params:Vec<f64>,life_time:f64,max_item_count:i64),opt(ambient_diffuse:Vec<f64>,reserved:Vec<f64>,specular:Vec<f64>,emission:Vec<f64>)-> i64 ),
                (add_particle_object_item,"addParticleObjectItem",(particle_object_handle:i64,item_data:Vec<f64>)),
                (add_script,"addScript",(add_script:i64)->i64),
                (adjust_view,"adjustView",(view_handle_or_index:i64,associated_viewable_object_handle:i64,options:i64), opt(view_label:String) ->i64),
                (alpha_beta_gamma_to_yaw_pitch_roll,"alphaBetaGammaToYawPitchRoll",(alpha:f64, beta:f64,gamma:f64)->(f64,f64,f64)),
                (announce_scene_content_change,"announceSceneContentChange"->i64),
                (associate_script_with_object,"associateScriptWithObject",(script_handle:i64, object_handle:i64)),
                (auxiliary_console_close,"auxiliaryConsoleClose",(console_handle:i64)->i64),
                (auxiliary_console_open,"auxiliaryConsoleOpen",(title:String,max_lines:i64,mode:i64),opt(position:Vec<i64>,size:Vec<i64>,text_color:Vec<f64>,background_color:Vec<f64>) ->i64),
                (auxiliary_console_print,"auxiliaryConsolePrint",(console_handle:i64,text:String)->i64),
                (auxiliary_console_show,"auxiliaryConsoleShow",(console_handle:i64,show_state:bool)->i64),
                (broadcast_msg,"broadcastMsg",(message:serde_json::Value),opt(options:i64)),
                (build_identity_matrix,"buildIdentityMatrix" -> Vec<f64>),
                (build_matrix,"buildMatrix",(position:Vec<f64>,euler_angles:Vec<f64>)->Vec<f64>),
                (build_pose,"buildMatrixQ",(position:Vec<f64>,quaternion:Vec<f64>)->Vec<f64>),
                (build_matrix_q,"buildPose",(position:Vec<f64>,euler_angles_or_axis:Vec<f64>), opt(mode:i64,axis_2:Vec<f64>) ->Vec<f64>),
                (call_script_function,"callScriptFunction",(function_name:String,script_handle:i64),opt(in_arg:serde_json::Value)->serde_json::Value),
                (camera_fit_to_view,"cameraFitToView",(view_handle_or_index:i64),opt(object_handles:Vec<i64>,options:Vec<i64>,scaling:f64)->i64),
                (change_entity_color,"changeEntityColor",(entity_handle:i64,new_color:Vec<f64>),opt(color_component:i64)->Vec<serde_json::Value>),
                (check_collision,"checkCollision",(entity1_handle:i64,entity2_handle:i64)->(i64,Vec<i64>)),
                (check_collision_ex,"checkCollisionEx",(entity1_handle:i64,entity2_handle:i64)->(i64,Vec<f64>)),
                (check_distance,"checkDistance",(entity1_handle:i64,entity2_handle:i64),opt(threshold:f64) ->(i64,Vec<f64>,Vec<i64>)),
                (check_octree_point_occupancy,"checkOctreePointOccupancy",(octree_handle:i64,options:i64,points:Vec<f64>)->(i64,i64,i64,i64)),
                (check_proximity_sensor,"checkProximitySensor",(sensor_handle:i64,entity_handle:i64)->(i64,f64,Vec<f64>,i64,Vec<f64>)),
                (check_proximity_sensor_ex,"checkProximitySensorEx",(sensor_handle:i64,entity_handle:i64,mode:i64,threshold:f64,max_angle:f64)->(i64,f64,Vec<f64>,i64,Vec<f64>)),
                (check_proximity_sensor_ex2,"checkProximitySensorEx2",(sensor_handle:i64,vertices:Vec<f64>,item_type:i64,item_count:i64,mode:i64,threshold:f64,max_angle:f64)->(i64,f64,Vec<f64>,Vec<f64>)),
                (check_vision_sensor,"checkVisionSensor",(sensor_handle:i64,entity_handle:i64)->(i64,Vec<f64>,Vec<f64>)),
                (check_vision_sensor_ex,"checkVisionSensorEx",(sensor_handle:i64,entity_handle:i64,return_image:bool)->Vec<f64>),
                (clear_double_signal,"clearDoubleSignal",(signal_name:String)),
                (clear_float_signal,"clearFloatSignal",(signal_name:String)),
                (clear_int32_signal,"clearInt32Signal",(signal_name:String)),
                (clear_string_signal,"clearStringSignal",(signal_name:String)),
                (close_scene,"closeScene"->i64),
                (combine_rgb_images,"combineRgbImages",(img1:Vec<u8>,img1_res:Vec<i64>,img2:Vec<u8>,img2_res:Vec<i64>,operation:i64)->Vec<u8>),
                (compute_mass_and_inertia,"computeMassAndInertia",(shape_handle:i64,density:f64)->i64),
                (convex_decompose,"convexDecompose",(shape_handle:i64,options:i64,int_params:Vec<i64>,float_params:Vec<i64>)->i64),
                (copy_paste_objects,"copyPasteObjects", (object_handles:Vec<i64>), opt(options:i64) ->Vec<i64>),
                (copy_table,"copyTable",(original:Vec<serde_json::Value>)->Vec<serde_json::Value>),
                (create_collection,"createCollection",opt(options:i64)->i64),
                (create_dummy,"createDummy",(size:f64)->i64),
                (create_force_sensor,"createForceSensor",(options:i64,int_params:Vec<i64>,float_params:Vec<f64>)->i64),
                (create_heightfield_shape,"createHeightfieldShape",(options:i64,shading_angle:f64,x_point_count:i64,y_point_count:i64,x_size:f64,heights:Vec<f64>)->i64),
                (create_joint,"createJoint",(joint_type:i64,joint_mode:i64,options:i64),opt(sizes:Vec<f64>)->i64),
                (create_mesh_shape,"createMeshShape",(options:i64,shading_angle:f64,vertices:Vec<f64>,indices:Vec<i64>)->i64),
                (create_octree,"createOctree",(voxel_size:f64,options:i64,point_size:f64)->i64),
                (create_path,"createPath",(ctrl_pts:Vec<f64>),opt(subdiv:i64,smoothness:f64,orientation_mode:i64,up_vector:Vec<f64>)->i64),
                (create_point_cloud,"createPointCloud",(max_voxel_size:f64,max_pt_cnt_per_voxel:i64,options:i64,point_size:f64)->i64),
                (create_primitive_shape,"createPrimitiveShape",(primitive_type:i64,sizes:Vec<f64>),opt(options:i64)->i64),
                (create_proximity_sensor,"createProximitySensor",(sensor_type:i64,sub_type:i64,options:i64,int_params:Vec<i64>,float_params:Vec<f64>)->i64),
                (create_vision_sensor,"createVisionSensor",(options:i64,int_params:Vec<i64>,float_params:Vec<f64>)->i64),
                (destroy_collection,"destroyCollection",(collection_handle:i64)),
                (destroy_graph_curve,"destroyGraphCurve",(graph_handle:i64,curve_id:i64)),
                (duplicate_graph_curve_to_static,"duplicateGraphCurveToStatic",(graph_handle:i64,curve_id:i64),opt(curve_name:String) ->i64),
                (execute_script_string,"executeScriptString",(string_at_script_name:String,script_handle_or_type:i64)->(i64,serde_json::Value)),
                (export_mesh,"exportMesh",(file_format:i64,path_and_filename:String,options:i64,scaling_factor:i64,vertices:Vec<f64>,indices:Vec<i64>)),
                (floating_view_add,"floatingViewAdd",(pos_x:f64,pos_y:f64,size_x:f64,size_y:f64,options:i64)->i64),
                (floating_view_remove,"floatingViewRemove",(floating_view_handle:i64)->i64),
                (generate_shape_from_path,"generateShapeFromPath", (path:Vec<f64>,section:Vec<f64>), opt(options:i64,up_vector:Vec<f64>) ->i64),
                (generate_text_shape,"generateTextShape",(txt:String), opt(color:Vec<f64>,height:f64,centered:bool,alphabet_location:String)->i64),
                (generate_time_optimal_trajectory,"generateTimeOptimalTrajectory",(path:Vec<f64>,path_lengths:Vec<f64>,min_max_vel:Vec<f64>,min_max_cel:Vec<f64>),opt(traj_pt_samples:i64,boundary_condition:String,timeout:f64)->(Vec<f64>,Vec<f64>)),
                (get_api_func,"getApiFunc",(script_handle_or_type:i64,api_word:String)->Vec<String>),
                (get_api_info,"getApiInfo",(script_handle_or_type:i64,api_word:String)->String),
                (get_array_param,"getArrayParam",(parameter:i64)->Vec<f64>),
                (get_bool_param,"getBoolParam",(parameter:i64)->bool),
                (get_closest_pos_on_path,"getClosestPosOnPath",(path:Vec<f64>,path_lengths:Vec<f64>,abs_pt:Vec<f64>)->f64),
                (get_collection_objects,"getCollectionObjects",(collection_handle:i64)->Vec<i64>),
                (get_config_distance,"getConfigDistance",(config_a:Vec<f64>,config_b:Vec<f64>),opt(metric:Vec<f64>,types:Vec<f64>)->f64),
                (get_contact_info,"getContactInfo",(dynamic_pass:i64,object_handle:i64,index:i64)->(Vec<i64>,Vec<f64>,Vec<f64>)),
                (get_decimated_mesh,"getDecimatedMesh",(vertices_in:Vec<f64>,indices_in:Vec<i64>,decimation_percentage:f64)->(Vec<f64>,Vec<i64>)),
                (get_double_signal,"getDoubleSignal",(signal_name:String)->f64),
                (get_engine_bool_param,"getEngineBoolParam",(param_id:i64,object_handle:i64)->bool),
                (get_engine_float_param,"getEngineFloatParam",(param_id:i64,object_handle:i64)->f64),
                (get_engine_int32_param,"getEngineInt32Param",(param_id:i64,object_handle:i64)->i64),
                (get_euler_angles_from_matrix,"getEulerAnglesFromMatrix",(matrix:Vec<f64>)->Vec<f64>),
                (get_explicit_handling,"getExplicitHandling",(object_handle:i64)->i64),
                (get_extension_string,"getExtensionString",(object_handle:i64,index:i64),opt(key:String)->i64),
                (get_float_param,"getFloatParam",(parameter:i64)->f64),
                (get_float_signal,"getFloatSignal",(signal_name:String)->f64),
                (get_genesis_events,"getGenesisEvents"->Vec<serde_json::Value>),
                (get_graph_curve,"getGraphCurve",(graph_handle:i64,graph_type:i64,curve_index:i64)->(String, i64, Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>, i64, i64)),
                (get_graph_info,"getGraphInfo",(graph_handle:i64)->(i64, Vec<f64>, Vec<f64>, i64)),
                (get_int32_param,"getInt32Param",(parameter:i64)->i64),
                (get_int32_signal,"getInt32Signal",(signal_name:i64)->i64),
                (get_joint_dependency,"getJointDependency",(joint_handle:i64)->(i64,f64,f64)),
                (get_joint_mode,"getJointMode",(joint_handle:i64)->(i64,i64)),
                (get_joint_position,"getJointPosition",(object_handle:i64)->f64),
                (get_joint_target_force,"getJointTargetForce",(joint_handle:i64)->f64),
                (get_joint_target_position,"getJointTargetPosition",(object_handle:i64)->f64),
                (get_joint_target_velocity,"getJointTargetVelocity",(object_handle:i64)->f64),
                (get_joint_type,"getJointType",(object_handle:i64)->i64),
                (get_joint_velocity,"getJointVelocity",(joint_handle:i64)->f64),
                (get_light_parameters,"getLightParameters",(light_handle:i64)->(i64,Vec<f64>,Vec<f64>,Vec<f64>)),
                (get_link_dummy,"getLinkDummy",(dummy_handle:i64)->i64),
                (get_matching_persistent_data_tags,"getMatchingPersistentDataTags",(pattern:String)->Vec<String>),
                (get_model_property,"getModelProperty",(object_handle:i64)->i64),
                (get_module_info,"getModuleInfo",(module_name:String,info_type:i64)->String),
                (get_module_name,"getModuleName",(index:i64)->(String,i64)),
                (get_named_string_param,"getNamedStringParam",(param_name:String)->Vec<u8>),
                (get_navigation_mode,"getNavigationMode" -> i64),
                (get_object,"getObject",(path:String),opt(options:serde_json::Value)->i64),
                (get_object_alias,"getObjectAlias",(object_handle:i64),opt(options:i64)->String),
                (get_object_child,"getObjectChild",(object_handle:i64,index:i64)->i64),
                (start_simulation,"startSimulation" ->i64),
                (stop_simulation,"stopSimulation" ->i64),
                (get_object_child_pose,"getObjectChildPose",(object_handle:i64)->Vec<f64>),
                (get_object_color,"getObjectColor",(object_handle:i64,index:i64,color_component:i64)->Vec<f64>),
                (get_object_float_param,"getObjectFloatParam",(object_handle:i64,parameter_id:i64)->f64),
                (get_object_from_uid,"getObjectFromUid",(uid:i64),opt(options:serde_json::Value)),
                (get_object_int32_param,"getObjectInt32Param",(object_handle:i64,parameter_id:i64)->i64),
                // (switch_thread,"switchThread"),
                // (quit_simulator,"quitSimulator"),
                // (pause_simulation,"pauseSimulation" ->i64),
                // (handle_joint_motion,"handleJointMotion"),
                // (handle_simulation_start,"handleSimulationStart"),
                // (handle_sensing_start,"handleSensingStart"),
                // (get_user_variables,"getUserVariables"),
                // (get_thread_switch_timing, "getThreadSwitchTiming"),
                // (get_thread_switch_allowed,"getThreadSwitchAllowed"),
                // (get_thread_id,"getThreadId"),
                // (get_thread_exist_request,"getThreadExistRequest"),
                // (get_thread_automatic_switch,"getThreadAutomaticSwitch"),
                // (get_system_time,"getSystemTime"),
                // (get_simulation_time_step,"getSimulationTimeStep"),
                (get_simulation_time,"getSimulationTime"),
                // (get_simulation_state,"getSimulationState"),
                // (get_simulator_message,"getSimulatorMessage"),
                // (get_real_time_simulation,"getRealTimeSimulation"),
                // (get_persistent_data_tags,"getPersistentDataTags"),
                // (get_page,"getPage"),
                // (get_object_selection,"getObjectSelection"),
                
                //
                
                
               
                // (create_environment,"createEnvironment"),







                
              
    
                
                
                
                
                
                
                
               
                // (set_joint_target_force,"setJointTargetForce",(object_handle:i64,force_or_torque:f64),opt(signed_value:bool,test:f64)),

    /*
            (setJointTargetForce,"setJointTargetForce",(objectHandle:i64,forceOrTorque:f64),opt(signedValue:bool),
    */


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
