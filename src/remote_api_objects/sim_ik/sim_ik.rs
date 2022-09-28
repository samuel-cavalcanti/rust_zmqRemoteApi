use crate::zmq_requests::RawRequest;
use serde_json::Value;

use crate::remote_api_client::RemoteApiClientInterface;

pub struct SimIK<'a, R: RemoteApiClientInterface> {
    client: &'a R,
}

impl<'a, R: RemoteApiClientInterface + 'a> SimIK<'a, R> {
    pub fn new(client: &'a R) -> SimIK<'a, R> {
        SimIK { client }
    }
    requests! {
    (add_ik_element,"addIkElement",(environment_handle:i64,ik_group_handle:i64,tip_dummy_handle:i64)->i64),
    (add_ik_element_from_scene,"addIkElementFromScene",(environment_handle:i64,ik_group:i64,base_handle:i64,tip_handle:i64,target_handle:i64,constraints:i64)->(i64,serde_json::Value)),
    (apply_ik_environment_to_scene,"applyIkEnvironmentToScene",(environment_handle:i64,ik_group:i64),opt(apply_only_when_successful:bool)->i64),
    (apply_scene_to_ik_environment,"applySceneToIkEnvironment",(environment_handle:i64,ik_group:i64)->()),
    (compute_jacobian,"computeJacobian",(environment_handle:i64,ik_group_handle:i64,options:i64)->bool),
    (create_dummy,"createDummy",(environment_handle:i64),opt(dummy_name:String)->i64),
    (create_environment,"createEnvironment"->i64),
    (create_ik_group,"createIkGroup",(environment_handle:i64),opt(ik_group_name:String)->i64),
    (create_joint,"createJoint",(environment_handle:i64,joint_type:i64),opt(joint_name:String)->i64),
    (does_ik_group_exist,"doesIkGroupExist",(environment_handle:i64,ik_group_name:String)->bool),
    (does_object_exist,"doesObjectExist",(environment_handle:i64,object_name:String)->bool),
    (duplicate_environment,"duplicateEnvironment",(environment_handle:i64)->i64),
    (erase_environment,"eraseEnvironment",(environment_handle:i64)->()),
    (erase_object,"eraseObject",(environment_handle:i64,object_handle:i64)->()),
    (find_config,"findConfig",(environment_handle:i64,ik_group_handle:i64,joint_handles:Vec<i64>),opt(threshold_dist:f64,max_time:f64,metric:Vec<f64>,validation_callback:String,aux_data:serde_json::Value)->Vec<f64>),
    (generate_path,"generatePath",(environment_handle:i64,ik_group_handle:i64,joint_handles:Vec<i64>,tip_handle:i64,path_point_count:i64),opt(validation_callback:String,aux_data:serde_json::Value)->Vec<f64>),
    (get_alternate_configs,"getAlternateConfigs",(environment_handle:i64,joint_handles:Vec<i64>),opt(low_limits:Vec<f64>,ranges:Vec<f64>)->Vec<f64>),
    (get_ik_element_base,"getIkElementBase",(environment_handle:i64,ik_group_handle:i64,element_handle:i64)->(i64,i64)),
    (get_ik_element_constraints,"getIkElementConstraints",(environment_handle:i64,ik_group_handle:i64,element_handle:i64)->i64),
    (get_ik_element_flags,"getIkElementFlags",(environment_handle:i64,ik_group_handle:i64,element_handle:i64)->i64),
    (get_ik_element_precision,"getIkElementPrecision",(environment_handle:i64,ik_group_handle:i64,element_handle:i64)->Vec<f64>),
    (get_ik_element_weights,"getIkElementWeights",(environment_handle:i64,ik_group_handle:i64,element_handle:i64)->Vec<f64>),
    (get_ik_group_calculation,"getIkGroupCalculation",(environment_handle:i64,ik_group_handle:i64)->(i64,f64,i64)),
    (get_ik_group_flags,"getIkGroupFlags",(environment_handle:i64,ik_group_handle:i64)->i64),
    (get_ik_group_handle,"getIkGroupHandle",(environment_handle:i64,ik_group_name:String)->i64),
    (get_jacobian,"getJacobian",(environment_handle:i64,ik_group_handle:i64)->(Vec<f64>,Vec<i64>)),
    (get_joint_dependency,"getJointDependency",(environment_handle:i64,joint_handle:i64)->(i64,f64,f64)),
    (get_joint_ik_weight,"getJointIkWeight",(environment_handle:i64,joint_handle:i64)->f64),
    (get_joint_interval,"getJointInterval",(environment_handle:i64,joint_handle:i64)->(bool,Vec<f64>)),
    (get_joint_matrix,"getJointMatrix",(environment_handle:i64,joint_handle:i64)->Vec<f64>),
    (get_joint_max_step_size,"getJointMaxStepSize",(environment_handle:i64,joint_handle:i64)->f64),
    (get_joint_mode,"getJointMode",(environment_handle:i64,joint_handle:i64)->i64),
    (get_joint_position,"getJointPosition",(environment_handle:i64,joint_handle:i64)->f64),
    (get_joint_screw_pitch,"getJointScrewPitch",(environment_handle:i64,joint_handle:i64)->f64),
    (get_joint_transformation,"getJointTransformation",(environment_handle:i64,joint_handle:i64)->(Vec<f64>,Vec<f64>,Vec<f64>)),
    (get_joint_type,"getJointType",(environment_handle:i64,joint_handle:i64)->i64),
    (get_linked_dummy,"getLinkedDummy",(environment_handle:i64,dummy_handle:i64)->i64),
    (get_manipulability,"getManipulability",(environment_handle:i64,ik_group_handle:i64)->f64),
    (get_object_handle,"getObjectHandle",(environment_handle:i64,object_name:String)->i64),
    (get_object_matrix,"getObjectMatrix",(environment_handle:i64,object_handle:i64,relative_to_object_handle:i64)->Vec<f64>),
    (get_object_parent,"getObjectParent",(environment_handle:i64,object_handle:i64)->i64),
    (get_object_pose,"getObjectPose",(environment_handle:i64,object_handle:i64,relative_to_object_handle:i64)->Vec<f64>),
    (get_object_transformation,"getObjectTransformation",(environment_handle:i64,object_handle:i64,relative_to_object_handle:i64)->(Vec<f64>,Vec<f64>,Vec<f64>)),
    (get_objects,"getObjects",(environment_handle:i64,index:i64)->(i64,String,bool,i64)),
    (handle_ik_group,"handleIkGroup",(environment_handle:i64),opt(ik_group_handle:i64)->i64),
    (load,"load",(environment_handle:i64,data:String)->()),
    (save,"save",(environment_handle:i64)->String),
    (set_ik_element_base,"setIkElementBase",(environment_handle:i64,ik_group_handle:i64,element_handle:i64,base_handle:i64),opt(constraints_base_handle:i64)->()),
    (set_ik_element_constraints,"setIkElementConstraints",(environment_handle:i64,ik_group_handle:i64,element_handle:i64,constraints:i64)->()),
    (set_ik_element_flags,"setIkElementFlags",(environment_handle:i64,ik_group_handle:i64,element_handle:i64,flags:i64)->()),
    (set_ik_element_precision,"setIkElementPrecision",(environment_handle:i64,ik_group_handle:i64,element_handle:i64,precision:Vec<f64>)->()),
    (set_ik_element_weights,"setIkElementWeights",(environment_handle:i64,ik_group_handle:i64,element_handle:i64,weights:Vec<f64>)->()),
    (set_ik_group_calculation,"setIkGroupCalculation",(environment_handle:i64,ik_group_handle:i64,method:i64,damping:f64,max_iterations:i64)->()),
    (set_ik_group_flags,"setIkGroupFlags",(environment_handle:i64,ik_group_handle:i64,flags:i64)->()),
    (set_joint_dependency,"setJointDependency",(environment_handle:i64,joint_handle:i64,dep_joint_handle:i64),opt(offset:f64,mult:f64)->()),
    (set_joint_ik_weight,"setJointIkWeight",(environment_handle:i64,joint_handle:i64,weight:f64)->()),
    (set_joint_interval,"setJointInterval",(environment_handle:i64,joint_handle:i64,cyclic:bool),opt(interval:Vec<f64>)->()),
    (set_joint_max_step_size,"setJointMaxStepSize",(environment_handle:i64,joint_handle:i64,step_size:f64)->()),
    (set_joint_mode,"setJointMode",(environment_handle:i64,joint_handle:i64,joint_mode:i64)->()),
    (set_joint_position,"setJointPosition",(environment_handle:i64,joint_handle:i64,position:f64)->()),
    (set_joint_screw_pitch,"setJointScrewPitch",(environment_handle:i64,joint_handle:i64,pitch:f64)->()),
    (set_linked_dummy,"setLinkedDummy",(environment_handle:i64,dummy_handle:i64,linked_dummy_handle:i64)->()),
    (set_object_matrix,"setObjectMatrix",(environment_handle:i64,object_handle:i64,relative_to_object_handle:i64,matrix:Vec<f64>)->()),
    (set_object_parent,"setObjectParent",(environment_handle:i64,object_handle:i64,parent_object_handle:i64),opt(keep_in_place:bool)->()),
    (set_object_pose,"setObjectPose",(environment_handle:i64,object_handle:i64,relative_to_object_handle:i64,pose:Vec<f64>)->()),
    (set_object_transformation,"setObjectTransformation",(environment_handle:i64,object_handle:i64,relative_to_object_handle:i64,position:Vec<f64>,euler_or_quaternion:Vec<f64>)->()),
    (set_spherical_joint_matrix,"setSphericalJointMatrix",(environment_handle:i64,joint_handle:i64,matrix:Vec<f64>)->()),
    (set_spherical_joint_rotation,"setSphericalJointRotation",(environment_handle:i64,joint_handle:i64,euler_or_quaternion:Vec<f64>)->())
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
}
