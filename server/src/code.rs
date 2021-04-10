//! Wrapper module for parsing and executing user-submitted code

// TODO: This module is currently a skeleton, yet to be implemented

/// User-submitted code - parsed, checked and ready to be executed
pub struct Code; // TODO

//TODO: Work out how code simulation actually works
pub struct Output {
    pub acc: f32, // fraction of how much the pedal is down - Between -1 and 1 negative being breaking
    pub turning_speed: f32, // Speed in rad/tick to turn
}

/// The execution environment for user-submitted code, providing information about the state of the
/// car in its race
pub struct ExecEnvironment; // TODO

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
