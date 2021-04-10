//! The implementation of the AI-Racing server
//!
//! This contains the functionality for:
//! * Receiving & executing user scripts;
//! * Simulating the car's run around a racetrack;
//! * Sending back the full race & time; and
//! * Displaying the leaderboad upon request

#![feature(decl_macro)]

use lazy_static::lazy_static;
use rocket::response::status::BadRequest;
use rocket::{get, routes};
use rocket_contrib::json::Json;
use std::sync::Mutex;

mod code;
mod leaderboard;
mod sim;

use code::Code;
use leaderboard::Leaderboard;
use sim::{Racetrack, Simulation, SimulationHistory};

//For exResults
use sim::{Point, Score, Car};

lazy_static! {
    static ref LEADERBOARD: Mutex<Leaderboard> = Mutex::new(Leaderboard::new());
    static ref RACETRACK: Racetrack = Racetrack::from_str(include_str!("default-racetrack.rtk"))
        .expect("failed to make initial racetrack");
}

type RequestResult<T> = Result<Json<T>, BadRequest<String>>;

#[get("/run/<username>", data = "<source_code>")]
fn exec_user_code(username: String, source_code: String) -> RequestResult<SimulationHistory> {
    let code = Code::from_str(&source_code).map_err(|e| BadRequest(Some(e)))?;
    //Not able to be parallel with just 1 as id
    //TODO - Get lazy static working rather than defing again
    let r: Racetrack = Racetrack::from_str(include_str!("default-racetrack.rtk")).expect("default racetrack is wrong");
    let (score, history) = (Simulation::new(1, code, r))
        .simulate()
        .map_err(|e| BadRequest(Some(e)))?;

    // Add the result of the simulation to the leaderboard
    LEADERBOARD
        .lock()
        .expect("leaderboard mutex already poisoned!")
        .add(username, source_code, score);

    Ok(Json(history))
}

fn main() {
    lazy_static::initialize(&RACETRACK);
    lazy_static::initialize(&LEADERBOARD);
//    ex_result()


    rocket::ignite()
        .mount("/", routes![exec_user_code])
        .launch();
}

fn ex_result() {
    let s = Score {
        successful: true,
        time : 129,
    };

    let base_car = Car {
        pos: Point{x:1.0,y:1.0},
        angle: 0.0,
        speed: 0.0,
        max_speed: 1.0,
        max_acc: 1.0,
        max_dec: 1.0,
    };

    let h = SimulationHistory {
        history: vec![
            base_car,

            Car {
                pos: Point{x:1.5, y:1.5},
                angle: 45.0,
                speed: 3.0,
                ..base_car
            },

            Car {
                pos: Point{x:3.5, y:3.5},
                angle: 90.0,
                speed: 12.0,
                ..base_car
            },
        ],
        tps: 100,
    };

    let a = Json(h);

    print!("{:?}", a)
}
