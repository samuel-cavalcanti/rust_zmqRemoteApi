use crate::RemoteApiClientInterface;
#[doc=r#"API functions for creating kinematics tasks. All units, unless otherwise indicated, are specified in meters and radians."#]
pub trait SimIK : RemoteApiClientInterface {
    requests!{
"simIK",
(r#######"Adds a new IK element to an IK group."#######,sim_ik_add_element,"addElement",(environment_handle:i64,ik_group_handle:i64,tip_dummy_handle:i64)->i64),
(r#######"Convenience function to quickly generate an IK element from a kinematic chain in the scene"#######,sim_ik_add_element_from_scene,"addElementFromScene",(environment_handle:i64,ik_group:i64,base_handle:i64,tip_handle:i64,target_handle:i64,constraints:i64)->(i64,serde_json::Value,serde_json::Value)),
(r#######"Computes the Jacobian and error vector for an IK group"#######,sim_ik_compute_group_jacobian,"computeGroupJacobian",(environment_handle:i64,ik_group_handle:i64)->(Vec<f64>,Vec<f64>)),
(r#######"Computes the Jacobian and error vector for a kinematic chain"#######,sim_ik_compute_jacobian,"computeJacobian",(environment_handle:i64,base_object:i64,last_joint:i64,constraints:i64,tip_matrix:Vec<f64>),opt(target_matrix:Vec<f64>,constr_base_matrix:Vec<f64>)->(Vec<f64>,Vec<f64>)),
(r#######"Creates a visual representation of an IK chain."#######,sim_ik_create_debug_overlay,"createDebugOverlay",(environment_handle:i64,tip_handle:i64),opt(base_handle:i64)->i64),
(r#######"Creates a dummy object."#######,sim_ik_create_dummy,"createDummy",(environment_handle:i64),opt(dummy_name:String)->i64),
(r#######"Creates an new IK environment."#######,sim_ik_create_environment,"createEnvironment",opt(flags:i64)->i64),
(r#######"Creates an IK group."#######,sim_ik_create_group,"createGroup",(environment_handle:i64),opt(ik_group_name:String)->i64),
(r#######"Creates a joint object."#######,sim_ik_create_joint,"createJoint",(environment_handle:i64,joint_type:i64),opt(joint_name:String)->i64),
(r#######"Checks whether an IK group exists, based on its name."#######,sim_ik_does_group_exist,"doesGroupExist",(environment_handle:i64,ik_group_name:String)->bool),
(r#######"Checks whether an object exists, based on its name."#######,sim_ik_does_object_exist,"doesObjectExist",(environment_handle:i64,object_name:String)->bool),
(r#######"Duplicates an IK environment. Useful when operating on an environment while leaving the original environment unchanged."#######,sim_ik_duplicate_environment,"duplicateEnvironment",(environment_handle:i64)->i64),
(r#######"Removes the visual representation of an IK chain."#######,sim_ik_erase_debug_overlay,"eraseDebugOverlay",(debug_object:i64)->()),
(r#######"Erases an IK environment."#######,sim_ik_erase_environment,"eraseEnvironment",(environment_handle:i64)->()),
(r#######"Erases an object."#######,sim_ik_erase_object,"eraseObject",(environment_handle:i64,object_handle:i64)->()),
(r#######"Deprecated. See simIK.findConfigs instead."#######,sim_ik_find_config,"findConfig",(environment_handle:i64,ik_group_handle:i64,joint_handles:Vec<i64>),opt(threshold_dist:f64,max_time:f64,metric:Vec<f64>,validation_callback:String,aux_data:serde_json::Value)->Vec<f64>),
(r#######"Generates a path that drives the IK tip onto its IK target, in a straight line (i.e. shortest path in Cartesian space). The function returns a path in the configuration space if the operation was successful. A reason for a non-successful operation can be: there are some forbidden poses/configurations on the way, or some of the configuration points cannot be reached (e.g. out of reach, or due to joint limits). The IK environment remains unchanged."#######,sim_ik_generate_path,"generatePath",(environment_handle:i64,ik_group_handle:i64,joint_handles:Vec<i64>,tip_handle:i64,path_point_count:i64),opt(validation_callback:String,aux_data:serde_json::Value)->Vec<f64>),
(r#######"Generates alternative manipulator configurations, for a same end-effector pose, for a manipulator that has revolute joints with a range larger than 360 degrees. The original submitted configuration will be part of the returned configurations. The IK environment remains unchanged."#######,sim_ik_get_alternate_configs,"getAlternateConfigs",(environment_handle:i64,joint_handles:Vec<i64>),opt(low_limits:Vec<f64>,ranges:Vec<f64>)->Vec<f64>),
(r#######"Retrieves the base object of an IK element."#######,sim_ik_get_element_base,"getElementBase",(environment_handle:i64,ik_group_handle:i64,element_handle:i64)->(i64,i64)),
(r#######"Retrieves the constraints of an IK element."#######,sim_ik_get_element_constraints,"getElementConstraints",(environment_handle:i64,ik_group_handle:i64,element_handle:i64)->i64),
(r#######"Retrieves various flags of an IK element."#######,sim_ik_get_element_flags,"getElementFlags",(environment_handle:i64,ik_group_handle:i64,element_handle:i64)->i64),
(r#######"Retrieves the precision settings of an IK element."#######,sim_ik_get_element_precision,"getElementPrecision",(environment_handle:i64,ik_group_handle:i64,element_handle:i64)->Vec<f64>),
(r#######"Retrieves the desired linear and angular resolution weights of an IK element."#######,sim_ik_get_element_weights,"getElementWeights",(environment_handle:i64,ik_group_handle:i64,element_handle:i64)->Vec<f64>),
(r#######""#######,sim_ik_get_failure_description,"getFailureDescription",(reason:i64)->String),
(r#######"Retrieves calculation properties for an IK group."#######,sim_ik_get_group_calculation,"getGroupCalculation",(environment_handle:i64,ik_group_handle:i64)->(i64,f64,i64)),
(r#######"Retrieves flags of an IK group."#######,sim_ik_get_group_flags,"getGroupFlags",(environment_handle:i64,ik_group_handle:i64)->i64),
(r#######"Retrieves the handle of an IK group based on its name."#######,sim_ik_get_group_handle,"getGroupHandle",(environment_handle:i64,ik_group_name:String)->i64),
(r#######"Checks which joints of an IK group hit a limit last time that IK group was handled"#######,sim_ik_get_group_joint_limit_hits,"getGroupJointLimitHits",(environment_handle:i64,ik_group_handle:i64)->(Vec<i64>,Vec<f64>)),
(r#######"Returns the joint handles involved in the IK group calculation, i.e. one handle per Jacobian column (except with revolute joints that have 3 corresponding Jacobian columns)"#######,sim_ik_get_group_joints,"getGroupJoints",(environment_handle:i64,ik_group_handle:i64)->Vec<i64>),
(r#######"Retrieves information about a possible joint dependency."#######,sim_ik_get_joint_dependency,"getJointDependency",(environment_handle:i64,joint_handle:i64)->(i64,f64,f64)),
(r#######"Retrieves the joint limits."#######,sim_ik_get_joint_interval,"getJointInterval",(environment_handle:i64,joint_handle:i64)->(bool,Vec<f64>)),
(r#######"Retrieves the limit margin of a joint, i.e. the threshold that will be used to counteract on joint limit violation during IK resolution, if the appropriate IK group flag was set"#######,sim_ik_get_joint_limit_margin,"getJointLimitMargin",(environment_handle:i64,joint_handle:i64)->f64),
(r#######"Retrieves the intrinsic transformation matrix of a joint."#######,sim_ik_get_joint_matrix,"getJointMatrix",(environment_handle:i64,joint_handle:i64)->Vec<f64>),
(r#######"Retrieves the maximum step size of a joint."#######,sim_ik_get_joint_max_step_size,"getJointMaxStepSize",(environment_handle:i64,joint_handle:i64)->f64),
(r#######"Retrieves the joint mode."#######,sim_ik_get_joint_mode,"getJointMode",(environment_handle:i64,joint_handle:i64)->i64),
(r#######"Retrieves the position (linear or angular) of a joint."#######,sim_ik_get_joint_position,"getJointPosition",(environment_handle:i64,joint_handle:i64)->f64),
(r#######"Retrieves the screw lead of a revolute joint."#######,sim_ik_get_joint_screw_lead,"getJointScrewLead",(environment_handle:i64,joint_handle:i64)->f64),
(r#######"Retrieves the intrinsic transformation of a joint."#######,sim_ik_get_joint_transformation,"getJointTransformation",(environment_handle:i64,joint_handle:i64)->(Vec<f64>,Vec<f64>,Vec<f64>)),
(r#######"Retrieves the joint type."#######,sim_ik_get_joint_type,"getJointType",(environment_handle:i64,joint_handle:i64)->i64),
(r#######"Retrieves the IK weight of a joint, i.e. the weight it has during IK resolution."#######,sim_ik_get_joint_weight,"getJointWeight",(environment_handle:i64,joint_handle:i64)->f64),
(r#######"Retrieves the handle of an object based on its name."#######,sim_ik_get_object_handle,"getObjectHandle",(environment_handle:i64,object_name:String)->i64),
(r#######"Retrieves the transformation matrix of an object. If the object is a joint object, the matrix does not include the joint's intrinsic transformation."#######,sim_ik_get_object_matrix,"getObjectMatrix",(environment_handle:i64,object_handle:i64),opt(relative_to_object_handle:i64)->Vec<f64>),
(r#######"Retrieves an object's parent handle."#######,sim_ik_get_object_parent,"getObjectParent",(environment_handle:i64,object_handle:i64)->i64),
(r#######"Retrieves the pose (position and quaternion) of an object. If the object is a joint object, the pose does not include the joint's intrinsic transformation."#######,sim_ik_get_object_pose,"getObjectPose",(environment_handle:i64,object_handle:i64),opt(relative_to_object_handle:i64)->Vec<f64>),
(r#######"Retrieves the transformation (position and quaternion/euler angles) of an object. If the object is a joint object, the transformation does not include the joint's intrinsic transformation."#######,sim_ik_get_object_transformation,"getObjectTransformation",(environment_handle:i64,object_handle:i64),opt(relative_to_object_handle:i64)->(Vec<f64>,Vec<f64>,Vec<f64>)),
(r#######"Retrieves the type of an object."#######,sim_ik_get_object_type,"getObjectType",(environment_handle:i64,object_handle:i64)->i64),
(r#######"Allows to loop through all objects in the environment."#######,sim_ik_get_objects,"getObjects",(environment_handle:i64,index:i64)->(i64,String,bool,i64)),
(r#######"Retrieves the handle of the target dummy associated with a tip dummy"#######,sim_ik_get_target_dummy,"getTargetDummy",(environment_handle:i64,dummy_handle:i64)->i64),
(r#######"Handles (i.e. computes/resolves) an IK group. Convenience function for simIK.handleIkGroups(ikEnv,{ikGroupHandle},..)"#######,sim_ik_handle_group,"handleGroup",(environment_handle:i64,ik_group:i64),opt(options:serde_json::Value)->(i64,i64,Vec<f64>)),
(r#######"Handles (i.e. computes/resolves) one or several IK groups"#######,sim_ik_handle_groups,"handleGroups",(environment_handle:i64,ik_groups:Vec<i64>),opt(options:serde_json::Value)->(i64,i64,Vec<f64>)),
(r#######"Loads kinematic content previously exported in the CoppeliaSim application. Make sure that the environment is empty before calling this function."#######,sim_ik_load,"load",(environment_handle:i64,data:String)->()),
(r#######"Saves the kinematic content of an IK environment."#######,sim_ik_save,"save",(environment_handle:i64)->String),
(r#######"Sets the base object of an IK element."#######,sim_ik_set_element_base,"setElementBase",(environment_handle:i64,ik_group_handle:i64,element_handle:i64,base_handle:i64),opt(constraints_base_handle:i64)->()),
(r#######"Sets the constraints of an IK element."#######,sim_ik_set_element_constraints,"setElementConstraints",(environment_handle:i64,ik_group_handle:i64,element_handle:i64,constraints:i64)->()),
(r#######"Sets various flags of an IK element."#######,sim_ik_set_element_flags,"setElementFlags",(environment_handle:i64,ik_group_handle:i64,element_handle:i64,flags:i64)->()),
(r#######"Sets the desired precision of an IK element."#######,sim_ik_set_element_precision,"setElementPrecision",(environment_handle:i64,ik_group_handle:i64,element_handle:i64,precision:Vec<f64>)->()),
(r#######"Sets the desired linear and angular resolution weights of an IK element."#######,sim_ik_set_element_weights,"setElementWeights",(environment_handle:i64,ik_group_handle:i64,element_handle:i64,weights:Vec<f64>)->()),
(r#######"Sets calculation properties for an IK group."#######,sim_ik_set_group_calculation,"setGroupCalculation",(environment_handle:i64,ik_group_handle:i64,method:i64,damping:f64,max_iterations:i64)->()),
(r#######"Sets flags of an IK group."#######,sim_ik_set_group_flags,"setGroupFlags",(environment_handle:i64,ik_group_handle:i64,flags:i64)->()),
(r#######"Sets information about a possible dependent joint."#######,sim_ik_set_joint_dependency,"setJointDependency",(environment_handle:i64,joint_handle:i64,master_joint_handle:i64),opt(offset:f64,mult:f64,callback:String)->()),
(r#######"Sets the joint limits."#######,sim_ik_set_joint_interval,"setJointInterval",(environment_handle:i64,joint_handle:i64,cyclic:bool),opt(interval:Vec<f64>)->()),
(r#######"Sets the limit margin of a joint, i.e. the threshold that will be used to counteract on joint limit violation during IK resolution, if the appropriate IK group flag was set"#######,sim_ik_set_joint_limit_margin,"setJointLimitMargin",(environment_handle:i64,joint_handle:i64,margin:f64)->()),
(r#######"Sets the maximum step size of a joint."#######,sim_ik_set_joint_max_step_size,"setJointMaxStepSize",(environment_handle:i64,joint_handle:i64,step_size:f64)->()),
(r#######"Sets the joint mode."#######,sim_ik_set_joint_mode,"setJointMode",(environment_handle:i64,joint_handle:i64,joint_mode:i64)->()),
(r#######"Sets the position (linear or angular) of a joint."#######,sim_ik_set_joint_position,"setJointPosition",(environment_handle:i64,joint_handle:i64,position:f64)->()),
(r#######"Sets the screw lead, in case of a revolute joint."#######,sim_ik_set_joint_screw_lead,"setJointScrewLead",(environment_handle:i64,joint_handle:i64,lead:f64)->()),
(r#######"Sets the IK weight of a joint, i.e. the weight it has during IK resolution."#######,sim_ik_set_joint_weight,"setJointWeight",(environment_handle:i64,joint_handle:i64,weight:f64)->()),
(r#######"Sets the transformation matrix of an object. If the object is a joint object, the matrix does not include the joint's intrinsic transformation."#######,sim_ik_set_object_matrix,"setObjectMatrix",(environment_handle:i64,object_handle:i64,matrix:Vec<f64>),opt(relative_to_object_handle:i64)->()),
(r#######"Sets the parent of an object."#######,sim_ik_set_object_parent,"setObjectParent",(environment_handle:i64,object_handle:i64,parent_object_handle:i64),opt(keep_in_place:bool)->()),
(r#######"Sets the pose (position and quaternion) of an object. If the object is a joint object, the pose does not include the joint's intrinsic transformation."#######,sim_ik_set_object_pose,"setObjectPose",(environment_handle:i64,object_handle:i64,pose:Vec<f64>),opt(relative_to_object_handle:i64)->()),
(r#######"Sets the transformation (position and quaternion/Euler angles) of an object. If the object is a joint object, the transformation does not include the joint's intrinsic transformation."#######,sim_ik_set_object_transformation,"setObjectTransformation",(environment_handle:i64,object_handle:i64,position:Vec<f64>,euler_or_quaternion:Vec<f64>),opt(relative_to_object_handle:i64)->()),
(r#######"Sets the rotation transformation matrix of a spherical joint."#######,sim_ik_set_spherical_joint_matrix,"setSphericalJointMatrix",(environment_handle:i64,joint_handle:i64,matrix:Vec<f64>)->()),
(r#######"Sets the rotation transformation of a spherical joint."#######,sim_ik_set_spherical_joint_rotation,"setSphericalJointRotation",(environment_handle:i64,joint_handle:i64,euler_or_quaternion:Vec<f64>)->()),
(r#######"Associates a tip dummy with a target dummy, or removes that association. If the tip dummy is already associated with another target dummy, then first remove that association before setting another one"#######,sim_ik_set_target_dummy,"setTargetDummy",(environment_handle:i64,dummy_handle:i64,target_dummy_handle:i64)->()),
(r#######"Convenience function to apply the scene state to its ik environment counterpart. Use together with simIK.addElementFromScene."#######,sim_ik_sync_from_sim,"syncFromSim",(environment_handle:i64,ik_groups:Vec<i64>)->()),
(r#######"Convenience function to apply inverse kinematic values computed in the IK world, to the scene. Use together with simIK.addElementFromScene."#######,sim_ik_sync_to_sim,"syncToSim",(environment_handle:i64,ik_groups:Vec<i64>)->())
}
}