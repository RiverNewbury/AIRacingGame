//! The implementation of the AI-Racing server
//!
//! This contains the functionality for:
//! * Receiving & executing user scripts;
//! * Simulating the car's run around a racetrack;
//! * Sending back the full race & time; and
//! * Displaying the leaderboard upon request

#![feature(decl_macro)]

use lazy_static::lazy_static;
use rocket::response::status::BadRequest;
use rocket::{get, post, routes};
use rocket::http::RawStr;
use rocket_contrib::json::Json;
use std::sync::Mutex;

mod code;
mod leaderboard;
mod sim;

use code::Code;
use leaderboard::{Leaderboard, LeaderboardEntry};
use sim::{Racetrack, Simulation, SimulationData};

lazy_static! {
    static ref LEADERBOARD: Mutex<Leaderboard> = Mutex::new(Leaderboard::new());
    static ref RACETRACK: Racetrack = Racetrack::from_str(include_str!("default-racetrack.rtk"))
        .expect("failed to make initial racetrack");
}

type RequestResult<T> = Result<Json<T>, BadRequest<String>>;

#[post("/run/<username>", data = "<source_code_raw>")]
fn exec_user_code(username: String, source_code_raw: String) -> RequestResult<SimulationData> {
    let source_code : String = (RawStr::from_str(&source_code_raw)).url_decode_lossy();

    let code = Code::from_str(&source_code).map_err(|e| BadRequest(Some(e)))?;

    let (score, history) = (Simulation::new(code, &RACETRACK))
        .simulate()
        .map_err(|e| BadRequest(Some(e)))?;

    // Add the result of the simulation to the leaderboard
    LEADERBOARD
        .lock()
        .expect("leaderboard mutex already poisoned!")
        .add(username, source_code, score);

    Ok(Json(SimulationData { history, score }))
}

#[get("/leaderboard/<n>")]
fn get_leaderboard(n: usize) -> RequestResult<Vec<LeaderboardEntry>> {
    let lb_guard = LEADERBOARD.lock().unwrap();
    let entries: Vec<_> = lb_guard.top_n(n).collect();
    drop(lb_guard);

    Ok(Json(entries))
}

#[get("/tileinfo/<x>/<y>")]
fn get_tile_info(x: f32, y: f32) -> RequestResult<sim::GridTile> {
    if x >= RACETRACK.width as f32 || y >= RACETRACK.height as f32 {
        return Err(BadRequest(Some("coordinates out of bounds".to_owned())));
    }

    let point = sim::Point { x, y };
    Ok(Json(RACETRACK.get_tile(point)))
}

fn main() {
    lazy_static::initialize(&RACETRACK);
    lazy_static::initialize(&LEADERBOARD);

    rocket::ignite()
        .mount("/", routes![exec_user_code, get_leaderboard, get_tile_info])
        .launch();
}
