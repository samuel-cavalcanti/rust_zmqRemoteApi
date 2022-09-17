mod api_objects;
pub mod remote_api_objects_const;
mod supported_types;
pub use api_objects::RemoteAPIObjects;

/*
   this suite test mocks the zmqClient, so it's possible
    test the API without open the simulator.
*/
#[cfg(test)]
mod tests;
