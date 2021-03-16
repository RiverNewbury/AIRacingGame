//! This is the module which deals with user code and also simulates the car around the racetrack

use racetrack::Racetrack;

// A tick is the unit on which thte simulation will update the world
const ticksPerSecond: i32 = 100;
// The number of ticks until the users code will be asked what it wants to do next
const ticksPerUpdate: i32 = 10;

// Almost all the computation will be done in the Simulation Object

struct Simulation {
    id: Int, //For keeping track of which call should be returned to who
    code: String //TODO: Work out how code simulation actually works
    track: Racetrack
}

impl Simulation {
    fn simulate
}
