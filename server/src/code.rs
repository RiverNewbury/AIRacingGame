//! Wrapper module for parsing and executing user-submitted code
//!
//! Known issues:
//! * Only one Python instance can run at a time -- this is actually an issue within Python itself,
//!   not PyO3.
//! * Detecting timeouts is not always guaranteed. It causes an exception, but it's possible for
//!   user code to handle that
//! * User code is not restricted in what it does; filesystem access, for example, is allowed

use pyo3::prelude::FromPyObject;
use pyo3::types::PyDict;
use pyo3::Python;

pub use crate::sim::{Point,Car};

// TODO: This module is currently a skeleton, yet to be implemented

/// User-submitted code
///
/// For Python code, this is parsed each time it is used
pub struct Code {
    code: String,
}

//TODO: Work out how code simulation actually works
#[derive(FromPyObject)]
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
        let code = format! {
r#"
def __user_main(env):
    {}

    return outputs(env)

# TODO: properly set environment
# res = __user_main(__env)
res = __user_main(None)
"#,
            input,
        };

        Ok(Code { code })
    }

    /// Execute's the users's code within the given race environment, returning the output as an
    /// in-Rust directive for the car's movement
    pub fn execute(&self, env: &ExecEnvironment) -> Result<Output, String> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let locals = PyDict::new(py);
        // TODO: Set the environment
        // locals.set_item("__env", env);

        // The result from `Python::run` is a PyResult<()> -- the actual user output is instead
        // given by the `res` entry in `locals`.
        py.run(&self.code, None, Some(locals))
            .map_err(|e| e.to_string())?;

        let output = locals
            .get_item("res")
            .ok_or_else(|| "unexpected lack of output from user code".to_owned())?;

        // This doesn't need to be given as explicitly, but it's nice to see what's going on
        <Output as FromPyObject>::extract(output)
            .map_err(|e| format!("unable to read user output: {}", e))
    }
}
