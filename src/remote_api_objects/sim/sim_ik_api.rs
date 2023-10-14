use crate::RemoteApiClientInterface;
pub trait SimIK : RemoteApiClientInterface {
    requests!{
"simIK",
(sim_ik_add_element,"addElement",(environment_handle:i64,ik_group_handle:i64,tip_dummy_handle:i64)->i64),
(sim_ik_add_element_from_scene,"addElementFromScene",(environment_handle:i64,ik_group:i64,base_handle:i64,tip_handle:i64,target_handle:i64,constraints:i64)->(i64,serde_json::Value,serde_json::Value)),
(sim_ik_compute_group_jacobian,"computeGroupJacobian",(environment_handle:i64,ik_group_handle:i64)->(Vec<f64>,Vec<f64>)),
(sim_ik_compute_jacobian,"computeJacobian",(environment_handle:i64,base_object:i64,last_joint:i64,constraints:i64,tip_matrix:Vec<f64>),opt(target_matrix:Vec<f64>,constr_base_matrix:Vec<f64>)->(Vec<f64>,Vec<f64>)),
(sim_ik_create_debug_overlay,"createDebugOverlay",(environment_handle:i64,tip_handle:i64),opt(base_handle:i64)->i64),
(sim_ik_create_dummy,"createDummy",(environment_handle:i64),opt(dummy_name:String)->i64),
(sim_ik_create_environment,"createEnvironment",opt(flags:i64)->i64),
(sim_ik_create_group,"createGroup",(environment_handle:i64),opt(ik_group_name:String)->i64),
(sim_ik_create_joint,"createJoint",(environment_handle:i64,joint_type:i64),opt(joint_name:String)->i64),
(sim_ik_does_group_exist,"doesGroupExist",(environment_handle:i64,ik_group_name:String)->bool),
(sim_ik_does_object_exist,"doesObjectExist",(environment_handle:i64,object_name:String)->bool),
(sim_ik_duplicate_environment,"duplicateEnvironment",(environment_handle:i64)->i64),
(sim_ik_erase_debug_overlay,"eraseDebugOverlay",(debug_object:i64)->()),
(sim_ik_erase_environment,"eraseEnvironment",(environment_handle:i64)->()),
(sim_ik_erase_object,"eraseObject",(environment_handle:i64,object_handle:i64)->()),
(sim_ik_find_config,"findConfig",(environment_handle:i64,ik_group_handle:i64,joint_handles:Vec<i64>),opt(threshold_dist:f64,max_time:f64,metric:Vec<f64>,validation_callback:String,aux_data:serde_json::Value)->Vec<f64>),
(sim_ik_generate_path,"generatePath",(environment_handle:i64,ik_group_handle:i64,joint_handles:Vec<i64>,tip_handle:i64,path_point_count:i64),opt(validation_callback:String,aux_data:serde_json::Value)->Vec<f64>),
(sim_ik_get_alternate_configs,"getAlternateConfigs",(environment_handle:i64,joint_handles:Vec<i64>),opt(low_limits:Vec<f64>,ranges:Vec<f64>)->Vec<f64>),
(sim_ik_get_element_base,"getElementBase",(environment_handle:i64,ik_group_handle:i64,element_handle:i64)->(i64,i64)),
(sim_ik_get_element_constraints,"getElementConstraints",(environment_handle:i64,ik_group_handle:i64,element_handle:i64)->i64),
(sim_ik_get_element_flags,"getElementFlags",(environment_handle:i64,ik_group_handle:i64,element_handle:i64)->i64),
(sim_ik_get_element_precision,"getElementPrecision",(environment_handle:i64,ik_group_handle:i64,element_handle:i64)->Vec<f64>),
(sim_ik_get_element_weights,"getElementWeights",(environment_handle:i64,ik_group_handle:i64,element_handle:i64)->Vec<f64>),
(sim_ik_get_failure_description,"getFailureDescription",(reason:i64)->String),
(sim_ik_get_group_calculation,"getGroupCalculation",(environment_handle:i64,ik_group_handle:i64)->(i64,f64,i64)),
(sim_ik_get_group_flags,"getGroupFlags",(environment_handle:i64,ik_group_handle:i64)->i64),
(sim_ik_get_group_handle,"getGroupHandle",(environment_handle:i64,ik_group_name:String)->i64),
(sim_ik_get_group_joint_limit_hits,"getGroupJointLimitHits",(environment_handle:i64,ik_group_handle:i64)->(Vec<i64>,Vec<f64>)),
(sim_ik_get_group_joints,"getGroupJoints",(environment_handle:i64,ik_group_handle:i64)->Vec<i64>),
(sim_ik_get_joint_dependency,"getJointDependency",(environment_handle:i64,joint_handle:i64)->(i64,f64,f64)),
(sim_ik_get_joint_interval,"getJointInterval",(environment_handle:i64,joint_handle:i64)->(bool,Vec<f64>)),
(sim_ik_get_joint_limit_margin,"getJointLimitMargin",(environment_handle:i64,joint_handle:i64)->f64),
(sim_ik_get_joint_matrix,"getJointMatrix",(environment_handle:i64,joint_handle:i64)->Vec<f64>),
(sim_ik_get_joint_max_step_size,"getJointMaxStepSize",(environment_handle:i64,joint_handle:i64)->f64),
(sim_ik_get_joint_mode,"getJointMode",(environment_handle:i64,joint_handle:i64)->i64),
(sim_ik_get_joint_position,"getJointPosition",(environment_handle:i64,joint_handle:i64)->f64),
(sim_ik_get_joint_screw_lead,"getJointScrewLead",(environment_handle:i64,joint_handle:i64)->f64),
(sim_ik_get_joint_transformation,"getJointTransformation",(environment_handle:i64,joint_handle:i64)->(Vec<f64>,Vec<f64>,Vec<f64>)),
(sim_ik_get_joint_type,"getJointType",(environment_handle:i64,joint_handle:i64)->i64),
(sim_ik_get_joint_weight,"getJointWeight",(environment_handle:i64,joint_handle:i64)->f64),
(sim_ik_get_object_handle,"getObjectHandle",(environment_handle:i64,object_name:String)->i64),
(sim_ik_get_object_matrix,"getObjectMatrix",(environment_handle:i64,object_handle:i64),opt(relative_to_object_handle:i64)->Vec<f64>),
(sim_ik_get_object_parent,"getObjectParent",(environment_handle:i64,object_handle:i64)->i64),
(sim_ik_get_object_pose,"getObjectPose",(environment_handle:i64,object_handle:i64),opt(relative_to_object_handle:i64)->Vec<f64>),
(sim_ik_get_object_transformation,"getObjectTransformation",(environment_handle:i64,object_handle:i64),opt(relative_to_object_handle:i64)->(Vec<f64>,Vec<f64>,Vec<f64>)),
(sim_ik_get_object_type,"getObjectType",(environment_handle:i64,object_handle:i64)->i64),
(sim_ik_get_objects,"getObjects",(environment_handle:i64,index:i64)->(i64,String,bool,i64)),
(sim_ik_get_target_dummy,"getTargetDummy",(environment_handle:i64,dummy_handle:i64)->i64),
(sim_ik_handle_group,"handleGroup",(environment_handle:i64,ik_group:i64),opt(options:serde_json::Value)->(i64,i64,Vec<f64>)),
(sim_ik_handle_groups,"handleGroups",(environment_handle:i64,ik_groups:Vec<i64>),opt(options:serde_json::Value)->(i64,i64,Vec<f64>)),
(sim_ik_load,"load",(environment_handle:i64,data:String)->()),
(sim_ik_save,"save",(environment_handle:i64)->String),
(sim_ik_set_element_base,"setElementBase",(environment_handle:i64,ik_group_handle:i64,element_handle:i64,base_handle:i64),opt(constraints_base_handle:i64)->()),
(sim_ik_set_element_constraints,"setElementConstraints",(environment_handle:i64,ik_group_handle:i64,element_handle:i64,constraints:i64)->()),
(sim_ik_set_element_flags,"setElementFlags",(environment_handle:i64,ik_group_handle:i64,element_handle:i64,flags:i64)->()),
(sim_ik_set_element_precision,"setElementPrecision",(environment_handle:i64,ik_group_handle:i64,element_handle:i64,precision:Vec<f64>)->()),
(sim_ik_set_element_weights,"setElementWeights",(environment_handle:i64,ik_group_handle:i64,element_handle:i64,weights:Vec<f64>)->()),
(sim_ik_set_group_calculation,"setGroupCalculation",(environment_handle:i64,ik_group_handle:i64,method:i64,damping:f64,max_iterations:i64)->()),
(sim_ik_set_group_flags,"setGroupFlags",(environment_handle:i64,ik_group_handle:i64,flags:i64)->()),
(sim_ik_set_joint_dependency,"setJointDependency",(environment_handle:i64,joint_handle:i64,master_joint_handle:i64),opt(offset:f64,mult:f64,callback:String)->()),
(sim_ik_set_joint_interval,"setJointInterval",(environment_handle:i64,joint_handle:i64,cyclic:bool),opt(interval:Vec<f64>)->()),
(sim_ik_set_joint_limit_margin,"setJointLimitMargin",(environment_handle:i64,joint_handle:i64,margin:f64)->()),
(sim_ik_set_joint_max_step_size,"setJointMaxStepSize",(environment_handle:i64,joint_handle:i64,step_size:f64)->()),
(sim_ik_set_joint_mode,"setJointMode",(environment_handle:i64,joint_handle:i64,joint_mode:i64)->()),
(sim_ik_set_joint_position,"setJointPosition",(environment_handle:i64,joint_handle:i64,position:f64)->()),
(sim_ik_set_joint_screw_lead,"setJointScrewLead",(environment_handle:i64,joint_handle:i64,lead:f64)->()),
(sim_ik_set_joint_weight,"setJointWeight",(environment_handle:i64,joint_handle:i64,weight:f64)->()),
(sim_ik_set_object_matrix,"setObjectMatrix",(environment_handle:i64,object_handle:i64,matrix:Vec<f64>),opt(relative_to_object_handle:i64)->()),
(sim_ik_set_object_parent,"setObjectParent",(environment_handle:i64,object_handle:i64,parent_object_handle:i64),opt(keep_in_place:bool)->()),
(sim_ik_set_object_pose,"setObjectPose",(environment_handle:i64,object_handle:i64,pose:Vec<f64>),opt(relative_to_object_handle:i64)->()),
(sim_ik_set_object_transformation,"setObjectTransformation",(environment_handle:i64,object_handle:i64,position:Vec<f64>,euler_or_quaternion:Vec<f64>),opt(relative_to_object_handle:i64)->()),
(sim_ik_set_spherical_joint_matrix,"setSphericalJointMatrix",(environment_handle:i64,joint_handle:i64,matrix:Vec<f64>)->()),
(sim_ik_set_spherical_joint_rotation,"setSphericalJointRotation",(environment_handle:i64,joint_handle:i64,euler_or_quaternion:Vec<f64>)->()),
(sim_ik_set_target_dummy,"setTargetDummy",(environment_handle:i64,dummy_handle:i64,target_dummy_handle:i64)->()),
(sim_ik_sync_from_sim,"syncFromSim",(environment_handle:i64,ik_groups:Vec<i64>)->()),
(sim_ik_sync_to_sim,"syncToSim",(environment_handle:i64,ik_groups:Vec<i64>)->())
}
}