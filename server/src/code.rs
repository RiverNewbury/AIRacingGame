//! Wrapper module for parsing and executing user-submitted code
//!
//! Known issues:
//! * Only one Python instance can run at a time -- this is actually an issue within Python itself,
//!   not PyO3.
//! * Detecting timeouts is not always guaranteed. It causes an exception, but it's possible for
//!   user code to handle that
//! * User code is not restricted in what it does; filesystem access, for example, is allowed

use pyo3::class::basic::PyObjectProtocol;
use pyo3::prelude::*;

pub use crate::sim::{Car, Point};

/// User-submitted code
///
/// We *would* like to represent this as the Python function, but there's scoping issues when we do
/// that (e.g. "`CarCommand` is not in scope" errors). Storing it inside of a module somehow
/// guarantees that they stay in scope, so we use that instead.
pub struct Code {
    code_module: Py<PyModule>,
}

//TODO: Work out how code simulation actually works
#[derive(FromPyObject)]
pub struct Output {
    pub acc: f32, // fraction of how much the pedal is down - Between -1 and 1 negative being breaking
    /// A value between -1 and 1 indicating how the car should turn. -1 is as left as possible, and
    /// 1 is as right as possible.
    pub turning_speed: f32,
}

/// The execution environment for user-submitted code, providing information about the state of the
/// car in its race
// TODO - Consider if this is actually what we want
#[pyclass]
#[derive(Clone, Debug)]
pub struct ExecEnvironment {
    #[pyo3(get)]
    pub car_currently: Car, // Gives current information about the car
    #[pyo3(get)]
    pub dist_to_wall: Vec<f32>, // Gives you the distance to the wall at regular intervals of angle starting from 0
                                // IE if there were 2 elements that'd mean one at 0 deg and one at 180 deg
}

macro_rules! impl_format {
    ($($ty:ty),*) => {
        $(
        #[pyproto]
        impl pyo3::class::basic::PyObjectProtocol for $ty {
            fn __str__(&self) -> PyResult<String> {
                Ok(format!("{:?}", self))
            }

            fn __format__(&self, _format_spec: &str) -> PyResult<String> {
                Ok(format!("{:?}", self))
            }
        }
        )*
    }
}

impl_format!(ExecEnvironment, Car, Point);

impl Code {
    /// Parses the user's code, returning any error as a string if there was one
    pub fn from_str(input: &str) -> Result<Code, String> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let code_module =
            PyModule::from_code(py, input, "code.py", "code").map_err(|e| e.to_string())?;

        let func = code_module.getattr("outputs").map_err(|e| e.to_string())?;

        if !func.is_callable() {
            return Err("expected a function `outputs`".into());
        }

        Ok(Code {
            code_module: code_module.into(),
        })
    }

    /// Execute's the users's code within the given race environment, returning the output as an
    /// in-Rust directive for the car's movement
    pub fn execute(&self, env: &ExecEnvironment) -> Result<Output, String> {
        Python::with_gil(|py| {
            let module = self.code_module.as_ref(py);
            let func = module.getattr("outputs").map_err(|e| e.to_string())?;

            let output = func.call1((env.clone(),)).map_err(|e| e.to_string())?;
            <Output as FromPyObject>::extract(output)
                .map_err(|e| format!("unable to read user output: {}", e))
        })
    }
}
