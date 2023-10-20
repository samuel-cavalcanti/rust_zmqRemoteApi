//! All the Supported Plugins is insite this module
//! Current only the Remote API and SimIK is Supported

#[rustfmt::skip]
mod sim_api;
#[rustfmt::skip]
mod sim_const;
#[rustfmt::skip]
mod sim_ik_api;

pub use sim_api::Sim;
pub use sim_const::*;
pub use sim_ik_api::SimIK;

/// The Supported extra plugins
pub enum Plugin {
    SimIK,
}

/*
   this suite test mocks the zmqClient, so it's possible
test the API without open the simulator.
*/
#[cfg(test)]
mod tests;
