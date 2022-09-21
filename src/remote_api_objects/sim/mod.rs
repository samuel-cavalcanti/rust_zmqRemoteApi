mod sim;
mod sim_const;

pub use sim::Sim;
pub use sim_const::*;


/*
   this suite test mocks the zmqClient, so it's possible
test the API without open the simulator.
*/
#[cfg(test)]
mod tests;
