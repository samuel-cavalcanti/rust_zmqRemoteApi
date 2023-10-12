mod sim_api;
mod sim_const;
mod sim_ik_api;

pub use sim_api::Sim;
pub use sim_ik_api::SimIk;
pub use sim_const::*;

/*
   this suite test mocks the zmqClient, so it's possible
test the API without open the simulator.
*/
#[cfg(test)]
mod tests;
