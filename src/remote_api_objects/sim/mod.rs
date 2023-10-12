#[rustfmt::skip]
mod sim_api;

#[rustfmt::skip]
mod sim_const;

#[rustfmt::skip]
mod sim_ik_api;

pub use sim_api::Sim;
pub use sim_const::*;
pub use sim_ik_api::SimIK;

pub enum Module {
    SimIK,
}

/*
   this suite test mocks the zmqClient, so it's possible
test the API without open the simulator.
*/
#[cfg(test)]
mod tests;
