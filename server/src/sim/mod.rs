//! This is the module which deals with user code and also simulates the car around the racetrack

use crate::code::Code;
use serde::Serialize;

mod racetrack;

pub use racetrack::Racetrack;

// A tick is the unit on which thte simulation will update the world
const TICKS_PER_SECOND: i32 = 100;
// The number of ticks until the users code will be asked what it wants to do next
const TICKS_PER_UPDATE: i32 = 10;

// Almost all the computation will be done in the Simulation Object

pub struct Simulation {
    id: usize,  //For keeping track of which call should be returned to who
    code: Code, //TODO: Work out how code simulation actually works
    track: Racetrack,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Score; // TODO

#[derive(Serialize)]
pub struct SimulationHistory; // TODO

impl Simulation {
    pub fn simulate(self) -> Result<(Score, SimulationHistory), String> {
        todo!()
    }
}
