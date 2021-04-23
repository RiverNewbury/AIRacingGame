//! Wrapper module for parsing and executing user-submitted code

pub use crate::sim::{Point,Car};

// TODO: This module is currently a skeleton, yet to be implemented

/// User-submitted code - parsed, checked and ready to be executed
pub struct Code; // TODO

//TODO: Work out how code simulation actually works
pub struct Output {
    pub acc: f32, // fraction of how much the pedal is down - Between -1 and 1 negative being breaking
    pub turning_speed: f32, // Speed in degree/tick to turn
}

/// The execution environment for user-submitted code, providing information about the state of the
/// car in its race
// TODO - Consider if this is actually what we want
pub struct ExecEnvironment {
    pub car_currently : Car, // Gives current information about the car
    pub dist_to_wall : Vec<f32> // Gives you the distance to the wall at regular intervals of angle starting from 0
    // IE if there were 2 elements that'd mean one at 0 deg and one at 180 deg
}


impl Code {
    /// Parses the user's code, returning any error as a string if there was one
    pub fn from_str(input: &str) -> Result<Code, String> {
        todo!()
    }

    /// Execute's the users's code within the given race environment, returning the output as an
    /// in-Rust directive for the car's movement
    pub fn execute(&self, env: &ExecEnvironment) -> Result<Output, String> {
        todo!()
    }
}
