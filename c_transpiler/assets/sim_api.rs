use crate::RemoteApiClientInterface;
#[doc=r#"All units going to, or coming from the API are in meters, kilograms, seconds and radians or a combination of those (unless otherwise explicitly indicated)"#]
pub trait Sim : RemoteApiClientInterface {
    requests!{
"sim",
(r#######"Deprecated. See [sim_get_joint_target_force](#method.sim_get_joint_target_force) instead."#######,sim_get_joint_max_force,"getJointMaxForce",(joint_handle:i64)->f64),
(r#######"Deprecated. See [sim_set_joint_target_force](#method.sim_set_joint_target_force) instead."#######,sim_set_joint_max_force,"setJointMaxForce",(object_handle:i64,force_or_torque:f64)->()),
(r#######"Deprecated. Use [sim_create_primitive_shape](#method.sim_create_primitive_shape) instead"#######,sim_create_pure_shape,"createPureShape",(primitive_type:i64,options:i64,sizes:Vec<f64>,mass:f64),opt(precision:Vec<i64>)->i64),
(r#######"Deprecated. See [sim_remove_objects](#method.sim_remove_objects) instead
"#######,sim_remove_object,"removeObject",(object_handle:i64)->()),
(r#######"Deprecated. Use [sim_get_vision_sensor_depth](#method.sim_get_vision_sensor_depth) instead"#######,sim_get_vision_sensor_depth_buffer,"getVisionSensorDepthBuffer",(sensor_handle:i64),opt(pos:Vec<i64>,size:Vec<i64>)->(Vec<u8>,Vec<i64>)),
(r#######"Deprecated. Use [sim_get_vision_sensor_img](#method.sim_get_vision_sensor_img) instead"#######,sim_get_vision_sensor_char_image,"getVisionSensorCharImage",(sensor_handle:i64),opt(pos:Vec<i64>,size:Vec<i64>)->(Vec<u8>,Vec<i64>)),
(r#######"Deprecated. Use [sim_set_vision_sensor_img](#method.sim_set_vision_sensor_img) instead"#######,sim_set_vision_sensor_char_image,"setVisionSensorCharImage",(sensor_handle:i64,image:Vec<u8>)->()),
(r#######"Deprecated. Use [sim_get_object_sel](#method.sim_get_object_sel) instead"#######,sim_get_object_selection,"getObjectSelection"->Vec<i64>),
(r#######"Deprecated. Use [sim_set_object_sel](#method.sim_set_object_sel) instead"#######,sim_set_object_selection,"setObjectSelection",(object_handles:Vec<f64>)->()),
(r#######"Deprecated. See properties instead"#######,sim_get_string_signal,"getStringSignal",(signal_name:String)->Option<Vec<u8>>),
(r#######"Deprecated. See properties instead"#######,sim_get_int32_signal,"getInt32Signal",(signal_name:String)->Option<i64>),
(r#######"Deprecated. See properties instead"#######,sim_get_float_signal,"getFloatSignal",(signal_name:String)->Option<f64>),
(r#######"Calls a script function (from a plugin, the main client application,
or from another script). This represents a user callback inside of a script. The
target script must be initialized for this call to succeed, e.g. when calling simulation scripts,
then simulation must be running"#######,sim_call_script_function,"callScriptFunction",(function_name:String,script_handle:i64,in_args:serde_json::Value)->serde_json::Value),
(r#######"Allows to have CoppeliaSim wait for a threaded code section to be executed without
interruption. Locking is cumulative"#######,sim_acquire_lock,"acquireLock"->()),
(r#######"Adds a drawing object that will be displayed in the scene. Drawing objects are containers
that hold several items of a given type. This can be used for several different applications
(simulation of paint, simulation of welding seam, display of 3D objects, etc.). Drawing objects
created in a simulation script will be automatically
removed at simulation end"#######,sim_add_drawing_object,"addDrawingObject",(object_type:i64,size:f64,duplicate_tolerance:f64,parent_object_handle:i64,max_item_count:i64),opt(color:Vec<f64>)->i64),
(r#######"Adds an item (or clears all items) to a previously inserted drawing object"#######,sim_add_drawing_object_item,"addDrawingObjectItem",(drawing_object_handle:i64,item_data:Vec<f64>)->i64),
(r#######"Adds a non-central force to a shape object that is dynamically enabled. Added forces are cumulative,
applied relative to the center of mass, and are reset to zero after [sim_handle_dynamics](#method.sim_handle_dynamics)
was called (or by using the following flag: sim.handleflag_resetforcetorque)"#######,sim_add_force,"addForce",(shape_handle:i64,position:Vec<f64>,force:Vec<f64>)->()),
(r#######"Adds a force and/or torque to a shape object that is dynamically enabled.
Forces are applied at the center of mass. Added forces and torques are cumulative, and are reset to zero
after [sim_handle_dynamics](#method.sim_handle_dynamics) was called (or by using the following
flags: sim.handleflag_resetforce and/or sim.handleflag_resettorque)"#######,sim_add_force_and_torque,"addForceAndTorque",(shape_handle:i64),opt(force:Vec<f64>,torque:Vec<f64>)->()),
(r#######"Adds or updates a graph curve. A graph curve is persistent, but can be removed with
[sim_destroy_graph_curve](#method.sim_destroy_graph_curve)
"#######,sim_add_graph_curve,"addGraphCurve",(graph_handle:i64,curve_name:String,dim:i64,stream_ids:Vec<i64>,default_values:Vec<f64>,unit_str:String),opt(options:i64,color:Vec<f64>,curve_width:i64)->i64),
(r#######"Adds or updates a graph stream. A graph stream is persistent, but can be removed
with [sim_destroy_graph_curve](#method.sim_destroy_graph_curve)
"#######,sim_add_graph_stream,"addGraphStream",(graph_handle:i64,stream_name:String,unit:String),opt(options:i64,color:Vec<f64>,cyclic_range:f64)->i64),
(r#######"Adds an item to a collection
"#######,sim_add_item_to_collection,"addItemToCollection",(collection_handle:i64,what:i64,object_handle:i64,options:i64)->()),
(r#######"Adds a log message that will be output in the console or status bar"#######,sim_add_log,"addLog",(verbosity_level:i64,log_message:String)->()),
(r#######"Adds a particle object that will be simulated and displayed in the scene. Particle objects
are containers that hold several items (particles) of a given type. This can be used for
several different applications (e.g. simulation of air/water jets)"#######,sim_add_particle_object,"addParticleObject",(object_type:i64,size:f64,density:f64,params:Vec<f64>,life_time:f64,max_item_count:i64),opt(color:Vec<f64>)->i64),
(r#######"Adds an item (or clears all items) to a previously inserted particle object"#######,sim_add_particle_object_item,"addParticleObjectItem",(object_handle:i64,item_data:Vec<f64>)->()),
(r#######""#######,sim_add_referenced_handle,"addReferencedHandle",(object_handle:i64,referenced_handle:i64),opt(tag:String,opts:serde_json::Value)->()),
(r#######"Adjusts parameters of a view"#######,sim_adjust_view,"adjustView",(view_handle_or_index:i64,object_handle:i64,options:i64),opt(view_label:String)->i64),
(r#######"Reorients the bounding box of a shape, while keeping the shape frame in place.
The shape's inertia properties are unaffected"#######,sim_align_shape_bb,"alignShapeBB",(shape_handle:i64,pose:Vec<f64>)->i64),
(r#######"Converts CoppeliaSim's alpha-beta-gamma angles to Yaw-Pitch-Roll angles"#######,sim_alpha_beta_gamma_to_yaw_pitch_roll,"alphaBetaGammaToYawPitchRoll",(alpha_angle:f64,beta_angle:f64,gamma_angle:f64)->(f64,f64,f64)),
(r#######"Announces a change in the scene. This is required for the undo/redo function to operate properly
when performing changes via the API. Only call this function directly after a change was made through
a dialog element (e.g. a checkbox was checked/unchecked) and that change was reported to the scene.
What this call will do is following: the whole scene will be serialized (saved) to memory as a
"scene image" and compared to a previously memorized "scene image". If both images
are same, then the last image is discarded, otherwise only the changes between the two images are memorized.
A call to this function has no effect (and doesn't generate any error) when called during simulation or when in edit mode."#######,sim_announce_scene_content_change,"announceSceneContentChange"->i64),
(r#######"Closes an auxiliary console window"#######,sim_auxiliary_console_close,"auxiliaryConsoleClose",(console_handle:i64)->i64),
(r#######"Opens an auxiliary console window for text display. This console window is different from
the application main console window. Console window handles are shared across all simulator scenes"#######,sim_auxiliary_console_open,"auxiliaryConsoleOpen",(title:String,max_lines:i64,mode:i64),opt(position:Vec<i64>,size:Vec<i64>,text_color:Vec<f64>,background_color:Vec<f64>)->i64),
(r#######"Prints to an auxiliary console window"#######,sim_auxiliary_console_print,"auxiliaryConsolePrint",(console_handle:i64,text:String)->i64),
(r#######"Shows or hides an auxiliary console window"#######,sim_auxiliary_console_show,"auxiliaryConsoleShow",(console_handle:i64,show_state:bool)->i64),
(r#######"Broadcasts a message to all scripts, except for the emitting script. Messages are received
synchronously via the sysCall_msg callback function
"#######,sim_broadcast_msg,"broadcastMsg",(message:serde_json::Value),opt(options:i64)->()),
(r#######"Builds an identity transformation matrix"#######,sim_build_identity_matrix,"buildIdentityMatrix"->Vec<f64>),
(r#######"Builds a transformation matrix based on a position vector and
Euler angles
"#######,sim_build_matrix,"buildMatrix",(position:Vec<f64>,euler_angles:Vec<f64>)->Vec<f64>),
(r#######"Builds a pose based on a position vector and Euler angles or
axes"#######,sim_build_pose,"buildPose",(position:Vec<f64>,euler_angles_or_axis:Vec<f64>),opt(mode:i64,axis2:Vec<f64>)->Vec<f64>),
(r#######"Shifts and adjusts a camera associated with a view to fill the view entirely with
the specified objects or models"#######,sim_camera_fit_to_view,"cameraFitToView",(view_handle_or_index:i64),opt(object_handles:Vec<i64>,options:i64,scaling:f64)->i64),
(r#######"Changes the color of an entity, and returns its original
color. Currently only takes into account collections
and shapes
"#######,sim_change_entity_color,"changeEntityColor",(entity_handle:i64,new_color:Vec<f64>),opt(color_component:i64)->Vec<serde_json::Value>),
(r#######"Checks whether two entities are colliding. The collidable flags of the entities
are overridden if the entities are objects. If the entities are both the same collection
(i.e. with the same collection handle), then same objects will not be checked against themselve"#######,sim_check_collision,"checkCollision",(entity1_handle:i64,entity2_handle:i64)->(i64,Vec<i64>)),
(r#######"Checks whether two entities are colliding, and will return all intersections between
the two entities. The collidable flags of the entities are overridden if the entities
are objects"#######,sim_check_collision_ex,"checkCollisionEx",(entity1_handle:i64,entity2_handle:i64)->(i64,Vec<f64>)),
(r#######"Checks the minimum distance between two entities. The measurable flags of the
entities are overridden if the entities are objects. If the entities are both
the same collection (i.e. with the same collection handle), then same objects
will not be checked against themselve"#######,sim_check_distance,"checkDistance",(entity1_handle:i64,entity2_handle:i64),opt(threshold:f64)->(i64,Vec<f64>,Vec<i64>)),
(r#######"Checks whether the provided points collide with the OC tree voxels"#######,sim_check_octree_point_occupancy,"checkOctreePointOccupancy",(octree_handle:i64,options:i64,points:Vec<f64>)->(i64,i64,i64,i64)),
(r#######"Checks whether the proximity sensor detects the indicated entity. Detection is silent (no visual
feedback) compared to [sim_handle_proximity_sensor](#method.sim_handle_proximity_sensor).
Also, the detectable flags of the entity are overridden if the entity is an object"#######,sim_check_proximity_sensor,"checkProximitySensor",(sensor_handle:i64,entity_handle:i64)->(i64,f64,Vec<f64>,i64,Vec<f64>)),
(r#######"Checks whether the proximity sensor detects the indicated entity. Detection is silent (no visual feedback)
compared to [sim_handle_proximity_sensor](#method.sim_handle_proximity_sensor). Also, the detectable flags
of the entity are overridden if the entity is an object"#######,sim_check_proximity_sensor_ex,"checkProximitySensorEx",(sensor_handle:i64,entity_handle:i64,mode:i64,threshold:f64,max_angle:f64)->(i64,f64,Vec<f64>,i64,Vec<f64>)),
(r#######"Checks whether the proximity sensor detects the indicated points, segments or triangles.
Detection is silent (no visual feedback)"#######,sim_check_proximity_sensor_ex2,"checkProximitySensorEx2",(sensor_handle:i64,vertices:Vec<f64>,item_type:i64,item_count:i64,mode:i64,threshold:f64,max_angle:f64)->(i64,f64,Vec<f64>,Vec<f64>)),
(r#######"Checks whether the vision sensor detects the indicated entity. Detection is silent
(no visual feedback) compared to [sim_handle_vision_sensor](#method.sim_handle_vision_sensor).
The vision callback functions will be called on the
acquired image. Also, the visibility state of the entity is overridden if the entity is an object"#######,sim_check_vision_sensor,"checkVisionSensor",(sensor_handle:i64,entity_handle:i64)->(i64,Vec<f64>,Vec<f64>)),
(r#######"Checks whether the vision sensor detects the indicated entity. Detection is silent (no visual feedback) compared
to [sim_handle_vision_sensor](#method.sim_handle_vision_sensor). The vision callback functions
will be called on the acquired image. Also, the visibility state of the entity is overridden if the entity is an object"#######,sim_check_vision_sensor_ex,"checkVisionSensorEx",(sensor_handle:i64,entity_handle:i64,return_image:bool)->Vec<f64>),
(r#######"Deprecated. See properties instead"#######,sim_clear_buffer_signal,"clearBufferSignal",(signal_name:String)->()),
(r#######"Deprecated. See properties instead"#######,sim_clear_float_signal,"clearFloatSignal",(signal_name:String)->()),
(r#######"Deprecated. See properties instead"#######,sim_clear_int32_signal,"clearInt32Signal",(signal_name:String)->()),
(r#######"Deprecated. See properties instead"#######,sim_clear_string_signal,"clearStringSignal",(signal_name:String)->()),
(r#######"Closes current scene, and switches to another open scene. If there is no other open scene,
a new scene is then created. Can only be called from an add-on,
or from the sanbox script, when called from within CoppeliaSim"#######,sim_close_scene,"closeScene"->i64),
(r#######"Combines two RGB images"#######,sim_combine_rgb_images,"combineRgbImages",(img1:Vec<u8>,img1_res:Vec<i64>,img2:Vec<u8>,img2_res:Vec<i64>,operation:i64)->Vec<u8>),
(r#######"Computes and applies the mass and inertia matrix for a shape, based on its convex representation.
When calling this function while the simulation is running, one should then call
[sim_reset_dynamic_object](#method.sim_reset_dynamic_object), for the changes to take effect"#######,sim_compute_mass_and_inertia,"computeMassAndInertia",(shape_handle:i64,density:f64)->i64),
(r#######"Copies and pastes objects"#######,sim_copy_paste_objects,"copyPasteObjects",(object_handles:Vec<i64>),opt(options:i64)->Vec<i64>),
(r#######"Lua only. Duplicates a table, i.e. makes a deep copy
        "#######,sim_copy_table,"copyTable",(original:Vec<serde_json::Value>)->Vec<serde_json::Value>),
(r#######"Creates a new collection. A collection created in a
simulation script, a customization script or in
the main script are automatically destroyed when the script ends"#######,sim_create_collection,"createCollection",opt(options:i64)->i64),
(r#######"Creates a dummy."#######,sim_create_dummy,"createDummy",(size:f64)->i64),
(r#######"Creates a force sensor."#######,sim_create_force_sensor,"createForceSensor",(options:i64,int_params:Vec<i64>,float_params:Vec<f64>)->i64),
(r#######"Creates a heightfield shape
"#######,sim_create_heightfield_shape,"createHeightfieldShape",(options:i64,shading_angle:f64,x_point_count:i64,y_point_count:i64,x_size:f64,heights:Vec<f64>)->i64),
(r#######"Creates a joint
"#######,sim_create_joint,"createJoint",(joint_type:i64,joint_mode:i64,options:i64),opt(sizes:Vec<f64>)->i64),
(r#######"Creates an empty OC tree
"#######,sim_create_octree,"createOctree",(voxel_size:f64,options:i64,point_size:f64)->i64),
(r#######"Creates a path."#######,sim_create_path,"createPath",(ctrl_pts:Vec<f64>),opt(options:i64,subdiv:i64,smoothness:f64,orientation_mode:i64,up_vector:Vec<f64>)->i64),
(r#######"Creates an empty point cloud
"#######,sim_create_point_cloud,"createPointCloud",(max_voxel_size:f64,max_pt_cnt_per_voxel:i64,options:i64,point_size:f64)->i64),
(r#######"Creates a primitive shape"#######,sim_create_primitive_shape,"createPrimitiveShape",(primitive_type:i64,sizes:Vec<f64>),opt(options:i64)->i64),
(r#######"Creates a proximity sensor
"#######,sim_create_proximity_sensor,"createProximitySensor",(sensor_type:i64,sub_type:i64,options:i64,int_params:Vec<i64>,float_params:Vec<f64>)->i64),
(r#######"Creates a script object."#######,sim_create_script,"createScript",(script_type:i64,script_string:String),opt(options:i64,lang:String)->i64),
(r#######"Creates a mesh shape"#######,sim_create_shape,"createShape",(options:i64,shading_angle:f64,vertices:Vec<f64>,indices:Vec<i64>,normals:Vec<f64>,texture_coordinates:Vec<f64>,texture:Vec<u8>,texture_resolution:Vec<i64>)->i64),
(r#######"Creates a planar shape, that will be textured with a new, or imported texture"#######,sim_create_texture,"createTexture",(file_name:String,options:i64),opt(plane_sizes:Vec<f64>,scaling_uv:Vec<f64>,xy_g:Vec<f64>,fixed_resolution:i64,resolution:Vec<i64>)->(i64,i64,Vec<i64>)),
(r#######"Creates a vision sensor."#######,sim_create_vision_sensor,"createVisionSensor",(options:i64,int_params:Vec<i64>,float_params:Vec<f64>)->i64),
(r#######"Removes a collection
"#######,sim_destroy_collection,"destroyCollection",(collection_handle:i64)->()),
(r#######"Destroys a graph stream or curve"#######,sim_destroy_graph_curve,"destroyGraphCurve",(graph_handle:i64,curve_id:i64)->()),
(r#######"Duplicates a graph stream or curve, and freezes it"#######,sim_duplicate_graph_curve_to_static,"duplicateGraphCurveToStatic",(graph_handle:i64,curve_id:i64),opt(curve_name:String)->i64),
(r#######"Executes some code in a specific script (from a plugin,
the main client application, or from another script).
The target script must be initialized for this call to succeed, e.g. when calling
simulation scripts, then simulation must be running


From C/C++, data exchange between a plugin and a script happens via a stack. Reading and writing arguments from/to the stack gives you a maximum of flexibility, and you wil be able to exchange also complex data structures. But it can also be tedious and error prone. Use instead the helper classes located in programming/include/simStack
"#######,sim_execute_script_string,"executeScriptString",(string_to_execute:String,script_handle:i64)->(i64,serde_json::Value)),
(r#######"Exports a mesh to a file"#######,sim_export_mesh,"exportMesh",(fileformat:i64,path_and_filename:String,options:i64,scaling_factor:f64,vertices:Vec<f64>,indices:Vec<i64>)->()),
(r#######"Adds a floating view to current page"#######,sim_floating_view_add,"floatingViewAdd",(pos_x:f64,pos_y:f64,size_x:f64,size_y:f64,options:i64)->i64),
(r#######"Removes a floating view previously added with [sim_floating_view_add](#method.sim_floating_view_add)."#######,sim_floating_view_remove,"floatingViewRemove",(floating_view_handle:i64)->i64),
(r#######""#######,sim_generate_shape_from_path,"generateShapeFromPath",(path:Vec<f64>,section:Vec<f64>),opt(options:i64,up_vector:Vec<f64>)->i64),
(r#######"Generates a 3D text model."#######,sim_generate_text_shape,"generateTextShape",(txt:String),opt(color:Vec<f64>,height:f64,centered:bool,alphabet_location:String)->i64),
(r#######"Generates a time optimal trajectory, based on the TOPPRA library"#######,sim_generate_time_optimal_trajectory,"generateTimeOptimalTrajectory",(path:Vec<f64>,path_lengths:Vec<f64>,min_max_vel:Vec<f64>,min_max_accel:Vec<f64>),opt(traj_pt_samples:i64,boundary_condition:String,timeout:f64)->(Vec<f64>,Vec<f64>)),
(r#######"Generates alternative manipulator configurations, for a same end-effector pose, for a manipulator
that has revolute joints with a range larger than 360 degrees. The original submitted configuration
will be part of the returned configurations"#######,sim_get_alternate_configs,"getAlternateConfigs",(joint_handles:Vec<i64>,input_config:Vec<f64>),opt(tip_handle:i64,low_limits:Vec<f64>,ranges:Vec<f64>)->Vec<f64>),
(r#######"Retrieves all API functions and variables that match a specific word.
Useful for script code auto-completion functionality"#######,sim_get_api_func,"getApiFunc",(script_handle:i64,api_word:String)->Vec<String>),
(r#######"Returns the call tip (or info text) for an API function"#######,sim_get_api_info,"getApiInfo",(script_handle:i64,api_word:String)->String),
(r#######"Deprecated. See properties instead"#######,sim_get_array_param,"getArrayParam",(parameter:i64)->Vec<f64>),
(r#######""#######,sim_get_auto_yield_delay,"getAutoYieldDelay"->f64),
(r#######"Deprecated. See properties instead"#######,sim_get_bool_param,"getBoolParam",(parameter:i64)->bool),
(r#######"Deprecated. See properties instead"#######,sim_get_buffer_signal,"getBufferSignal",(signal_name:String)->Vec<u8>),
(r#######"Returns the position or distance along a path that is closest to a specified point in space"#######,sim_get_closest_pos_on_path,"getClosestPosOnPath",(path:Vec<f64>,path_lengths:Vec<f64>,abs_pt:Vec<f64>)->f64),
(r#######"Retrieves the object handles that compose a collection
"#######,sim_get_collection_objects,"getCollectionObjects",(collection_handle:i64)->Vec<i64>),
(r#######"Returns the distance between two configurations points"#######,sim_get_config_distance,"getConfigDistance",(config_a:Vec<f64>,config_b:Vec<f64>),opt(metric:Vec<f64>,types:Vec<i64>)->f64),
(r#######"Retrieves contact point information of a dynamic simulation pass."#######,sim_get_contact_info,"getContactInfo",(dynamic_pass:i64,object_handle:i64,index:i64)->(Vec<i64>,Vec<f64>,Vec<f64>,Vec<f64>)),
(r#######"Deprecated. See properties instead"#######,sim_get_engine_bool_param,"getEngineBoolParam",(param_id:i64,object_handle:i64)->bool),
(r#######"Deprecated. See properties instead"#######,sim_get_engine_float_param,"getEngineFloatParam",(param_id:i64,object_handle:i64)->f64),
(r#######"Deprecated. See properties instead"#######,sim_get_engine_int32_param,"getEngineInt32Param",(param_id:i64,object_handle:i64)->i64),
(r#######"Retrieves the Euler angles from a transformation matrix"#######,sim_get_euler_angles_from_matrix,"getEulerAnglesFromMatrix",(matrix:Vec<f64>)->Vec<f64>),
(r#######"Retrieves the explicit handling flags for a scene object"#######,sim_get_explicit_handling,"getExplicitHandling",(object_handle:i64)->i64),
(r#######"Retrieves a string that describes additional environment or object properties, mainly used by extension plugins."#######,sim_get_extension_string,"getExtensionString",(object_handle:i64,index:i64),opt(key:String)->String),
(r#######"Deprecated. See properties instead"#######,sim_get_float_param,"getFloatParam",(parameter:i64)->f64),
(r#######"Retrieves all events that allow to reconstruct a scene's (mostly) visual content remotely"#######,sim_get_genesis_events,"getGenesisEvents"->Vec<serde_json::Value>),
(r#######""#######,sim_get_graph_curve,"getGraphCurve",(graph_handle:i64,graph_type:i64,curve_index:i64)->(String,i64,Vec<f64>,Vec<f64>,Vec<f64>,Vec<f64>,i64,i64)),
(r#######""#######,sim_get_graph_info,"getGraphInfo",(graph_handle:i64)->(i64,Vec<f64>,Vec<f64>)),
(r#######"Deprecated. See properties instead"#######,sim_get_int32_param,"getInt32Param",(parameter:i64)->i64),
(r#######""#######,sim_get_is_real_time_simulation,"getIsRealTimeSimulation"->i64),
(r#######"Retrieves joint dependency information, when the joint is in dependent mode"#######,sim_get_joint_dependency,"getJointDependency",(joint_handle:i64)->(i64,f64,f64)),
(r#######"Retrieves the force or torque applied to a joint along/about its active axis.
This function retrieves meaningful information only if the joint is prismatic or revolute,
and is dynamically enabled. With the Bullet, MuJoCo and Newton engine, this function returns
the force or torque applied to the joint motor  (torques from joint limits are not taken into account).
With the ODE and Vortex engine, this function returns the total force or torque applied to a joint
along/about its z-axis"#######,sim_get_joint_force,"getJointForce",(joint_handle:i64)->f64),
(r#######"Retrieves the interval parameters of a joint"#######,sim_get_joint_interval,"getJointInterval",(object_handle:i64)->(bool,Vec<f64>)),
(r#######"Retrieves the operation mode of a joint"#######,sim_get_joint_mode,"getJointMode",(joint_handle:i64)->(i64,i64)),
(r#######"Retrieves the linear/angular position of a joint. This function cannot be used with spherical joints
(use [sim_get_object_child_pose](#method.sim_get_object_child_pose) instead)"#######,sim_get_joint_position,"getJointPosition",(object_handle:i64)->f64),
(r#######"Retrieves the force or torque that a joint can exert"#######,sim_get_joint_target_force,"getJointTargetForce",(joint_handle:i64)->f64),
(r#######"Retrieves the target linear/angular position of a joint"#######,sim_get_joint_target_position,"getJointTargetPosition",(object_handle:i64)->f64),
(r#######"Retrieves the target linear/angular velocity of a non-spherical joint"#######,sim_get_joint_target_velocity,"getJointTargetVelocity",(object_handle:i64)->f64),
(r#######"Retrieves the type of a joint"#######,sim_get_joint_type,"getJointType",(object_handle:i64)->i64),
(r#######"Retrieves the linear or angular velocity of a joint. The velocity is a measured velocity
(i.e. from one simulation step to the next), and is available for all joints in the scene"#######,sim_get_joint_velocity,"getJointVelocity",(joint_handle:i64)->f64),
(r#######"Retrieves and clears the information string generated by last API call"#######,sim_get_last_info,"getLastInfo"->String),
(r#######"Deprecated. See properties instead"#######,sim_get_light_parameters,"getLightParameters",(light_handle:i64)->(i64,Vec<f64>,Vec<f64>,Vec<f64>)),
(r#######"Retrieves the object handle of the dummy linked to this one"#######,sim_get_link_dummy,"getLinkDummy",(dummy_handle:i64)->i64),
(r#######"Deprecated. See [sim_read_custom_data_tags](#method.sim_read_custom_data_tags) instead."#######,sim_get_matching_persistent_data_tags,"getMatchingPersistentDataTags",(pattern:String)->Vec<String>),
(r#######"Inverts a transformation matrix"#######,sim_get_matrix_inverse,"getMatrixInverse",(matrix:Vec<f64>)->Vec<f64>),
(r#######"Deprecated. See properties instead"#######,sim_get_model_property,"getModelProperty",(object_handle:i64)->i64),
(r#######"Deprecated. See properties instead"#######,sim_get_named_bool_param,"getNamedBoolParam",(name:String)->bool),
(r#######"Deprecated. See properties instead"#######,sim_get_named_float_param,"getNamedFloatParam",(name:String)->f64),
(r#######"Deprecated. See properties instead"#######,sim_get_named_int32_param,"getNamedInt32Param",(name:String)->i64),
(r#######"Deprecated. See properties instead"#######,sim_get_named_string_param,"getNamedStringParam",(param_name:String)->Vec<u8>),
(r#######"Retrieves the navigation and selection mode for the mouse"#######,sim_get_navigation_mode,"getNavigationMode"->i64),
(r#######"Retrieves an object handle based on its path and alias"#######,sim_get_object,"getObject",(path:String),opt(options:serde_json::Value)->i64),
(r#######"Retrieves the alias or path of an object based on its handle"#######,sim_get_object_alias,"getObjectAlias",(object_handle:i64),opt(options:i64)->String),
(r#######""#######,sim_get_object_alias_relative,"getObjectAliasRelative",(handle:i64,base_handle:i64),opt(options:i64)->String),
(r#######"Retrieves the handle of an object's child object"#######,sim_get_object_child,"getObjectChild",(object_handle:i64,index:i64)->i64),
(r#######"Retrieves the intrinsic or internal transformation of an object. For a joint, this is the transformation caused
by the joint movement, mainly. For joints and force sensors, this will also include a possible error
transformation caused by the physics engine (a physics engine can cause joints and force sensors to come apart,
when constraints can't be perfectly resolved)"#######,sim_get_object_child_pose,"getObjectChildPose",(object_handle:i64)->Vec<f64>),
(r#######"Retrieves the color of a scene object"#######,sim_get_object_color,"getObjectColor",(object_handle:i64,index:i64,color_component:i64)->Vec<f64>),
(r#######"Deprecated. See properties instead"#######,sim_get_object_float_array_param,"getObjectFloatArrayParam",(object_handle:i64,parameter_id:i64)->Vec<f64>),
(r#######"Deprecated. See properties instead"#######,sim_get_object_float_param,"getObjectFloatParam",(object_handle:i64,parameter_id:i64)->f64),
(r#######"Retrieves an object handle based on its unique identifier"#######,sim_get_object_from_uid,"getObjectFromUid",(uid:i64),opt(options:serde_json::Value)->()),
(r#######"Retrieves the zero-based position of an object among its siblings in the scene hierarchy"#######,sim_get_object_hierarchy_order,"getObjectHierarchyOrder",(object_handle:i64)->(i64,i64)),
(r#######"Deprecated. See properties instead"#######,sim_get_object_int32_param,"getObjectInt32Param",(object_handle:i64,parameter_id:i64)->i64),
(r#######"Retrieves the transformation matrix of an object"#######,sim_get_object_matrix,"getObjectMatrix",(object_handle:i64),opt(relative_to_object_handle:i64)->Vec<f64>),
(r#######"Retrieves the orientation (Euler angles) of an object"#######,sim_get_object_orientation,"getObjectOrientation",(object_handle:i64),opt(relative_to_object_handle:i64)->Vec<f64>),
(r#######"Retrieves the handle of an object's parent object"#######,sim_get_object_parent,"getObjectParent",(object_handle:i64)->i64),
(r#######"Retrieves the pose of an object"#######,sim_get_object_pose,"getObjectPose",(object_handle:i64),opt(relative_to_object_handle:i64)->Vec<f64>),
(r#######"Retrieves the position of an object"#######,sim_get_object_position,"getObjectPosition",(object_handle:i64),opt(relative_to_object_handle:i64)->Vec<f64>),
(r#######"Deprecated. See properties instead"#######,sim_get_object_property,"getObjectProperty",(object_handle:i64)->i64),
(r#######"Retrieves the quaternion of an object"#######,sim_get_object_quaternion,"getObjectQuaternion",(object_handle:i64),opt(relative_to_object_handle:i64)->Vec<f64>),
(r#######"Retrieves the handles of selected objects"#######,sim_get_object_sel,"getObjectSel"->Vec<i64>),
(r#######"Retrieves the size factor of a scene object. The size factor is different from the real object size.
Use this to be able to adapt to scaling operations"#######,sim_get_object_size_factor,"getObjectSizeFactor",(object_handle:i64)->f64),
(r#######"Deprecated. See properties instead"#######,sim_get_object_special_property,"getObjectSpecialProperty",(object_handle:i64)->i64),
(r#######"Deprecated. See properties instead"#######,sim_get_object_string_param,"getObjectStringParam",(object_handle:i64,parameter_id:i64)->Vec<u8>),
(r#######"Retrieves the type of an object"#######,sim_get_object_type,"getObjectType",(object_handle:i64)->i64),
(r#######"Retrieves the unique identifier of an object: throughout a CoppeliaSim session, there won't be two
identical unique identifiers. Unique identifiers are however not persistent (i.e. are not saved with the object)"#######,sim_get_object_uid,"getObjectUid",(object_handle:i64)->i64),
(r#######"Retrieves the linear and/or angular velocity of an object, in absolute coordinates. The velocity is
a measured velocity (i.e. from one simulation step to the next), and is available for all
objects in the scene"#######,sim_get_object_velocity,"getObjectVelocity",(object_handle:i64)->(Vec<f64>,Vec<f64>)),
(r#######"Retrieves object handles. Use this in a loop where index starts at 0 and is incremented to get all
object handles in the scene"#######,sim_get_objects,"getObjects",(index:i64,object_type:i64)->i64),
(r#######"Retrieves object handles in a given hierarchy tree"#######,sim_get_objects_in_tree,"getObjectsInTree",(tree_base_handle:i64),opt(object_type:i64,options:i64)->Vec<i64>),
(r#######"Retrieves voxel positions from an OC tree
"#######,sim_get_octree_voxels,"getOctreeVoxels",(octree_handle:i64)->Vec<f64>),
(r#######"Retrieves the current page index (view)"#######,sim_get_page,"getPage"->i64),
(r#######"Returns an interpolated configuration from a path"#######,sim_get_path_interpolated_config,"getPathInterpolatedConfig",(path:Vec<f64>,path_lengths:Vec<f64>,t:f64),opt(method:serde_json::Value,types:Vec<i64>)->Vec<f64>),
(r#######"Returns the lengths of a path in 1, 2 or 3D Cartesian space, even if more coordinates are
provided. Each path point will have a corresponding length value (taken as the distance from the
path's first point, along the path)"#######,sim_get_path_lengths,"getPathLengths",(path:Vec<f64>,dof:i64),opt(dist_callback:String)->(Vec<f64>,f64)),
(r#######"Deprecated. See [sim_read_custom_data_tags](#method.sim_read_custom_data_tags) instead."#######,sim_get_persistent_data_tags,"getPersistentDataTags"->Vec<String>),
(r#######"Returns auxiliary information about a loaded plugin"#######,sim_get_plugin_info,"getPluginInfo",(plugin_name:String,info_type:i64)->String),
(r#######"Retrieves a plugin name based on an index"#######,sim_get_plugin_name,"getPluginName",(index:i64)->String),
(r#######"Gets various properties of a point cloud
"#######,sim_get_point_cloud_options,"getPointCloudOptions",(point_cloud_handle:i64)->(f64,i64,i64,f64)),
(r#######"Retrieves point positions from a point cloud
"#######,sim_get_point_cloud_points,"getPointCloudPoints",(point_cloud_handle:i64)->Vec<f64>),
(r#######"Inverts a pose"#######,sim_get_pose_inverse,"getPoseInverse",(pose:Vec<f64>)->Vec<f64>),
(r#######"Generates a random value in the range between 0.0 and 1.0. The value is generated from an individual generator attached to the calling script"#######,sim_get_random,"getRandom",opt(seed:i64)->f64),
(r#######"Indicates whether the simulation is real-time"#######,sim_get_real_time_simulation,"getRealTimeSimulation"->bool),
(r#######"Retrieves a list of custom handles, linking a given scene object to other scene objects"#######,sim_get_referenced_handles,"getReferencedHandles",(object_handle:i64),opt(tag:String)->Vec<i64>),
(r#######"Retrieves a list of all referenced handles tags"#######,sim_get_referenced_handles_tags,"getReferencedHandlesTags",(object_handle:i64)->Vec<String>),
(r#######"Retrieves an axis and rotation angle that brings one pose or transformation matrix
onto another one. The translation part of the poses/matrices is ignored. This function,
when used in combination with [sim_rotate_around_axis](#method.sim_rotate_around_axis),
can be used to build interpolations between transformation matrices"#######,sim_get_rotation_axis,"getRotationAxis",(matrix_start:Vec<f64>,matrix_goal:Vec<f64>)->(Vec<f64>,f64)),
(r#######"Generates a scaled-up or scaled down version of the input image"#######,sim_get_scaled_image,"getScaledImage",(image_in:Vec<u8>,resolution_in:Vec<i64>,desired_resolution_out:Vec<i64>,options:i64)->(Vec<u8>,Vec<i64>)),
(r#######"Retrieves the handle of a script. For script objects, use [sim_get_object](#method.sim_get_object) instead"#######,sim_get_script,"getScript",(script_type:i64),opt(script_name:String)->i64),
(r#######"Retrieves a map of another script functions, that can be called"#######,sim_get_script_functions,"getScriptFunctions",(script_handle:i64)->serde_json::Value),
(r#######""#######,sim_get_setting_bool,"getSettingBool",(key:String)->bool),
(r#######""#######,sim_get_setting_float,"getSettingFloat",(key:String)->f64),
(r#######""#######,sim_get_setting_int32,"getSettingInt32",(key:String)->i64),
(r#######""#######,sim_get_setting_string,"getSettingString",(key:String)->String),
(r#######""#######,sim_get_shape_appearance,"getShapeAppearance",(handle:i64),opt(opts:serde_json::Value)->serde_json::Value),
(r#######"Returns the size and relative pose of a shape's bounding box"#######,sim_get_shape_bb,"getShapeBB",(shape_handle:i64)->(Vec<f64>,Vec<f64>)),
(r#######"Retrieves the color of a shape"#######,sim_get_shape_color,"getShapeColor",(shape_handle:i64,color_name:String,color_component:i64)->(i64,Vec<f64>)),
(r#######"Retrieves geometric information related to a shape"#######,sim_get_shape_geom_info,"getShapeGeomInfo",(shape_handle:i64)->(i64,i64,Vec<f64>)),
(r#######"Retrieves the inertia information from a shape"#######,sim_get_shape_inertia,"getShapeInertia",(shape_handle:i64)->(Vec<f64>,Vec<f64>)),
(r#######"Retrieves the mass of a shape"#######,sim_get_shape_mass,"getShapeMass",(shape_handle:i64)->f64),
(r#######"Retrieves a shape's mesh information"#######,sim_get_shape_mesh,"getShapeMesh",(shape_handle:i64)->(Vec<f64>,Vec<i64>,Vec<f64>)),
(r#######"Retrieves the texture ID of a texture that is applied to a specific shape"#######,sim_get_shape_texture_id,"getShapeTextureId",(shape_handle:i64)->i64),
(r#######"Retrieves a shape's visual information."#######,sim_get_shape_viz,"getShapeViz",(shape_handle:i64,item_index:i64)->serde_json::Value),
(r#######"Deprecated. See properties instead"#######,sim_get_signal_name,"getSignalName",(signal_index:i64,signal_type:i64)->String),
(r#######"Retrieves current simulation state
"#######,sim_get_simulation_state,"getSimulationState"->i64),
(r#######"Convenience function that returns true when the simulation is about to stop or stopped."#######,sim_get_simulation_stopping,"getSimulationStopping"->bool),
(r#######"Retrieves the current simulation time"#######,sim_get_simulation_time,"getSimulationTime"->f64),
(r#######"Retrieves the simulation time step (the simulation time (i.e. not real-time) that
passes at each main script simulation pass). This value might not be constant for a given simulation."#######,sim_get_simulation_time_step,"getSimulationTimeStep"->f64),
(r#######"Retrieves and removes the next message in the C/C++ or Lua message queues. Use this in a while-loop until
all messages have been extracted. While the C/C++ interface has one single message queue, each Lua script
has its own message queue. The C/C++ version of this function should only be called from the CoppeliaSim
client application. A given message queue cannot hold more than 64 messages, unread messages will be discarded."#######,sim_get_simulator_message,"getSimulatorMessage"->(i64,Vec<i64>,Vec<i64>)),
(r#######"Lua only. Retrieves and clears the last generated stack traceback for a script"#######,sim_get_stack_traceback,"getStackTraceback",opt(script_handle:i64)->String),
(r#######"Deprecated. See properties instead"#######,sim_get_string_param,"getStringParam",(parameter:i64)->String),
(r#######"Retrieves the system time."#######,sim_get_system_time,"getSystemTime"->f64),
(r#######"Retrieves the texture ID of a specific texture"#######,sim_get_texture_id,"getTextureId",(texture_name:String)->(i64,Vec<i64>)),
(r#######""#######,sim_get_thread_id,"getThreadId"->i64),
(r#######"Lua only. Returns all variables, except those set by CoppeliaSim."#######,sim_get_user_variables,"getUserVariables"->Vec<String>),
(r#######"Retrieves the linear and/or angular velocity of the center of mass of a non-static shape.
Data is provided by the selected physics engine"#######,sim_get_velocity,"getVelocity",(shape_handle:i64)->(Vec<f64>,Vec<f64>)),
(r#######"Reads the depth buffer of a vision sensor. The returned data doesn't make sense if
[sim_handle_vision_sensor](#method.sim_handle_vision_sensor) wasn't called previously"#######,sim_get_vision_sensor_depth,"getVisionSensorDepth",(sensor_handle:i64),opt(options:i64,pos:Vec<i64>,size:Vec<i64>)->(Vec<u8>,Vec<i64>)),
(r#######"Reads the image of a vision sensor. The returned data doesn't make sense if
[sim_handle_vision_sensor](#method.sim_handle_vision_sensor) wasn't called previously"#######,sim_get_vision_sensor_img,"getVisionSensorImg",(sensor_handle:i64),opt(options:i64,rgba_cut_off:f64,pos:Vec<i64>,size:Vec<i64>)->(Vec<u8>,Vec<i64>)),
(r#######"Returns the resolution of the vision sensor"#######,sim_get_vision_sensor_res,"getVisionSensorRes",(sensor_handle:i64)->()),
(r#######"Groups (or merges) several shapes into a compound shape (or simple shape)"#######,sim_group_shapes,"groupShapes",(shape_handles:Vec<i64>),opt(merge:bool)->i64),
(r#######"Lua only. Calls a specific function in add-ons.
Should only be called from the main script
"#######,sim_handle_add_on_scripts,"handleAddOnScripts",(call_type:i64)->i64),
(r#######"Handles the dynamics functionality in a scene"#######,sim_handle_dynamics,"handleDynamics",(delta_time:f64)->i64),
(r#######"Lua only. Calls a specific system callback function in
simulation scripts and customization scripts.
Simulation- and customization scripts will be executed in a precise order.
This function should only be called from the main script
"#######,sim_handle_embedded_scripts,"handleEmbeddedScripts",(call_type:i64)->i64),
(r#######"Handles the message pump for threaded scripts"#######,sim_handle_ext_calls,"handleExtCalls"->()),
(r#######"Handles a graph object (i.e. records another value for each curve, given that such value was provided
via [sim_set_graph_stream_value](#method.sim_set_graph_stream_value)
"#######,sim_handle_graph,"handleGraph",(object_handle:i64,simulation_time:f64)->()),
(r#######""#######,sim_handle_joint_motion,"handleJointMotion"->()),
(r#######"Handles (performs sensing, etc. of) a registered proximity sensor object"#######,sim_handle_proximity_sensor,"handleProximitySensor",(sensor_handle:i64)->(i64,f64,Vec<f64>,i64,Vec<f64>)),
(r#######"Lua only. Calls a specific function in the sandbox.
[sim_handle_sandbox_script](#method.sim_handle_sandbox_script) should only be called from the main script
"#######,sim_handle_sandbox_script,"handleSandboxScript",(call_type:i64)->()),
(r#######"Handles various functionality (e.g. camera tracking during simulation, object velocity calculation, etc.).
Should only be called from the main script, as the first instruction in
the sensing section"#######,sim_handle_sensing_start,"handleSensingStart"->()),
(r#######"Lua only. Calls a specific system callback function in
simulation scripts. Simulation scripts will be executed in a precise order.
This function should only be called from the main script."#######,sim_handle_simulation_scripts,"handleSimulationScripts",(call_type:i64)->i64),
(r#######"Initializes various functionality (e.g. camera tracking during simulation, object velocity calculation, etc.).
Should only be called from the main script, as the first instruction in the
initialization section"#######,sim_handle_simulation_start,"handleSimulationStart"->()),
(r#######"Handles (performs sensing, etc. of) a vision sensor object. It (1) clear previous computed image processing data, (2) reads
an image and (3) performs image processing via the vision callback functions
(if the vision sensor is using an external input only (1) is performed)"#######,sim_handle_vision_sensor,"handleVisionSensor",(sensor_handle:i64)->(i64,Vec<f64>,Vec<f64>)),
(r#######"Imports a mesh from a file"#######,sim_import_mesh,"importMesh",(fileformat:i64,path_and_filename:String,options:i64,identical_vertice_tolerance:f64,scaling_factor:f64)->(Vec<f64>,Vec<i64>)),
(r#######"Imports a shape from a file (first imports meshes, then groups/merges
them into a shape)"#######,sim_import_shape,"importShape",(fileformat:i64,path_and_filename:String,options:i64,identical_vertice_tolerance:f64,scaling_factor:f64)->i64),
(r#######"Initializes/reinitializes a script. Operates in an asynchronous way, and cannot be called from within the script being reinitialized"#######,sim_init_script,"initScript",(script_handle:i64)->()),
(r#######"Inserts an object into an OC tree, as voxels. Each voxel will store
a color and a tag value"#######,sim_insert_object_into_octree,"insertObjectIntoOctree",(octree_handle:i64,object_handle:i64,options:i64),opt(color:Vec<f64>,tag:i64)->i64),
(r#######"Inserts an object into a point cloud, as points"#######,sim_insert_object_into_point_cloud,"insertObjectIntoPointCloud",(point_cloud_handle:i64,object_handle:i64,options:i64,grid_size:f64),opt(color:Vec<f64>,duplicate_tolerance:f64)->i64),
(r#######"Inserts points into a point cloud
"#######,sim_insert_points_into_point_cloud,"insertPointsIntoPointCloud",(point_cloud_handle:i64,options:i64,points:Vec<f64>),opt(color:Vec<f64>,duplicate_tolerance:f64)->i64),
(r#######"Inserts voxels into an OC tree. Each voxel will store a color and a tag value"#######,sim_insert_voxels_into_octree,"insertVoxelsIntoOctree",(octree_handle:i64,options:i64,points:Vec<f64>),opt(color:Vec<f64>,tag:Vec<i64>)->i64),
(r#######"Computes the interpolated transformation matrix between matrixIn1 and matrixIn2. Quaternions are used internally"#######,sim_interpolate_matrices,"interpolateMatrices",(matrix_in1:Vec<f64>,matrix_in2:Vec<f64>,interpol_factor:f64)->Vec<f64>),
(r#######"Computes the interpolated pose between poseIn1 and poseIn2"#######,sim_interpolate_poses,"interpolatePoses",(pose_in1:Vec<f64>,pose_in2:Vec<f64>,interpol_factor:f64)->Vec<f64>),
(r#######"Removes points from a point cloud, that do not intersect with the
provided points (i.e. the result in the point cloud will be the intersection between the two sets of points).
When a point cloud doesn't use an OC tree calculation structure, then this operation cannot be performed"#######,sim_intersect_points_with_point_cloud,"intersectPointsWithPointCloud",(point_cloud_handle:i64,options:i64,points:Vec<f64>,tolerance:f64)->i64),
(r#######"Deprecated (yes, what an irony!). Returns 0"#######,sim_is_deprecated,"isDeprecated",(func_or_const:String)->i64),
(r#######"Checks whether a scene object is dynamically enabled, i.e. is being handled and simulated by the physics engine.
Note that until the physics engine has parsed the scene in the first simulation step (i.e. the first time
[sim_handle_dynamics](#method.sim_handle_dynamics) is called), no object will be dynamically enabled"#######,sim_is_dynamically_enabled,"isDynamicallyEnabled",(object_handle:i64)->bool),
(r#######"Checks whether a general object handle is still valid. When a general object is destroyed
(e.g. programmatically or via the user interface), then its related handle is not valid anymore
and will trigger an error when used. Use this function to avoid triggering an error"#######,sim_is_handle,"isHandle",(object_handle:i64)->bool),
(r#######"Launches an executable. Similar to os.execute or io.popen, but is system independent."#######,sim_launch_executable,"launchExecutable",(filename:String),opt(parameters:String,show_status:i64)->()),
(r#######"Loads an image from file or memory"#######,sim_load_image,"loadImage",(options:i64,filename:String)->(Vec<u8>,Vec<i64>)),
(r#######"Loads a previously saved model, and selects it"#######,sim_load_model,"loadModel",(filename:String)->i64),
(r#######"Loads a previously saved scene"#######,sim_load_scene,"loadScene",(filename:String)->()),
(r#######"Converts a transformation matrix to a pose"#######,sim_matrix_to_pose,"matrixToPose",(matrix:Vec<f64>)->Vec<f64>),
(r#######"Creates, modifies or destroys module menu entries. Those are user selectable items located in [Menu bar > Modules].
When selected, the corresponding script will have its sysCall_moduleEntry
callback function triggered, or sim_message_eventcallback_moduleentry triggered"#######,sim_module_entry,"moduleEntry",(handle:i64),opt(label:String,state:i64)->i64),
(r#######"Executes a n-DoF motion using the Ruckig online trajectory generator. This function can only be called from scripts running in a thread, since this is a blocking operation"#######,sim_move_to_config,"moveToConfig",(params:serde_json::Value)->serde_json::Value),
(r#######""#######,sim_move_to_config_cleanup,"moveToConfig_cleanup",(motion_object:serde_json::Value)->()),
(r#######""#######,sim_move_to_config_init,"moveToConfig_init",(params:serde_json::Value)->serde_json::Value),
(r#######""#######,sim_move_to_config_step,"moveToConfig_step",(motion_object:serde_json::Value)->(i64,serde_json::Value)),
(r#######"Executes an object motion using the Ruckig online trajectory generator, by performing interpolations between two poses. The function can operate by handling 4 motion variables (x,y,z and
angle between the two poses), or a single movement variable (t, which requires a metric to be specified for distance
calculation between the two poses). This function can only be called from a script running in a thread, since this is a blocking
operation"#######,sim_move_to_pose,"moveToPose",(params:serde_json::Value)->serde_json::Value),
(r#######""#######,sim_move_to_pose_cleanup,"moveToPose_cleanup",(motion_object:serde_json::Value)->()),
(r#######""#######,sim_move_to_pose_init,"moveToPose_init",(params:serde_json::Value)->serde_json::Value),
(r#######""#######,sim_move_to_pose_step,"moveToPose_step",(motion_object:serde_json::Value)->(i64,serde_json::Value)),
(r#######"Multiplies two transformation matrices"#######,sim_multiply_matrices,"multiplyMatrices",(matrix_in1:Vec<f64>,matrix_in2:Vec<f64>)->Vec<f64>),
(r#######"Multiplies two poses"#######,sim_multiply_poses,"multiplyPoses",(pose_in1:Vec<f64>,pose_in2:Vec<f64>)->Vec<f64>),
(r#######"Multiplies a vector with a pose or a matrix (e.g. v=m*v)"#######,sim_multiply_vector,"multiplyVector",(matrix:Vec<f64>,in_vectors:Vec<f64>)->Vec<f64>),
(r#######"Packs ab array of double floating-point numbers into a string"#######,sim_pack_double_table,"packDoubleTable",(double_numbers:Vec<f64>),opt(start_double_index:i64,double_count:i64)->Vec<u8>),
(r#######"Packs an array of floating-point numbers into a string"#######,sim_pack_float_table,"packFloatTable",(float_numbers:Vec<f64>),opt(start_float_index:i64,float_count:i64)->Vec<u8>),
(r#######"Packs an array of int32 numbers into a string"#######,sim_pack_int32_table,"packInt32Table",(int32_numbers:Vec<i64>),opt(start_int32_index:i64,int32_count:i64)->Vec<u8>),
(r#######"Packs a table into a buffer. The table may contain other nested arrays, maps, None/nil, bool,
number or string values. All other types (e.g. functions) will be considered as string or None/nil values.
You can also use [sim_pack_table](#method.sim_pack_table) to quickly compare two tables or to perform a deep copy of a table"#######,sim_pack_table,"packTable",(a_table:Vec<serde_json::Value>),opt(scheme:i64)->Vec<u8>),
(r#######"Packs an array of uint16 numbers into a string"#######,sim_pack_u_int16_table,"packUInt16Table",(uint16_numbers:Vec<i64>),opt(start_uint16_index:i64,uint16_count:i64)->Vec<u8>),
(r#######"Packs an array of uint32 numbers into a string"#######,sim_pack_u_int32_table,"packUInt32Table",(uint32_numbers:Vec<i64>),opt(start_u_int32_index:i64,uint32_count:i64)->Vec<u8>),
(r#######"Packs an array of uint8 numbers into a string"#######,sim_pack_u_int8_table,"packUInt8Table",(uint8_numbers:Vec<i64>),opt(start_uint8_index:i64,uint8count:i64)->Vec<u8>),
(r#######"Requests a pause of a simulation"#######,sim_pause_simulation,"pauseSimulation"->()),
(r#######"Deprecated. See [sim_read_custom_buffer_data](#method.sim_read_custom_buffer_data) instead."#######,sim_persistent_data_read,"persistentDataRead",(data_tag:String)->Vec<u8>),
(r#######"Deprecated. See [sim_write_custom_buffer_data](#method.sim_write_custom_buffer_data) instead."#######,sim_persistent_data_write,"persistentDataWrite",(data_tag:String,data_value:Vec<u8>),opt(options:i64)->()),
(r#######"Converts a pose to a transformation matrix"#######,sim_pose_to_matrix,"poseToMatrix",(pose:Vec<f64>)->Vec<f64>),
(r#######"Pushes a user-triggered event. Messages are received asynchronously via the
sysCall_event callback function and via the plugin
sim_message_eventcallback_events message call"#######,sim_push_user_event,"pushUserEvent",(event:String,handle:i64,uid:i64,event_data:serde_json::Value),opt(options:i64)->()),
(r#######"Triggers a quit signal after which the application eventually ends"#######,sim_quit_simulator,"quitSimulator"->()),
(r#######"Deprecated. See properties instead"#######,sim_read_custom_buffer_data,"readCustomBufferData",(object_handle:i64,tag_name:String)->Vec<u8>),
(r#######"Deprecated. See properties instead"#######,sim_read_custom_data_tags,"readCustomDataTags",(object_handle:i64)->Vec<String>),
(r#######"Deprecated. See properties instead"#######,sim_read_custom_string_data,"readCustomStringData",(object_handle:i64,tag_name:String)->String),
(r#######"Deprecated. See properties instead"#######,sim_read_custom_table_data,"readCustomTableData",(handle:i64,tag_name:String),opt(options:serde_json::Value)->serde_json::Value),
(r#######"Reads the force and torque applied to a force sensor (filtered values are read)"#######,sim_read_force_sensor,"readForceSensor",(object_handle:i64)->(i64,Vec<f64>,Vec<f64>)),
(r#######"Reads the state of a proximity sensor. This function doesn't perform detection,
it merely reads the result from a previous call to [sim_handle_proximity_sensor](#method.sim_handle_proximity_sensor)
"#######,sim_read_proximity_sensor,"readProximitySensor",(sensor_handle:i64)->(i64,f64,Vec<f64>,i64,Vec<f64>)),
(r#######"Retrieves the RGB data (or a portion of it) related to a specific texture"#######,sim_read_texture,"readTexture",(texture_id:i64,options:i64),opt(pos_x:i64,pos_y:i64,size_x:i64,size_y:i64)->Vec<u8>),
(r#######"Reads the state of a vision sensor. This function doesn't perform detection, it merely
reads the result from a previous call to [sim_handle_vision_sensor](#method.sim_handle_vision_sensor)
"#######,sim_read_vision_sensor,"readVisionSensor",(sensor_handle:i64)->(i64,Vec<f64>,Vec<f64>)),
(r#######"Refreshes CoppeliaSim's internal dialogs. Calling [sim_refresh_dialogs](#method.sim_refresh_dialogs) will
not trigger a sim.message_eventcallback_refreshdialogs message"#######,sim_refresh_dialogs,"refreshDialogs",(refresh_degree:i64)->i64),
(r#######"Counterpart function to [sim_acquire_lock](#method.sim_acquire_lock). Unlocking is cumulative"#######,sim_release_lock,"releaseLock"->()),
(r#######"Repositions and reorients the reference frame of a shape, while keeping the mesh
in place. The shape's inertia properties are unaffected."#######,sim_relocate_shape_frame,"relocateShapeFrame",(shape_handle:i64,pose:Vec<f64>)->i64),
(r#######"Removes a previously added drawing object"#######,sim_remove_drawing_object,"removeDrawingObject",(drawing_object_handle:i64)->()),
(r#######"Removes a model from the scene"#######,sim_remove_model,"removeModel",(object_handle:i64),opt(delayed_removal:bool)->i64),
(r#######"Removes one or several objects from the scene"#######,sim_remove_objects,"removeObjects",(object_handles:Vec<i64>),opt(delayed_removal:bool)->()),
(r#######"Removes a previously added particle object"#######,sim_remove_particle_object,"removeParticleObject",(particle_object_handle:i64)->()),
(r#######"Removes points from a point cloud. When a point cloud doesn't
use an OC tree calculation structure, then individual points cannot be removed,
only all points can be removed in that case"#######,sim_remove_points_from_point_cloud,"removePointsFromPointCloud",(point_cloud_handle:i64,options:i64,points:Vec<f64>,tolerance:f64)->i64),
(r#######""#######,sim_remove_referenced_objects,"removeReferencedObjects",(object_handle:i64),opt(tag:String)->()),
(r#######"Removes voxels from an OC tree
"#######,sim_remove_voxels_from_octree,"removeVoxelsFromOctree",(octree_handle:i64,options:i64,points:Vec<f64>)->i64),
(r#######"Returns a resampled path"#######,sim_resample_path,"resamplePath",(path:Vec<f64>,path_lengths:Vec<f64>,final_config_cnt:i64),opt(method:serde_json::Value,types:Vec<i64>)->Vec<f64>),
(r#######"Dynamically resets an object that is dynamically simulated. This means that the object representation
in the dynamics engine is removed, and added again. This can be useful when the set-up of a dynamically
simulated chain needs to be modified during simulation (e.g. joint or shape attachement position/orientation changed)"#######,sim_reset_dynamic_object,"resetDynamicObject",(object_handle:i64)->()),
(r#######"Resets a graph object (i.e. clears all its data streams)"#######,sim_reset_graph,"resetGraph",(object_handle:i64)->()),
(r#######"Clears the detection state, detection color, detection segments, etc. of a proximity sensor object"#######,sim_reset_proximity_sensor,"resetProximitySensor",(object_handle:i64)->()),
(r#######"Clears the detection state, etc. of a proximity sensor object"#######,sim_reset_vision_sensor,"resetVisionSensor",(sensor_handle:i64)->()),
(r#######"Restores the color of an entity, previously
modified with [sim_change_entity_color](#method.sim_change_entity_color)
"#######,sim_restore_entity_color,"restoreEntityColor",(original_color_data:Vec<serde_json::Value>)->()),
(r#######"Rotates a pose or transformation matrix around a random axis in space. This function,
when used in combination with [sim_get_rotation_axis](#method.sim_get_rotation_axis), can
be used to build interpolations between poses or transformation matrices"#######,sim_rotate_around_axis,"rotateAroundAxis",(matrix_in:Vec<f64>,axis:Vec<f64>,axis_pos:Vec<f64>,angle:f64)->Vec<f64>),
(r#######"Executes a call to the Ruckig online trajectory generator.
The Ruckig online trajectory generator provides instantaneous trajectory generation capabilities
for motion control systems. This function prepares a position-based trajectory generation object,
that can then be calculated with [sim_ruckig_step](#method.sim_ruckig_step). When this object
is not needed anymore, remove it with sim.ruckigRemove
"#######,sim_ruckig_pos,"ruckigPos",(dofs:i64,base_cycle_time:f64,flags:i64,current_pos_vel_accel:Vec<f64>,max_vel_accel_jerk:Vec<f64>,selection:Vec<i64>,target_pos_vel:Vec<f64>)->i64),
(r#######"Removes an object previously created via [sim_ruckig_pos](#method.sim_ruckig_pos)
or sim.ruckigVel."#######,sim_ruckig_remove,"ruckigRemove",(handle:i64)->()),
(r#######"Executes a call to the Ruckig online trajectory generator.
The Ruckig online trajectory generator provides instantaneous trajectory generation capabilities for
motion control systems. This function steps forward a trajectory generation algorithm previously prepared
via [sim_ruckig_pos](#method.sim_ruckig_pos) or sim.ruckigVel
"#######,sim_ruckig_step,"ruckigStep",(handle:i64,cycle_time:f64)->(i64,Vec<f64>,f64)),
(r#######"Executes a call to the Ruckig online trajectory generator.
The Ruckig online trajectory generator provides instantaneous trajectory generation capabilities for
motion control systems. This function prepares a velocity-based trajectory generation object,
that can then be calculated with [sim_ruckig_step](#method.sim_ruckig_step). When this object
is not needed anymore, remove it with sim.ruckigRemove
"#######,sim_ruckig_vel,"ruckigVel",(dofs:i64,base_cycle_time:f64,flags:i64,current_pos_vel_accel:Vec<f64>,max_accel_jerk:Vec<f64>,selection:Vec<i64>,target_vel:Vec<f64>)->i64),
(r#######"Saves an image to file or to memory"#######,sim_save_image,"saveImage",(image:Vec<u8>,resolution:Vec<i64>,options:i64,filename:String,quality:i64)->Vec<u8>),
(r#######"Saves a model (an object marked as "Object is model base" and all other
objects in its hierarchy tree). Any existing file with same name will be overwritten"#######,sim_save_model,"saveModel",(model_base_handle:i64,filename:String)->()),
(r#######"Saves a scene. Any existing file with same name will be overwritten"#######,sim_save_scene,"saveScene",(filename:String)->()),
(r#######"Scales specified objects in a non-isometric fashion, if possible. Only non-compound shapes
can be non-isometrically scaled. Some primitive shapes can have some constraints between their axes"#######,sim_scale_object,"scaleObject",(object_handle:i64,x_scale:f64,y_scale:f64,z_scale:f64),opt(options:i64)->()),
(r#######"Scales specified objects. All related values are automatically scaled appropriately
(e.g. masses, forces, etc.)"#######,sim_scale_objects,"scaleObjects",(object_handles:Vec<i64>,scaling_factor:f64,scale_positions_too:bool)->()),
(r#######"Reads how many bytes are waiting to be read on a serial port (RS-232)"#######,sim_serial_check,"serialCheck",(port_handle:i64)->i64),
(r#######"Closes a serial port (RS-232)"#######,sim_serial_close,"serialClose",(port_handle:i64)->()),
(r#######"Opens a serial port (RS-232) for communication. When called from a script,
the function can only be called when the simulation is running (and in that case the port is
automatically closed at simulation stop)"#######,sim_serial_open,"serialOpen",(port_string:String,baudrate:i64)->i64),
(r#######"Reads from a previously opened serial port (RS-232). The C version of the function cannot
be blocking"#######,sim_serial_read,"serialRead",(port_handle:i64,data_length_to_read:i64,blocking_operation:bool),opt(closing_string:Vec<u8>,timeout:f64)->Vec<u8>),
(r#######"Writes data to a previously opened serial port (RS-232)"#######,sim_serial_send,"serialSend",(port_handle:i64,data:Vec<u8>)->i64),
(r#######"Deprecated. See properties instead"#######,sim_set_array_param,"setArrayParam",(parameter:i64,array_of_values:Vec<f64>)->()),
(r#######"Allows specifying a thread interruption or yield delay, that will be
automatically enforced by the system (preemptive threading). By default this value is 2 ms"#######,sim_set_auto_yield_delay,"setAutoYieldDelay",(dt:f64)->()),
(r#######"Deprecated. See properties instead"#######,sim_set_bool_param,"setBoolParam",(parameter:i64,bool_state:bool)->()),
(r#######"Deprecated. See properties instead"#######,sim_set_buffer_signal,"setBufferSignal",(signal_name:String,signal_value:Vec<u8>)->()),
(r#######"Deprecated. See properties instead"#######,sim_set_engine_bool_param,"setEngineBoolParam",(param_id:i64,object_handle:i64,bool_param:bool)->()),
(r#######"Deprecated. See properties instead"#######,sim_set_engine_float_param,"setEngineFloatParam",(param_id:i64,object_handle:i64,float_param:f64)->()),
(r#######"Deprecated. See properties instead"#######,sim_set_engine_int32_param,"setEngineInt32Param",(param_id:i64,object_handle:i64,int32_param:i64)->()),
(r#######"Sets the explicit handling flags for a scene object"#######,sim_set_explicit_handling,"setExplicitHandling",(object_handle:i64,explicit_handling_flags:i64)->()),
(r#######"Deprecated. See properties instead"#######,sim_set_float_param,"setFloatParam",(parameter:i64,float_state:f64)->()),
(r#######"Deprecated. See properties instead"#######,sim_set_float_signal,"setFloatSignal",(signal_name:String,signal_value:f64)->()),
(r#######"Applies a transformation to a graph stream"#######,sim_set_graph_stream_transformation,"setGraphStreamTransformation",(graph_handle:i64,stream_id:i64,tr_type:i64),opt(mult:f64,off:f64,mov_avg_period:i64)->()),
(r#######"Sets the next value to be recorded for a graph stream"#######,sim_set_graph_stream_value,"setGraphStreamValue",(graph_handle:i64,stream_id:i64,value:f64)->()),
(r#######"Deprecated. See properties instead"#######,sim_set_int32_param,"setInt32Param",(parameter:i64,int_state:i64)->()),
(r#######"Deprecated. See properties instead"#######,sim_set_int32_signal,"setInt32Signal",(signal_name:String,signal_value:i64)->()),
(r#######"Sets a joint dependent of another joint. The dependent joint should first be set
into dependent mode via [sim_set_joint_mode](#method.sim_set_joint_mode)
"#######,sim_set_joint_dependency,"setJointDependency",(joint_handle:i64,master_joint_handle:i64,offset:f64,mult_coeff:f64)->()),
(r#######"Sets the interval parameters of a joint (i.e. range values). The attributes or
interval parameters might have no effect, depending on the joint-type"#######,sim_set_joint_interval,"setJointInterval",(object_handle:i64,cyclic:bool,interval:Vec<f64>)->()),
(r#######"Sets the operation mode of a joint"#######,sim_set_joint_mode,"setJointMode",(joint_handle:i64,joint_mode:i64,options:i64)->()),
(r#######"Sets the linear/angular position of a joint. Cannot be used with spherical joints
(use [sim_set_object_child_pose](#method.sim_set_object_child_pose) instead)"#######,sim_set_joint_position,"setJointPosition",(object_handle:i64,position:f64)->()),
(r#######"Sets the force or torque that a joint can exert"#######,sim_set_joint_target_force,"setJointTargetForce",(object_handle:i64,force_or_torque:f64),opt(signed_value:bool)->()),
(r#######"Sets the target linear/angular position of a joint. When in kinematic mode,
the joint moves according to a motion profile that respects maximum velocity, acceleration and jerk values.
In dynamic and position/custom control mode, the controller is instructed about the desired position"#######,sim_set_joint_target_position,"setJointTargetPosition",(object_handle:i64,target_position:f64),opt(motion_params:Vec<f64>)->()),
(r#######"Sets the target linear/angular velocity of a non-spherical joint. When in kinematic mode,
the joint moves according to a motion profile that respects maximum acceleration and
jerk values. In dynamic and velocity control mode, the controller is instructed about the
desired velocity"#######,sim_set_joint_target_velocity,"setJointTargetVelocity",(object_handle:i64,target_velocity:f64),opt(motion_params:Vec<f64>)->()),
(r#######"Deprecated. See properties instead"#######,sim_set_light_parameters,"setLightParameters",(light_handle:i64,state:i64,reserved:Vec<f64>,diffuse_part:Vec<f64>,specular_part:Vec<f64>)->()),
(r#######"Defines (or breaks) a dummy-dummy link pair. Useful to create dynamic loop closure
constraints on the fly (among others)"#######,sim_set_link_dummy,"setLinkDummy",(dummy_handle:i64,link_dummy_handle:i64)->()),
(r#######"Deprecated. See properties instead"#######,sim_set_model_property,"setModelProperty",(object_handle:i64,property:i64)->()),
(r#######""#######,sim_set_named_bool_param,"setNamedBoolParam",(name:String,value:bool)->()),
(r#######""#######,sim_set_named_float_param,"setNamedFloatParam",(name:String,value:f64)->()),
(r#######""#######,sim_set_named_int32_param,"setNamedInt32Param",(name:String,value:i64)->()),
(r#######"Deprecated. See properties instead"#######,sim_set_named_string_param,"setNamedStringParam",(param_name:String,string_param:Vec<u8>)->()),
(r#######"Sets the navigation and selection mode for the mouse"#######,sim_set_navigation_mode,"setNavigationMode",(navigation_mode:i64)->()),
(r#######"Sets the alias of an object"#######,sim_set_object_alias,"setObjectAlias",(object_handle:i64,object_alias:String)->()),
(r#######"Can be used to set a spherical joint's rotational transformation
(the translational part is ignored)"#######,sim_set_object_child_pose,"setObjectChildPose",(object_handle:i64,pose:Vec<f64>)->()),
(r#######"Sets the color of a scene object"#######,sim_set_object_color,"setObjectColor",(object_handle:i64,index:i64,color_component:i64,rgb_data:Vec<f64>)->bool),
(r#######"Deprecated. See properties instead"#######,sim_set_object_float_array_param,"setObjectFloatArrayParam",(object_handle:i64,parameter_id:i64,params:Vec<f64>)->()),
(r#######"Deprecated. See properties instead"#######,sim_set_object_float_param,"setObjectFloatParam",(object_handle:i64,parameter_id:i64,parameter:f64)->()),
(r#######"Moves an object up or down among its siblings in the scene hierarchy"#######,sim_set_object_hierarchy_order,"setObjectHierarchyOrder",(object_handle:i64,order:i64)->()),
(r#######"Deprecated. See properties instead"#######,sim_set_object_int32_param,"setObjectInt32Param",(object_handle:i64,parameter_id:i64,parameter:i64)->()),
(r#######"Sets the transformation matrix of an object. Dynamically simulated objects,
together with their hierarchy tree, are dynamically reset (this however does not apply to static shapes)"#######,sim_set_object_matrix,"setObjectMatrix",(object_handle:i64,matrix:Vec<f64>),opt(relative_to_object_handle:i64)->()),
(r#######"Sets the orientation (Euler angles) of an object.
Dynamically simulated objects, together with their hierarchy tree, are dynamically reset (this however does not
apply to static shapes)"#######,sim_set_object_orientation,"setObjectOrientation",(object_handle:i64,euler_angles:Vec<f64>),opt(relative_to_object_handle:i64)->()),
(r#######"Sets an object's parent object. Dynamically simulated objects, together with their
hierarchy tree, are dynamically reset (this however does not apply to static shapes)"#######,sim_set_object_parent,"setObjectParent",(object_handle:i64,parent_object_handle:i64),opt(keep_in_place:bool)->()),
(r#######"Sets the pose of an object. Dynamically simulated objects, together with their hierarchy tree,
are dynamically reset (this however does not apply to static shapes)"#######,sim_set_object_pose,"setObjectPose",(object_handle:i64,pose:Vec<f64>),opt(relative_to_object_handle:i64)->()),
(r#######"Sets the position (x, y and z-coordinates) of an object. Dynamically simulated objects,
together with their hierarchy tree, are dynamically reset (this however does not apply
to static shapes)"#######,sim_set_object_position,"setObjectPosition",(object_handle:i64,position:Vec<f64>),opt(relative_to_object_handle:i64)->()),
(r#######"Deprecated. See properties instead"#######,sim_set_object_property,"setObjectProperty",(object_handle:i64,property:i64)->()),
(r#######"Sets the quaternion of an object. Dynamically simulated objects, together with their
hierarchy tree, are dynamically reset (this however does not apply to static shapes)"#######,sim_set_object_quaternion,"setObjectQuaternion",(object_handle:i64,quaternion:Vec<f64>),opt(relative_to_object_handle:i64)->()),
(r#######"Sets the object selection state"#######,sim_set_object_sel,"setObjectSel",(object_handles:Vec<i64>)->()),
(r#######"Deprecated. See properties instead"#######,sim_set_object_special_property,"setObjectSpecialProperty",(object_handle:i64,property:i64)->()),
(r#######"Deprecated. See properties instead"#######,sim_set_object_string_param,"setObjectStringParam",(object_handle:i64,parameter_id:i64,parameter:Vec<u8>)->()),
(r#######"Switches between pages (main scene views)"#######,sim_set_page,"setPage",(page_index:i64)->()),
(r#######"Attaches additional information to a loaded plugin"#######,sim_set_plugin_info,"setPluginInfo",(plugin_name:String,info_type:i64,info:String)->()),
(r#######"Sets various properties of a point cloud
"#######,sim_set_point_cloud_options,"setPointCloudOptions",(point_cloud_handle:i64,max_voxel_size:f64,max_pt_cnt_per_voxel:i64,options:i64,point_size:f64)->()),
(r#######"Attaches a list of custom handles to a given scene object. Those custom handles
are handles of other scene objects, that are linked to the given scene object (for whatever purpose).
The advantage of storing references to other objects with this function is that CoppeliaSim will take
care of correctly adjusting the references if needed: For instance, imagine objectA storing
the handle of objectB via this function. If objectB is deleted, then the stored
handle becomes -1. If objectA and objectB are duplicated at the same time, then the
duplicate of objectA stores the handle of the duplicate of objectB. Optionally, if
[sim_handleflag](#method.sim_handleflag)_keeporiginal is specified, then linking to original objects is guaranteed, e.g. in above example,
after a duplication of objectA, the duplicate of objectA will store the handle of the
original objectB (if objectB still exists)"#######,sim_set_referenced_handles,"setReferencedHandles",(object_handle:i64,referenced_handles:Vec<i64>),opt(tag:String)->()),
(r#######""#######,sim_set_shape_appearance,"setShapeAppearance",(handle:i64,saved_data:serde_json::Value),opt(opts:serde_json::Value)->i64),
(r#######"Sets the size of a shape's bounding box, effectively scaling the shape.
Non-isometric scaling is not always possible"#######,sim_set_shape_bb,"setShapeBB",(shape_handle:i64,size:Vec<f64>)->()),
(r#######"Sets the color of a shape"#######,sim_set_shape_color,"setShapeColor",(shape_handle:i64,color_name:String,color_component:i64,rgb_data:Vec<f64>)->()),
(r#######"Applies a new inertia matrix to a shape. If simulation is running, the shape is
dynamically reset (similar to calling [sim_reset_dynamic_object](#method.sim_reset_dynamic_object) right after)"#######,sim_set_shape_inertia,"setShapeInertia",(shape_handle:i64,inertia_matrix:Vec<f64>,com_matrix:Vec<f64>)->()),
(r#######"Applies a new mass value to a shape. If simulation is running, the shape is dynamically reset
(similar to calling [sim_reset_dynamic_object](#method.sim_reset_dynamic_object) right after)"#######,sim_set_shape_mass,"setShapeMass",(shape_handle:i64,mass:f64)->()),
(r#######"Deprecated"#######,sim_set_shape_material,"setShapeMaterial",(shape_handle:i64,material_id_or_shape_handle:i64)->()),
(r#######"Applies/removes a texture to/from a shape"#######,sim_set_shape_texture,"setShapeTexture",(shape_handle:i64,texture_id:i64,mapping_mode:i64,options:i64,uv_scaling:Vec<f64>),opt(position:Vec<f64>,orientation:Vec<f64>)->()),
(r#######"Enables or disables the stepping operation mode for a threaded script. If enabled, then the current script has
to trigger each simulation step explicitly, via [sim_step](#method.sim_step). Is applied
cumulatively, i.e. if the stepping operation mode is enabled n times, it needs to be disabled n times to return
to the initial state


(Lua specific: in stepping operation mode, automatic thread interruptions, i.e. preemptive threading, is supressed)"#######,sim_set_stepping,"setStepping",(enabled:bool)->i64),
(r#######"Deprecated. See properties instead"#######,sim_set_string_param,"setStringParam",(parameter:i64,string_state:String)->()),
(r#######"Deprecated. See properties instead"#######,sim_set_string_signal,"setStringSignal",(signal_name:String,signal_value:String)->()),
(r#######"Writes the image of a vision sensor (and applies any image processing via the
vision callback functions). Make sure the vision
sensor is flagged as external input
"#######,sim_set_vision_sensor_img,"setVisionSensorImg",(sensor_handle:i64,image:Vec<u8>),opt(options:i64,pos:Vec<i64>,size:Vec<i64>)->()),
(r#######"Requests a start of a simulation (or a resume of a paused simulation)"#######,sim_start_simulation,"startSimulation"->()),
(r#######"Triggers the next simulation step, when in stepping operation mode. When simulation is running, then [sim_step](#method.sim_step)
only returns once the simulation time has changed"#######,sim_step,"step"->()),
(r#######"Requests a stop of the running simulation"#######,sim_stop_simulation,"stopSimulation",opt(wait:bool)->()),
(r#######"Removes an object from an OC tree, as voxel subtractions"#######,sim_subtract_object_from_octree,"subtractObjectFromOctree",(octree_handle:i64,object_handle:i64,options:i64)->i64),
(r#######"Removes an object from a point cloud, as a subtraction"#######,sim_subtract_object_from_point_cloud,"subtractObjectFromPointCloud",(point_cloud_handle:i64,object_handle:i64,options:i64,tolerance:f64)->i64),
(r#######""#######,sim_test_cb,"testCB",(a:i64,cb:String,b:i64)->i64),
(r#######"Closes a text edition window previously opened with [sim_text_editor_open](#method.sim_text_editor_open)
"#######,sim_text_editor_close,"textEditorClose",(handle:i64)->(String,Vec<i64>,Vec<i64>)),
(r#######"Retieves information from a text edition window previously opened with [sim_text_editor_open](#method.sim_text_editor_open)
"#######,sim_text_editor_get_info,"textEditorGetInfo",(handle:i64)->(String,Vec<i64>,Vec<i64>,bool)),
(r#######"Opens a text edition window"#######,sim_text_editor_open,"textEditorOpen",(init_text:String,properties:String)->i64),
(r#######"Shows or hides a text edition window previously opened with [sim_text_editor_open](#method.sim_text_editor_open)
"#######,sim_text_editor_show,"textEditorShow",(handle:i64,show_state:bool)->()),
(r#######"Modifies a buffer than contains packed data"#######,sim_transform_buffer,"transformBuffer",(in_buffer:Vec<u8>,in_format:i64,multiplier:f64,offset:f64,out_format:i64)->Vec<u8>),
(r#######"Transforms an image in various ways"#######,sim_transform_image,"transformImage",(image:Vec<u8>,resolution:Vec<i64>,options:i64)->()),
(r#######"Ungroups a compound shape into several shapes
"#######,sim_ungroup_shape,"ungroupShape",(shape_handle:i64)->Vec<i64>),
(r#######"Unpacks a string (or part of it) into an array of double floating-point numbers"#######,sim_unpack_double_table,"unpackDoubleTable",(data:Vec<u8>),opt(start_double_index:i64,double_count:i64,additional_byte_offset:i64)->Vec<f64>),
(r#######"Unpacks a string (or part of it) into an array of floating-point numbers"#######,sim_unpack_float_table,"unpackFloatTable",(data:Vec<u8>),opt(start_float_index:i64,float_count:i64,additional_byte_offset:i64)->Vec<f64>),
(r#######"Unpacks a string (or part of it) into an array of int32 numbers"#######,sim_unpack_int32_table,"unpackInt32Table",(data:Vec<u8>),opt(start_int32_index:i64,int32_count:i64,additional_byte_offset:i64)->Vec<i64>),
(r#######"Unpacks a buffer into a table"#######,sim_unpack_table,"unpackTable",(buffer:Vec<u8>)->serde_json::Value),
(r#######"Unpacks a string (or part of it) into an array of uint16 numbers"#######,sim_unpack_u_int16_table,"unpackUInt16Table",(data:Vec<u8>),opt(start_uint16_index:i64,uint16_count:i64,additional_byte_offset:i64)->Vec<i64>),
(r#######"Unpacks a string (or part of it) into an array of uint32 numbers"#######,sim_unpack_u_int32_table,"unpackUInt32Table",(data:Vec<u8>),opt(start_uint32_index:i64,uint32_count:i64,additional_byte_offset:i64)->Vec<i64>),
(r#######"Unpacks a string (or part of it) into an array of uint8 numbers"#######,sim_unpack_u_int8_table,"unpackUInt8Table",(data:Vec<u8>),opt(start_uint8_index:i64,uint8count:i64)->Vec<i64>),
(r#######""#######,sim_visit_tree,"visitTree",(root_handle:i64,visitor_func:String),opt(options:serde_json::Value)->()),
(r#######"Waits for a certain amount of time"#######,sim_wait,"wait",(dt:f64),opt(simulation_time:bool)->f64),
(r#######"Deprecated. See properties instead"#######,sim_wait_for_signal,"waitForSignal",(sig_name:String)->serde_json::Value),
(r#######"Deprecated. See properties instead"#######,sim_write_custom_buffer_data,"writeCustomBufferData",(object_handle:i64,tag_name:String,data:Vec<u8>)->()),
(r#######"Deprecated. See properties instead"#######,sim_write_custom_string_data,"writeCustomStringData",(object_handle:i64,tag_name:String,data:String)->()),
(r#######"Deprecated. See properties instead"#######,sim_write_custom_table_data,"writeCustomTableData",(handle:i64,tag_name:String,the_table:serde_json::Value),opt(options:serde_json::Value)->()),
(r#######"Overwrites a specific texture (or a portion of it) with RGB data"#######,sim_write_texture,"writeTexture",(texture_id:i64,options:i64,texture_data:Vec<u8>),opt(pos_x:i64,pos_y:i64,size_x:i64,size_y:i64,interpol:f64)->()),
(r#######"Converts Yaw-Pitch-Roll angles to CoppeliaSim's alpha-beta-gamma angles"#######,sim_yaw_pitch_roll_to_alpha_beta_gamma,"yawPitchRollToAlphaBetaGamma",(yaw_angle:f64,pitch_angle:f64,roll_angle:f64)->(f64,f64,f64)),
(r#######""#######,sim_yield,"yield"->())
}
}