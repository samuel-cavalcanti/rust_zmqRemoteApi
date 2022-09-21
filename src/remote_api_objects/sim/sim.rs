use crate::remote_api_client::RemoteApiClientInterface;

use crate::remote_api_objects::cbor_arg_convert::CborArgConvert;
use crate::zmq_requests;
use crate::zmq_requests::RawRequest;
use serde_json::Value;
pub struct Sim<'a, R: RemoteApiClientInterface> {
    client: &'a R,
}

macro_rules! requests {


    ( $(
        ($rust_fn:ident,$function_name:literal $(, ( $($arg_id:ident : $arg_type:ty),+ )  )? $(, opt( $($opt_arg_id:ident : $opt_arg_type:ty),+   ) )?  $(->$return_type:ty)? )
    ),+ $(,)? ) => {
        $(
            pub fn $rust_fn(&self, $( $($arg_id:$arg_type,)*  )* $( $($opt_arg_id:Option<$opt_arg_type>,)*  )*   ) -> Result<$($return_type)*, zmq::Error>  {//

                let mut _brk = false;
                /*
                    CborArgConvert::from($arg_id).to_cbor()
                    converting the arg type properly.
                    ciborium::cbor!(value) is transforming Vec<u8> in an array of integers,
                 */
                let mut _args = vec![$($(CborArgConvert::from($arg_id).to_cbor()),*)* ]; //


                $(
                    $(
                        if let Some(option) = $opt_arg_id{
                            let option = ciborium::cbor!(option).unwrap();
                            if _brk{
                                panic!("no gaps allowed");
                            }
                            _args.push(option);

                        }
                        else{
                            _brk = true;
                        }
                    )*
                )*

                let request = zmq_requests::ZmqRequest {
                    function_name: format!("sim.{}",$function_name),
                    args: _args,
                };

                let result = self.client.send_raw_request(request.to_raw_request())?;

                if let Err(error) = Self::is_success(&result) {
                    panic!("error: {}", error)
                }

                let mut ret  =result["ret"].to_owned();
               $(

                if let Some(vec) = ret.as_array_mut() {
                    if vec.len() == 1{

                        log::trace!("vec: {:?} return type: {}", vec,stringify!($return_type));
                        let json_item = vec.remove(0);
                        let value:Result<$return_type, serde_json::Error> = serde_json::from_value(json_item);

                        match value {
                            Ok(value) => {
                                  // return a single value
                                    return Ok(value);
                            },
                            Err(e) => {
                                log::trace!("Err {}",e);

                                // Expected return ()
                                return Ok(Default::default());
                            },
                        }


                    }

                }

                let value:Result<$return_type, serde_json::Error> = serde_json::from_value(ret);

                match value {
                    Ok(value) => {
                          // Expected return tuple
                            Ok(value)
                    },           // Expected return ()
                    Err(_) => Ok(Default::default()),
                }

               )+

            }
        )*
    };

}

impl<'a, R: RemoteApiClientInterface + 'a> Sim<'a, R> {
    pub fn new(client: &'a R) -> Sim<'a, R> {
        Sim { client }
    }
    requests! {
    // DEPRECATED START
    (get_joint_max_force,"getJointMaxForce",(joint_handle:i64)->f64),
    (set_joint_max_force,"setJointMaxForce",(object_handle:i64,force_or_torque:f64)->()),
    (create_pure_shape,"createPureShape",(primitive_type:i64,options:i64,sizes:Vec<f64>,mass:f64),opt(precision:Vec<i64>)->i64),
    (remove_object,"removeObject",(object_handle:i64)->()),
    (get_vision_sensor_depth_buffer,"getVisionSensorDepthBuffer",(sensor_handle:i64),opt(pos:Vec<i64>,size:Vec<i64>)->(Vec<u8>,Vec<i64>)),
    (get_vision_sensor_char_image,"getVisionSensorCharImage",(sensor_handle:i64),opt(pos:Vec<i64>,size:Vec<i64>)->(Vec<u8>,Vec<i64>)),
    (set_vision_sensor_char_image,"setVisionSensorCharImage",(sensor_handle:i64,image:Vec<u8>)->()),
     // DEPRECATED END
    (add_drawing_object,"addDrawingObject",(object_type:i64,size:f64,duplicate_tolerance:f64,parent_object_handle:i64,max_item_count:i64),opt(ambient_diffuse:Vec<f64>,reserved:Vec<f64>,specular:Vec<f64>,emission:Vec<f64>)->i64),
    (add_drawing_object_item,"addDrawingObjectItem",(drawing_object_handle:i64,item_data:Vec<f64>)->i64),
    (add_force,"addForce",(shape_handle:i64,position:Vec<f64>,force:Vec<f64>)->()),
    (add_force_and_torque,"addForceAndTorque",(shape_handle:i64),opt(force:Vec<f64>,torque:Vec<f64>)->()),
    (add_graph_curve,"addGraphCurve",(graph_handle:i64,curve_name:String,dim:i64,stream_ids:Vec<i64>,default_values:Vec<f64>,unit_str:String),opt(options:i64,color:Vec<f64>,curve_width:i64)->i64),
    (add_graph_stream,"addGraphStream",(graph_handle:i64,stream_name:String,unit:String),opt(options:i64,color:Vec<f64>,cyclic_range:f64)->i64),
    (add_item_to_collection,"addItemToCollection",(collection_handle:i64,what:i64,object_handle:i64,options:i64)->()),
    (add_log,"addLog",(verbosity_level:i64,log_message:String)->()),
    (add_particle_object,"addParticleObject",(object_type:i64,size:f64,density:f64,params:Vec<f64>,life_time:f64,max_item_count:i64),opt(ambient_diffuse:Vec<f64>,reserved:Vec<f64>,specular:Vec<f64>,emission:Vec<f64>)->i64),
    (add_particle_object_item,"addParticleObjectItem",(particle_object_handle:i64,item_data:Vec<f64>)->()),
    (add_script,"addScript",(script_type:i64)->i64),
    (adjust_view,"adjustView",(view_handle_or_index:i64,associated_viewable_object_handle:i64,options:i64),opt(view_label:String)->i64),
    (alpha_beta_gamma_to_yaw_pitch_roll,"alphaBetaGammaToYawPitchRoll",(alpha_angle:f64,beta_angle:f64,gamma_angle:f64)->(f64,f64,f64)),
    (announce_scene_content_change,"announceSceneContentChange"->i64),
    (associate_script_with_object,"associateScriptWithObject",(script_handle:i64,object_handle:i64)->()),
    (auxiliary_console_close,"auxiliaryConsoleClose",(console_handle:i64)->i64),
    (auxiliary_console_open,"auxiliaryConsoleOpen",(title:String,max_lines:i64,mode:i64),opt(position:Vec<i64>,size:Vec<i64>,text_color:Vec<f64>,background_color:Vec<f64>)->i64),
    (auxiliary_console_print,"auxiliaryConsolePrint",(console_handle:i64,text:String)->i64),
    (auxiliary_console_show,"auxiliaryConsoleShow",(console_handle:i64,show_state:bool)->i64),
    (broadcast_msg,"broadcastMsg",(message:serde_json::Value),opt(options:i64)->()),
    (build_identity_matrix,"buildIdentityMatrix"->Vec<f64>),
    (build_matrix,"buildMatrix",(position:Vec<f64>,euler_angles:Vec<f64>)->Vec<f64>),
    (build_matrix_q,"buildMatrixQ",(position:Vec<f64>,quaternion:Vec<f64>)->Vec<f64>),
    (build_pose,"buildPose",(position:Vec<f64>,euler_angles_or_axis:Vec<f64>),opt(mode:i64,axis2:Vec<f64>)->Vec<f64>),
    (call_script_function,"callScriptFunction",(function_name:String,script_handle:i64),opt(in_arg:serde_json::Value)->serde_json::Value),
    (camera_fit_to_view,"cameraFitToView",(view_handle_or_index:i64),opt(object_handles:Vec<i64>,options:i64,scaling:f64)->i64),
    (change_entity_color,"changeEntityColor",(entity_handle:i64,new_color:Vec<f64>),opt(color_component:i64)->Vec<serde_json::Value>),
    (check_collision,"checkCollision",(entity1_handle:i64,entity2_handle:i64)->(i64,Vec<i64>)),
    (check_collision_ex,"checkCollisionEx",(entity1_handle:i64,entity2_handle:i64)->(i64,Vec<f64>)),
    (check_distance,"checkDistance",(entity1_handle:i64,entity2_handle:i64),opt(threshold:f64)->(i64,Vec<f64>,Vec<i64>)),
    (check_octree_point_occupancy,"checkOctreePointOccupancy",(octree_handle:i64,options:i64,points:Vec<f64>)->(i64,i64,i64,i64)),
    (check_proximity_sensor,"checkProximitySensor",(sensor_handle:i64,entity_handle:i64)->(i64,f64,Vec<f64>,i64,Vec<f64>)),
    (check_proximity_sensor_ex,"checkProximitySensorEx",(sensor_handle:i64,entity_handle:i64,mode:i64,threshold:f64,max_angle:f64)->(i64,f64,Vec<f64>,i64,Vec<f64>)),
    (check_proximity_sensor_ex2,"checkProximitySensorEx2",(sensor_handle:i64,vertices:Vec<f64>,item_type:i64,item_count:i64,mode:i64,threshold:f64,max_angle:f64)->(i64,f64,Vec<f64>,Vec<f64>)),
    (check_vision_sensor,"checkVisionSensor",(sensor_handle:i64,entity_handle:i64)->(i64,Vec<f64>,Vec<f64>)),
    (check_vision_sensor_ex,"checkVisionSensorEx",(sensor_handle:i64,entity_handle:i64,return_image:bool)->Vec<f64>),
    (clear_double_signal,"clearDoubleSignal",(signal_name:String)->()),
    (clear_float_signal,"clearFloatSignal",(signal_name:String)->()),
    (clear_int32_signal,"clearInt32Signal",(signal_name:String)->()),
    (clear_string_signal,"clearStringSignal",(signal_name:String)->()),
    (close_scene,"closeScene"->i64),
    (combine_rgb_images,"combineRgbImages",(img1:Vec<u8>,img1_res:Vec<i64>,img2:Vec<u8>,img2_res:Vec<i64>,operation:i64)->Vec<u8>),
    (compute_mass_and_inertia,"computeMassAndInertia",(shape_handle:i64,density:f64)->i64),
    (convex_decompose,"convexDecompose",(shape_handle:i64,options:i64,int_params:Vec<i64>,float_params:Vec<f64>)->i64),
    (copy_paste_objects,"copyPasteObjects",(object_handles:Vec<i64>),opt(options:i64)->Vec<i64>),
    (copy_table,"copyTable",(original:Vec<serde_json::Value>)->Vec<serde_json::Value>),
    (create_collection,"createCollection",opt(options:i64)->i64),
    (create_dummy,"createDummy",(size:f64)->i64),
    (create_force_sensor,"createForceSensor",(options:i64,int_params:Vec<i64>,float_params:Vec<f64>)->i64),
    (create_heightfield_shape,"createHeightfieldShape",(options:i64,shading_angle:f64,x_point_count:i64,y_point_count:i64,x_size:f64,heights:Vec<f64>)->i64),
    (create_joint,"createJoint",(joint_type:i64,joint_mode:i64,options:i64),opt(sizes:Vec<f64>)->i64),
    (create_mesh_shape,"createMeshShape",(options:i64,shading_angle:f64,vertices:Vec<f64>,indices:Vec<i64>)->i64),
    (create_octree,"createOctree",(voxel_size:f64,options:i64,point_size:f64)->i64),
    (create_path,"createPath",(ctrl_pts:Vec<f64>),opt(options:i64,subdiv:i64,smoothness:f64,orientation_mode:i64,up_vector:Vec<f64>)->i64),
    (create_point_cloud,"createPointCloud",(max_voxel_size:f64,max_pt_cnt_per_voxel:i64,options:i64,point_size:f64)->i64),
    (create_primitive_shape,"createPrimitiveShape",(primitive_type:i64,sizes:Vec<f64>),opt(options:i64)->i64),
    (create_proximity_sensor,"createProximitySensor",(sensor_type:i64,sub_type:i64,options:i64,int_params:Vec<i64>,float_params:Vec<f64>)->i64),
    (create_texture,"createTexture",(file_name:String,options:i64),opt(plane_sizes:Vec<f64>,scaling_uv:Vec<f64>,xy_g:Vec<f64>,fixed_resolution:i64,resolution:Vec<i64>)->(i64,i64,Vec<i64>)),
    (create_vision_sensor,"createVisionSensor",(options:i64,int_params:Vec<i64>,float_params:Vec<f64>)->i64),
    (destroy_collection,"destroyCollection",(collection_handle:i64)->()),
    (destroy_graph_curve,"destroyGraphCurve",(graph_handle:i64,curve_id:i64)->()),
    (duplicate_graph_curve_to_static,"duplicateGraphCurveToStatic",(graph_handle:i64,curve_id:i64),opt(curve_name:String)->i64),
    (execute_script_string,"executeScriptString",(string_at_script_name:String,script_handle_or_type:i64)->(i64,serde_json::Value)),
    (export_mesh,"exportMesh",(fileformat:i64,path_and_filename:String,options:i64,scaling_factor:f64,vertices:Vec<f64>,indices:Vec<i64>)->()),
    (floating_view_add,"floatingViewAdd",(pos_x:f64,pos_y:f64,size_x:f64,size_y:f64,options:i64)->i64),
    (floating_view_remove,"floatingViewRemove",(floating_view_handle:i64)->i64),
    (generate_shape_from_path,"generateShapeFromPath",(path:Vec<f64>,section:Vec<f64>),opt(options:i64,up_vector:Vec<f64>)->i64),
    (generate_text_shape,"generateTextShape",(txt:String),opt(color:Vec<f64>,height:f64,centered:bool,alphabet_location:String)->i64),
    (generate_time_optimal_trajectory,"generateTimeOptimalTrajectory",(path:Vec<f64>,path_lengths:Vec<f64>,min_max_vel:Vec<f64>,min_max_accel:Vec<f64>),opt(traj_pt_samples:i64,boundary_condition:String,timeout:f64)->(Vec<f64>,Vec<f64>)),
    (get_alternate_configs,"getAlternateConfigs",(joint_handles:Vec<i64>,input_config:Vec<f64>),opt(tip_handle:i64,low_limits:Vec<f64>,ranges:Vec<f64>)->Vec<f64>),
    (get_api_func,"getApiFunc",(script_handle_or_type:i64,api_word:String)->Vec<String>),
    (get_api_info,"getApiInfo",(script_handle_or_type:i64,api_word:String)->String),
    (get_array_param,"getArrayParam",(parameter:i64)->Vec<f64>),
    (get_bool_param,"getBoolParam",(parameter:i64)->bool),
    (get_closest_pos_on_path,"getClosestPosOnPath",(path:Vec<f64>,path_lengths:Vec<f64>,abs_pt:Vec<f64>)->f64),
    (get_collection_objects,"getCollectionObjects",(collection_handle:i64)->Vec<i64>),
    (get_config_distance,"getConfigDistance",(config_a:Vec<f64>,config_b:Vec<f64>),opt(metric:Vec<f64>,types:Vec<i64>)->f64),
    (get_contact_info,"getContactInfo",(dynamic_pass:i64,object_handle:i64,index:i64)->(Vec<i64>,Vec<f64>,Vec<f64>,Vec<f64>)),
    (get_decimated_mesh,"getDecimatedMesh",(vertices_in:Vec<f64>,indices_in:Vec<i64>,decimation_percentage:f64)->(Vec<f64>,Vec<i64>)),
    (get_double_signal,"getDoubleSignal",(signal_name:String)->f64),
    (get_engine_bool_param,"getEngineBoolParam",(param_id:i64,object_handle:i64)->bool),
    (get_engine_float_param,"getEngineFloatParam",(param_id:i64,object_handle:i64)->f64),
    (get_engine_int32_param,"getEngineInt32Param",(param_id:i64,object_handle:i64)->i64),
    (get_euler_angles_from_matrix,"getEulerAnglesFromMatrix",(matrix:Vec<f64>)->Vec<f64>),
    (get_explicit_handling,"getExplicitHandling",(object_handle:i64)->i64),
    (get_extension_string,"getExtensionString",(object_handle:i64,index:i64),opt(key:String)->String),
    (get_float_param,"getFloatParam",(parameter:i64)->f64),
    (get_float_signal,"getFloatSignal",(signal_name:String)->f64),
    (get_genesis_events,"getGenesisEvents"->Vec<serde_json::Value>),
    (get_graph_curve,"getGraphCurve",(graph_handle:i64,graph_type:i64,curve_index:i64)->(String,i64,Vec<f64>,Vec<f64>,Vec<f64>,Vec<f64>,i64,i64)),
    (get_graph_info,"getGraphInfo",(graph_handle:i64)->(i64,Vec<f64>,Vec<f64>,i64)),
    (get_int32_param,"getInt32Param",(parameter:i64)->i64),
    (get_int32_signal,"getInt32Signal",(signal_name:String)->i64),
    (get_joint_dependency,"getJointDependency",(joint_handle:i64)->(i64,f64,f64)),
    (get_joint_force,"getJointForce",(joint_handle:i64)->f64),
    (get_joint_interval,"getJointInterval",(object_handle:i64)->(bool,Vec<f64>)),
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
    (get_navigation_mode,"getNavigationMode"->i64),
    (get_object,"getObject",(path:String),opt(options:serde_json::Value)->i64),
    (get_object_alias,"getObjectAlias",(object_handle:i64),opt(options:i64)->String),
    (get_object_child,"getObjectChild",(object_handle:i64,index:i64)->i64),
    (get_object_child_pose,"getObjectChildPose",(object_handle:i64)->Vec<f64>),
    (get_object_color,"getObjectColor",(object_handle:i64,index:i64,color_component:i64)->Vec<f64>),
    (get_object_float_param,"getObjectFloatParam",(object_handle:i64,parameter_id:i64)->f64),
    (get_object_from_uid,"getObjectFromUid",(uid:i64),opt(options:serde_json::Value)->()),
    (get_object_int32_param,"getObjectInt32Param",(object_handle:i64,parameter_id:i64)->i64),
    (get_object_matrix,"getObjectMatrix",(object_handle:i64,relative_to_object_handle:i64)->Vec<f64>),
    (get_object_orientation,"getObjectOrientation",(object_handle:i64,relative_to_object_handle:i64)->Vec<f64>),
    (get_object_parent,"getObjectParent",(object_handle:i64)->i64),
    (get_object_pose,"getObjectPose",(object_handle:i64,relative_to_object_handle:i64)->Vec<f64>),
    (get_object_position,"getObjectPosition",(object_handle:i64,relative_to_object_handle:i64)->Vec<f64>),
    (get_object_property,"getObjectProperty",(object_handle:i64)->i64),
    (get_object_quaternion,"getObjectQuaternion",(object_handle:i64,relative_to_object_handle:i64)->Vec<f64>),
    (get_object_selection,"getObjectSelection"->Vec<i64>),
    (get_object_size_factor,"getObjectSizeFactor",(object_handle:i64)->f64),
    (get_object_special_property,"getObjectSpecialProperty",(object_handle:i64)->i64),
    (get_object_string_param,"getObjectStringParam",(object_handle:i64,parameter_id:i64)->Vec<u8>),
    (get_object_type,"getObjectType",(object_handle:i64)->i64),
    (get_object_uid,"getObjectUid",(object_handle:i64)->i64),
    (get_object_velocity,"getObjectVelocity",(object_handle:i64)->(Vec<f64>,Vec<f64>)),
    (get_objects,"getObjects",(index:i64,object_type:i64)->i64),
    (get_objects_in_tree,"getObjectsInTree",(tree_base_handle:i64),opt(object_type:i64,options:i64)->Vec<i64>),
    (get_octree_voxels,"getOctreeVoxels",(octree_handle:i64)->Vec<f64>),
    (get_page,"getPage"->i64),
    (get_path_interpolated_config,"getPathInterpolatedConfig",(path:Vec<f64>,path_lengths:Vec<f64>,t:f64),opt(method:serde_json::Value,types:Vec<i64>)->Vec<f64>),
    (get_path_lengths,"getPathLengths",(path:Vec<f64>,dof:i64),opt(dist_callback:String)->(Vec<f64>,f64)),
    (get_persistent_data_tags,"getPersistentDataTags"->Vec<String>),
    (get_point_cloud_options,"getPointCloudOptions",(point_cloud_handle:i64)->(f64,i64,i64,f64)),
    (get_point_cloud_points,"getPointCloudPoints",(point_cloud_handle:i64)->Vec<f64>),
    (get_q_hull,"getQHull",(vertices_in:Vec<f64>)->(Vec<f64>,Vec<i64>)),
    (get_quaternion_from_matrix,"getQuaternionFromMatrix",(matrix:Vec<f64>)->Vec<f64>),
    (get_random,"getRandom",opt(seed:i64)->f64),
    (get_real_time_simulation,"getRealTimeSimulation"->i64),
    (get_referenced_handles,"getReferencedHandles",(object_handle:i64)->Vec<i64>),
    (get_rotation_axis,"getRotationAxis",(matrix_start:Vec<f64>,matrix_goal:Vec<f64>)->(Vec<f64>,f64)),
    (get_scaled_image,"getScaledImage",(image_in:Vec<u8>,resolution_in:Vec<i64>,desired_resolution_out:Vec<i64>,options:i64)->(Vec<u8>,Vec<i64>)),
    (get_script,"getScript",(script_type:i64),opt(object_handle:i64,script_name:String)->i64),
    (get_script_int32_param,"getScriptInt32Param",(script_handle:i64,parameter_id:i64)->i64),
    (get_script_string_param,"getScriptStringParam",(script_handle:i64,parameter_id:i64)->Vec<u8>),
    (get_shape_bb,"getShapeBB",(shape_handle:i64)->Vec<f64>),
    (get_shape_color,"getShapeColor",(shape_handle:i64,color_name:String,color_component:i64)->(i64,Vec<f64>)),
    (get_shape_geom_info,"getShapeGeomInfo",(shape_handle:i64)->(i64,i64,Vec<f64>)),
    (get_shape_inertia,"getShapeInertia",(shape_handle:i64)->(Vec<f64>,Vec<f64>)),
    (get_shape_mass,"getShapeMass",(shape_handle:i64)->f64),
    (get_shape_mesh,"getShapeMesh",(shape_handle:i64)->(Vec<f64>,Vec<i64>,Vec<f64>)),
    (get_shape_texture_id,"getShapeTextureId",(shape_handle:i64)->i64),
    (get_shape_viz,"getShapeViz",(shape_handle:i64,item_index:i64)->serde_json::Value),
    (get_signal_name,"getSignalName",(signal_index:i64,signal_type:i64)->String),
    (get_simulation_state,"getSimulationState"->i64),
    (get_simulation_time,"getSimulationTime"->f64),
    (get_simulation_time_step,"getSimulationTimeStep"->f64),
    (get_simulator_message,"getSimulatorMessage"->(i64,Vec<i64>,Vec<i64>)),
    (get_stack_traceback,"getStackTraceback",opt(script_handle:i64)->String),
    (get_string_param,"getStringParam",(parameter:i64)->String),
    (get_string_signal,"getStringSignal",(signal_name:String)->String),// changed
    (get_system_time,"getSystemTime"->f64),
    (get_texture_id,"getTextureId",(texture_name:String)->(i64,Vec<i64>)),
    (get_thread_automatic_switch,"getThreadAutomaticSwitch"->bool),
    (get_thread_exist_request,"getThreadExistRequest"->bool),
    (get_thread_id,"getThreadId"->i64),
    (get_thread_switch_allowed,"getThreadSwitchAllowed"->bool),
    (get_thread_switch_timing,"getThreadSwitchTiming"->i64),
    (get_user_variables,"getUserVariables"->Vec<String>),
    (get_velocity,"getVelocity",(shape_handle:i64)->(Vec<f64>,Vec<f64>)),
    (get_vision_sensor_depth,"getVisionSensorDepth",(sensor_handle:i64),opt(options:i64,pos:Vec<i64>,size:Vec<i64>)->(Vec<u8>,Vec<i64>)),
    (get_vision_sensor_img,"getVisionSensorImg",(sensor_handle:i64),opt(options:i64,rgba_cut_off:f64,pos:Vec<i64>,size:Vec<i64>)->(Vec<u8>,Vec<i64>)),
    (group_shapes,"groupShapes",(shape_handles:Vec<i64>),opt(merge:bool)->i64),
    (handle_add_on_scripts,"handleAddOnScripts",(call_type:i64)->i64),
    (handle_child_scripts,"handleChildScripts",(call_type:i64)->i64),
    (handle_customization_scripts,"handleCustomizationScripts",(call_type:i64)->i64),
    (handle_dynamics,"handleDynamics",(delta_time:f64)->i64),
    (handle_graph,"handleGraph",(object_handle:i64,simulation_time:f64)->()),
    (handle_joint_motion,"handleJointMotion"->()),
    (handle_proximity_sensor,"handleProximitySensor",(sensor_handle:i64)->(i64,f64,Vec<f64>,i64,Vec<f64>)),
    (handle_sandbox_script,"handleSandboxScript",(call_type:i64)->()),
    (handle_sensing_start,"handleSensingStart"->()),
    (handle_simulation_start,"handleSimulationStart"->()),
    (handle_vision_sensor,"handleVisionSensor",(sensor_handle:i64)->(i64,Vec<f64>,Vec<f64>)),
    (import_mesh,"importMesh",(fileformat:i64,path_and_filename:String,options:i64,identical_vertice_tolerance:f64,scaling_factor:f64)->(Vec<f64>,Vec<i64>)),
    (import_shape,"importShape",(fileformat:i64,path_and_filename:String,options:i64,identical_vertice_tolerance:f64,scaling_factor:f64)->i64),
    (init_script,"initScript",(script_handle:i64)->bool),
    (insert_object_into_octree,"insertObjectIntoOctree",(octree_handle:i64,object_handle:i64,options:i64),opt(color:Vec<f64>,tag:i64)->i64),
    (insert_object_into_point_cloud,"insertObjectIntoPointCloud",(point_cloud_handle:i64,object_handle:i64,options:i64,grid_size:f64),opt(color:Vec<f64>,duplicate_tolerance:f64)->i64),
    (insert_points_into_point_cloud,"insertPointsIntoPointCloud",(point_cloud_handle:i64,options:i64,points:Vec<f64>),opt(color:Vec<f64>,duplicate_tolerance:f64)->i64),
    (insert_voxels_into_octree,"insertVoxelsIntoOctree",(octree_handle:i64,options:i64,points:Vec<f64>),opt(color:Vec<f64>,tag:Vec<i64>)->i64),
    (interpolate_matrices,"interpolateMatrices",(matrix_in1:Vec<f64>,matrix_in2:Vec<f64>,interpol_factor:f64)->Vec<f64>),
    (intersect_points_with_point_cloud,"intersectPointsWithPointCloud",(point_cloud_handle:i64,options:i64,points:Vec<f64>,tolerance:f64)->i64),
    (invert_matrix,"invertMatrix",(matrix:Vec<f64>)->()),
    (is_deprecated,"isDeprecated",(func_or_const:String)->i64),
    (is_dynamically_enabled,"isDynamicallyEnabled",(object_handle:i64)->bool),
    (is_handle,"isHandle",(object_handle:i64)->bool),
    (launch_executable,"launchExecutable",(filename:String),opt(parameters:String,show_status:i64)->()),
    (load_image,"loadImage",(options:i64,filename:String)->(Vec<u8>,Vec<i64>)),
    (load_model,"loadModel",(filename:String)->i64),
    (load_module,"loadModule",(filename_and_path:String,plugin_name:String)->i64),
    (load_scene,"loadScene",(filename:String)->()),
    (module_entry,"moduleEntry",(handle:i64),opt(label:String,state:i64)->i64),
    (move_to_config,"moveToConfig",(flags:i64,current_pos:Vec<f64>,current_vel:Vec<f64>,current_accel:Vec<f64>,max_vel:Vec<f64>,max_accel:Vec<f64>,max_jerk:Vec<f64>,target_pos:Vec<f64>,target_vel:Vec<f64>,callback:String),opt(aux_data:serde_json::Value,cyclic_joints:Vec<bool>,time_step:f64)->(Vec<f64>,Vec<f64>,Vec<f64>,f64)),
    (move_to_pose,"moveToPose",(flags:i64,current_pose:Vec<f64>,max_vel:Vec<f64>,max_accel:Vec<f64>,max_jerk:Vec<f64>,target_pose:Vec<f64>,callback:String),opt(aux_data:serde_json::Value,metric:Vec<f64>,time_step:f64)->(Vec<f64>,f64)),
    (multiply_matrices,"multiplyMatrices",(matrix_in1:Vec<f64>,matrix_in2:Vec<f64>)->Vec<f64>),
    (multiply_vector,"multiplyVector",(matrix:Vec<f64>,in_vectors:Vec<f64>)->Vec<f64>),
    (pack_double_table,"packDoubleTable",(double_numbers:Vec<f64>),opt(start_double_index:i64,double_count:i64)->Vec<u8>),
    (pack_float_table,"packFloatTable",(float_numbers:Vec<f64>),opt(start_float_index:i64,float_count:i64)->Vec<u8>),
    (pack_int32_table,"packInt32Table",(int32_numbers:Vec<i64>),opt(start_int32_index:i64,int32_count:i64)->Vec<u8>),
    (pack_table,"packTable",(a_table:Vec<serde_json::Value>),opt(scheme:i64)->Vec<u8>),
    (pack_u_int16_table,"packUInt16Table",(uint16_numbers:Vec<i64>),opt(start_uint16_index:i64,uint16_count:i64)->Vec<u8>),
    (pack_u_int32_table,"packUInt32Table",(uint32_numbers:Vec<i64>),opt(start_u_int32_index:i64,uint32_count:i64)->Vec<u8>),
    (pack_u_int8_table,"packUInt8Table",(uint8_numbers:Vec<i64>),opt(start_uint8_index:i64,uint8count:i64)->Vec<u8>),
    (pause_simulation,"pauseSimulation"->i64),
    (persistent_data_read,"persistentDataRead",(data_tag:String)->Vec<u8>),
    (persistent_data_write,"persistentDataWrite",(data_tag:String,data_value:Vec<u8>),opt(options:i64)->()),
    (push_user_event,"pushUserEvent",(event:String,handle:i64,uid:i64,event_data:serde_json::Value),opt(options:i64)->()),
    (quit_simulator,"quitSimulator"->()),
    (read_custom_data_block,"readCustomDataBlock",(object_handle:i64,tag_name:String)->Vec<u8>),
    (read_custom_data_block_tags,"readCustomDataBlockTags",(object_handle:i64)->Vec<String>),
    (read_custom_table_data,"readCustomTableData",(object_handle:i64,tag_name:String)->serde_json::Value),
    (read_force_sensor,"readForceSensor",(object_handle:i64)->(i64,Vec<f64>,Vec<f64>)),
    (read_proximity_sensor,"readProximitySensor",(sensor_handle:i64)->(i64,f64,Vec<f64>,i64,Vec<f64>)),
    (read_texture,"readTexture",(texture_id:i64,options:i64),opt(pos_x:i64,pos_y:i64,size_x:i64,size_y:i64)->Vec<u8>),
    (read_vision_sensor,"readVisionSensor",(sensor_handle:i64)->(i64,Vec<f64>,Vec<f64>)),
    (refresh_dialogs,"refreshDialogs",(refresh_degree:i64)->i64),
    (register_script_func_hook,"registerScriptFuncHook",(func_to_hook:String,user_func:String,exec_before:bool)->i64),
    (register_script_function,"registerScriptFunction",(func_name_at_plugin_name:String,call_tips:String)->i64),
    (register_script_variable,"registerScriptVariable",(var_name_at_plugin_name:String)->i64),
    (remove_drawing_object,"removeDrawingObject",(drawing_object_handle:i64)->()),
    (remove_model,"removeModel",(object_handle:i64)->i64),
    (remove_objects,"removeObjects",(object_handles:Vec<i64>)->()),
    (remove_particle_object,"removeParticleObject",(particle_object_handle:i64)->()),
    (remove_points_from_point_cloud,"removePointsFromPointCloud",(point_cloud_handle:i64,options:i64,points:Vec<f64>,tolerance:f64)->i64),
    (remove_script,"removeScript",(script_handle:i64)->()),
    (remove_voxels_from_octree,"removeVoxelsFromOctree",(octree_handle:i64,options:i64,points:Vec<f64>)->i64),
    (reorient_shape_bounding_box,"reorientShapeBoundingBox",(shape_handle:i64,relative_to_handle:i64)->i64),
    (resample_path,"resamplePath",(path:Vec<f64>,path_lengths:Vec<f64>,final_config_cnt:i64),opt(method:serde_json::Value,types:Vec<i64>)->Vec<f64>),
    (reset_dynamic_object,"resetDynamicObject",(object_handle:i64)->()),
    (reset_graph,"resetGraph",(object_handle:i64)->()),
    (reset_proximity_sensor,"resetProximitySensor",(object_handle:i64)->()),
    (reset_vision_sensor,"resetVisionSensor",(sensor_handle:i64)->()),
    (restore_entity_color,"restoreEntityColor",(original_color_data:Vec<serde_json::Value>)->()),
    (rotate_around_axis,"rotateAroundAxis",(matrix_in:Vec<f64>,axis:Vec<f64>,axis_pos:Vec<f64>,angle:f64)->Vec<f64>),
    (ruckig_pos,"ruckigPos",(dofs:i64,base_cycle_time:f64,flags:i64,current_pos_vel_accel:Vec<f64>,max_vel_accel_jerk:Vec<f64>,selection:Vec<i64>,target_pos_vel:Vec<f64>)->i64),
    (ruckig_remove,"ruckigRemove",(handle:i64)->()),
    (ruckig_step,"ruckigStep",(handle:i64,cycle_time:f64)->(i64,Vec<f64>,f64)),
    (ruckig_vel,"ruckigVel",(dofs:i64,base_cycle_time:f64,flags:i64,current_pos_vel_accel:Vec<f64>,max_accel_jerk:Vec<f64>,selection:Vec<i64>,target_vel:Vec<f64>)->i64),
    (save_image,"saveImage",(image:Vec<u8>,resolution:Vec<i64>,options:i64,filename:String,quality:i64)->Vec<u8>),
    (save_model,"saveModel",(model_base_handle:i64,filename:String)->()),
    (save_scene,"saveScene",(filename:String)->()),
    (scale_object,"scaleObject",(object_handle:i64,x_scale:f64,y_scale:f64,z_scale:f64),opt(options:i64)->()),
    (scale_objects,"scaleObjects",(object_handles:Vec<i64>,scaling_factor:f64,scale_positions_too:bool)->()),
    (serial_check,"serialCheck",(port_handle:i64)->i64),
    (serial_close,"serialClose",(port_handle:i64)->()),
    (serial_open,"serialOpen",(port_string:String,baudrate:i64)->i64),
    (serial_read,"serialRead",(port_handle:i64,data_length_to_read:i64,blocking_operation:bool),opt(closing_string:Vec<u8>,timeout:f64)->Vec<u8>),
    (serial_send,"serialSend",(port_handle:i64,data:Vec<u8>)->i64),
    (set_array_param,"setArrayParam",(parameter:i64,array_of_values:Vec<f64>)->()),
    (set_bool_param,"setBoolParam",(parameter:i64,bool_state:bool)->()),
    (set_double_signal,"setDoubleSignal",(signal_name:String,signal_value:f64)->()),
    (set_engine_bool_param,"setEngineBoolParam",(param_id:i64,object_handle:i64,bool_param:bool)->()),
    (set_engine_float_param,"setEngineFloatParam",(param_id:i64,object_handle:i64,float_param:f64)->()),
    (set_engine_int32_param,"setEngineInt32Param",(param_id:i64,object_handle:i64,int32_param:i64)->()),
    (set_explicit_handling,"setExplicitHandling",(object_handle:i64,explicit_handling_flags:i64)->()),
    (set_float_param,"setFloatParam",(parameter:i64,float_state:f64)->()),
    (set_float_signal,"setFloatSignal",(signal_name:String,signal_value:f64)->()),
    (set_graph_stream_transformation,"setGraphStreamTransformation",(graph_handle:i64,stream_id:i64,tr_type:i64),opt(mult:f64,off:f64,mov_avg_period:i64)->()),
    (set_graph_stream_value,"setGraphStreamValue",(graph_handle:i64,stream_id:i64,value:f64)->()),
    (set_int32_param,"setInt32Param",(parameter:i64,int_state:i64)->()),
    (set_int32_signal,"setInt32Signal",(signal_name:String,signal_value:i64)->()),
    (set_joint_dependency,"setJointDependency",(joint_handle:i64,master_joint_handle:i64,offset:f64,mult_coeff:f64)->()),
    (set_joint_interval,"setJointInterval",(object_handle:i64,cyclic:bool,interval:Vec<f64>)->()),
    (set_joint_mode,"setJointMode",(joint_handle:i64,joint_mode:i64,options:i64)->()),
    (set_joint_position,"setJointPosition",(object_handle:i64,position:f64)->()),
    (set_joint_target_force,"setJointTargetForce",(object_handle:i64,force_or_torque:f64),opt(signed_value:bool)->()),
    (set_joint_target_position,"setJointTargetPosition",(object_handle:i64,target_position:f64),opt(max_vel_accel_jerk:Vec<f64>)->()),
    (set_joint_target_velocity,"setJointTargetVelocity",(object_handle:i64,target_velocity:f64),opt(max_accel_jerk:Vec<f64>,init_velocity:f64)->()),
    (set_light_parameters,"setLightParameters",(light_handle:i64,state:i64,reserved:Vec<f64>,diffuse_part:Vec<f64>,specular_part:Vec<f64>)->()),
    (set_link_dummy,"setLinkDummy",(dummy_handle:i64,link_dummy_handle:i64)->()),
    (set_model_property,"setModelProperty",(object_handle:i64,property:i64)->()),
    (set_module_info,"setModuleInfo",(module_name:String,info_type:i64,info:String)->()),
    (set_named_string_param,"setNamedStringParam",(param_name:String,string_param:Vec<u8>)->()),
    (set_navigation_mode,"setNavigationMode",(navigation_mode:i64)->()),
    (set_object_alias,"setObjectAlias",(object_handle:i64,object_alias:String)->()),
    (set_object_child_pose,"setObjectChildPose",(object_handle:i64,pose:Vec<f64>)->()),
    (set_object_color,"setObjectColor",(object_handle:i64,index:i64,color_component:i64,rgb_data:Vec<f64>)->bool),
    (set_object_float_param,"setObjectFloatParam",(object_handle:i64,parameter_id:i64,parameter:f64)->()),
    (set_object_int32_param,"setObjectInt32Param",(object_handle:i64,parameter_id:i64,parameter:i64)->()),
    (set_object_matrix,"setObjectMatrix",(object_handle:i64,relative_to_object_handle:i64,matrix:Vec<f64>)->()),
    (set_object_orientation,"setObjectOrientation",(object_handle:i64,relative_to_object_handle:i64,euler_angles:Vec<f64>)->()),
    (set_object_parent,"setObjectParent",(object_handle:i64,parent_object_handle:i64),opt(keep_in_place:bool)->()),
    (set_object_pose,"setObjectPose",(object_handle:i64,relative_to_object_handle:i64,pose:Vec<f64>)->()),
    (set_object_position,"setObjectPosition",(object_handle:i64,relative_to_object_handle:i64,position:Vec<f64>)->()),
    (set_object_property,"setObjectProperty",(object_handle:i64,property:i64)->()),
    (set_object_quaternion,"setObjectQuaternion",(object_handle:i64,relative_to_object_handle:i64,quaternion:Vec<f64>)->()),
    (set_object_selection,"setObjectSelection",(object_handles:Vec<f64>)->()),
    (set_object_special_property,"setObjectSpecialProperty",(object_handle:i64,property:i64)->()),
    (set_object_string_param,"setObjectStringParam",(object_handle:i64,parameter_id:i64,parameter:Vec<u8>)->()),
    (set_page,"setPage",(page_index:i64)->()),
    (set_point_cloud_options,"setPointCloudOptions",(point_cloud_handle:i64,max_voxel_size:f64,max_pt_cnt_per_voxel:i64,options:i64,point_size:f64)->()),
    (set_referenced_handles,"setReferencedHandles",(object_handle:i64,referenced_handles:Vec<i64>)->()),
    (set_script_int32_param,"setScriptInt32Param",(script_handle:i64,parameter_id:i64,parameter:i64)->()),
    (set_script_string_param,"setScriptStringParam",(script_handle:i64,parameter_id:i64,parameter:Vec<u8>)->()),
    (set_shape_bb,"setShapeBB",(shape_handle:i64,size:Vec<f64>)->()),
    (set_shape_color,"setShapeColor",(shape_handle:i64,color_name:String,color_component:i64,rgb_data:Vec<f64>)->()),
    (set_shape_inertia,"setShapeInertia",(shape_handle:i64,inertia_matrix:Vec<f64>,transformation_matrix:Vec<f64>)->()),
    (set_shape_mass,"setShapeMass",(shape_handle:i64,mass:f64)->()),
    (set_shape_material,"setShapeMaterial",(shape_handle:i64,material_id_or_shape_handle:i64)->()),
    (set_shape_texture,"setShapeTexture",(shape_handle:i64,texture_id:i64,mapping_mode:i64,options:i64,uv_scaling:Vec<f64>),opt(position:Vec<f64>,orientation:Vec<f64>)->()),
    (set_string_param,"setStringParam",(parameter:i64,string_state:String)->()),
    (set_string_signal,"setStringSignal",(signal_name:String,signal_value:Vec<u8>)->()),
    (set_thread_automatic_switch,"setThreadAutomaticSwitch",(automatic_switch:bool)->i64),
    (set_thread_switch_allowed,"setThreadSwitchAllowed",(allowed:bool)->i64),
    (set_thread_switch_timing,"setThreadSwitchTiming",(dt_in_ms:i64)->()),
    (set_vision_sensor_img,"setVisionSensorImg",(sensor_handle:i64,image:Vec<u8>),opt(options:i64,pos:Vec<i64>,size:Vec<i64>)->()),
    (start_simulation,"startSimulation"->i64),
    (stop_simulation,"stopSimulation"->i64),
    (subtract_object_from_octree,"subtractObjectFromOctree",(octree_handle:i64,object_handle:i64,options:i64)->i64),
    (subtract_object_from_point_cloud,"subtractObjectFromPointCloud",(point_cloud_handle:i64,object_handle:i64,options:i64,tolerance:f64)->i64),
    (switch_thread,"switchThread"->()),
    (text_editor_close,"textEditorClose",(handle:i64)->(String,Vec<i64>,Vec<i64>)),
    (text_editor_get_info,"textEditorGetInfo",(handle:i64)->(String,Vec<i64>,Vec<i64>,bool)),
    (text_editor_open,"textEditorOpen",(init_text:String,properties:String)->i64),
    (text_editor_show,"textEditorShow",(handle:i64,show_state:bool)->()),
    (transform_buffer,"transformBuffer",(in_buffer:Vec<u8>,in_format:i64,multiplier:f64,offset:f64,out_format:i64)->Vec<u8>),
    (transform_image,"transformImage",(image:Vec<u8>,resolution:Vec<i64>,options:i64)->()),
    (ungroup_shape,"ungroupShape",(shape_handle:i64)->Vec<i64>),
    (unload_module,"unloadModule",(plugin_handle:i64)->i64),
    (unpack_double_table,"unpackDoubleTable",(data:Vec<u8>),opt(start_double_index:i64,double_count:i64,additional_byte_offset:i64)->Vec<f64>),
    (unpack_float_table,"unpackFloatTable",(data:Vec<u8>),opt(start_float_index:i64,float_count:i64,additional_byte_offset:i64)->Vec<f64>),
    (unpack_int32_table,"unpackInt32Table",(data:Vec<u8>),opt(start_int32_index:i64,int32_count:i64,additional_byte_offset:i64)->Vec<i64>),
    (unpack_table,"unpackTable",(buffer:Vec<u8>)->serde_json::Value),
    (unpack_u_int16_table,"unpackUInt16Table",(data:Vec<u8>),opt(start_uint16_index:i64,uint16_count:i64,additional_byte_offset:i64)->Vec<i64>),
    (unpack_u_int32_table,"unpackUInt32Table",(data:Vec<u8>),opt(start_uint32_index:i64,uint32_count:i64,additional_byte_offset:i64)->Vec<i64>),
    (unpack_u_int8_table,"unpackUInt8Table",(data:Vec<u8>),opt(start_uint8_index:i64,uint8count:i64)->Vec<i64>),
    (wait,"wait",(dt:f64),opt(simulation_time:bool)->f64),
    (wait_for_signal,"waitForSignal",(sig_name:String)->serde_json::Value),
    (write_custom_data_block,"writeCustomDataBlock",(object_handle:i64,tag_name:String,data:Vec<u8>)->()),
    (write_custom_table_data,"writeCustomTableData",(object_handle:i64,tag_name:String,data:Vec<serde_json::Value>)->()),
    (write_texture,"writeTexture",(texture_id:i64,options:i64,texture_data:Vec<u8>),opt(pos_x:i64,pos_y:i64,size_x:i64,size_y:i64,interpol:f64)->()),
    (yaw_pitch_roll_to_alpha_beta_gamma,"yawPitchRollToAlphaBetaGamma",(yaw_angle:f64,pitch_angle:f64,roll_angle:f64)->(f64,f64,f64))
    }

    fn is_success(json: &Value) -> Result<(), String> {
        if let Some(Value::Bool(success)) = json.get("success") {
            if *success {
                Ok(())
            } else {
                Err(Self::get_error(json))
            }
        } else {
            Err(Self::get_error(json))
        }
    }

    fn get_error(json: &Value) -> String {
        if let Some(Value::String(error)) = json.get("error") {
            error.clone()
        } else {
            "unknown error".to_string()
        }
    }

    // pub fn start_simulation(&self) -> Result<i64, zmq::Error> {
    //     let request = zmq_requests::ZmqRequest {
    //         function_name: format!("sim.startSimulation"),
    //         args: vec![],
    //     };

    //     let result = self.client.send_request(request)?;

    //     if let Err(error) = Self::is_success(&result) {
    //         panic!("error: {}", error)
    //     }

    //     let value = result["ret"].as_array().unwrap().to_owned().remove(0);

    //     Ok(serde_json::from_value(value).unwrap())
    // }

    // pub fn stop_simulation(&self) -> Result<i64, zmq::Error> {
    //     let request = zmq_requests::ZmqRequest {
    //         function_name: format!("sim.stopSimulation"),
    //         args: vec![],
    //     };

    //     let result = self.client.send_request(request)?;

    //     if let Err(error) = Self::is_success(&result) {
    //         panic!("error: {}", error)
    //     }

    //     let value = result["ret"].as_array().unwrap().to_owned().remove(0);

    //     Ok(serde_json::from_value(value).unwrap())
    // }

    // pub fn get_simulation_time(&self) -> Result<f64, zmq::Error> {
    //     let request = zmq_requests::ZmqRequest {
    //         function_name: format!("sim.getSimulationTime"),
    //         args: vec![],
    //     };

    //     let result = self.client.send_request(request)?;

    //     if let Err(error) = Self::is_success(&result) {
    //         panic!("error: {}", error)
    //     }
    //     let value = result["ret"].as_array().unwrap().to_owned().remove(0);

    //     Ok(serde_json::from_value(value).unwrap())
    // }
}