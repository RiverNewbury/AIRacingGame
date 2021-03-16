//! The implementation of the AI-Racing server
//!
//! This contains the functionality for:
//! * Receiving & executing user scripts;
//! * Simulating the car's run around a racetrack;
//! * Sending back the full race & time; and
//! * Displaying the leaderboad upon request

#![feature(decl_macro)]

use rocket::get;
use rocket_contrib::json::Json;

mod code;
mod sim;

use sim::SimulationHistory;

#[get("/run/<username>", data = "<code>")]
fn exec_user_code(username: String, code: String) -> Json<SimulationHistory> {
    todo!()
}

fn main() {
    println!("Hello, world!");
}
