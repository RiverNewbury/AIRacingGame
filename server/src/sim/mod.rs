use crate::code::{Code, ExecEnvironment};
use serde::Serialize;
use std::cmp::max;

mod point;
mod racetrack;

pub use point::Point;
pub use racetrack::{Car, GridTile, Racetrack};

// A tick is the unit on which thte simulation will update the world
const TICKS_PER_SECOND: i32 = 100;
// The number of ticks until the users code will be asked what it wants to do next
const TICKS_PER_UPDATE: i32 = 10;
// The number of checks along a line that the car travels to make sure it never goes out of bounds
const NUMBER_CHECKS: i32 = 10;

// Almost all the computation will be done in the Simulation Object

pub struct Simulation {
    id: usize, //For keeping track of which call should be returned to who
    code: Code,
    track: Racetrack,
    car: Car,
}

//TODO - Made field public for score + sim hist pub for ex result

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Score {
    pub successful: bool,
    pub time: i32, // In terms of ticks
}

// TODO - added debug for ex result
#[derive(Serialize, Debug)]
pub struct SimulationHistory {
    pub history: Vec<Car>,
    pub tps: i32, // Ticks per second used for this simulation
}

impl Simulation {
    fn make_environment(&self) -> ExecEnvironment {
        todo!()
    }

    // TODO - shouldn't need to know where you started and finished - not very clever
    // Currently literally only checks that the car is going at a positive speed and is on the finish line
    // Currently doesn't check starting possition as that'd probably mean that as soon as the car moves it'd have finished
    fn passed_finish_line(&self, start: Point, end: Point) -> bool {
        let mut point_checking = start;
        let delta = (end - start) / (NUMBER_CHECKS as f32);

        for i in 0..NUMBER_CHECKS {
            point_checking += delta;
            if self.am_on_finish_line(point_checking) && self.car.speed > 0.0 {
                return true;
            }
        }
        false
    }

    // TODO - make more intelligent decisions about when the car should die
    fn hit_wall(&self, start: Point, end: Point) -> bool {
        let mut point_checking = start;
        let delta = (end - start) / (NUMBER_CHECKS as f32);

        for i in 0..=NUMBER_CHECKS {
            if !self.in_bounds(point_checking) {
                return true;
            }
            point_checking += delta;
        }
        false
    }

    //TODO: Add f to car to define the max acc depending on current speed
    fn speed_after_tick(&self, starting_speed: f32, acc: f32) -> f32 {
        let car = self.car;
        let actual_acc = acc * car.max_acc;

        (car.speed + actual_acc).max(car.max_speed)
    }

    //TODO: Don't let them turn at any speed per tick like a god damn owl
    fn angle_after_tick(&self, starting_angle: f32, turning_speed: f32) -> f32 {
        (starting_angle + turning_speed)
    }

    // TODO -
    //  In_bounds and am_on_finish_line could be combined
    fn in_bounds(&self, point: Point) -> bool {
        let square = self.track.grid[point.x as usize][point.y as usize];
        match square {
            GridTile::Outside => false,
            GridTile::Inside { .. } => true,
            GridTile::Border {
                border: (p1, p2), ..
            } => {
                // Boolean value to indicate if `point` is "underneath" the line from `p1` to `p2`,
                // where "underneath" is the same direction w.r.t. p1 and p2, regardless of the
                // frame of reference.
                //
                // The boolean value is true if p2.x > p1.x and point.y is below the line from p1
                // to p2.
                let is_underneath = |p: Point| {
                    let amount = (p.x - p1.x) * (p2.y - p1.y) - (p.y - p1.y) * (p2.x - p1.x);
                    amount < 0.0
                };
                let base_is_underneath = is_underneath(point);

                // Now need to work out which neighbours are on the same side of the square to check to see if they're inside
                let center = Point {
                    x: point.x.floor() + 0.5,
                    y: point.y.floor() + 0.5,
                };

                let up = center.add_y(1.0);
                let left = center.add_x(-1.0);
                let down = center.add_y(-1.0);
                let right = center.add_x(1.0);

                let on_inside = [up, left, down, right].iter().cloned().any(|p| {
                    is_underneath(p) == base_is_underneath && self.track.get_tile(p).is_inside()
                });

                on_inside
            }
        }
    }

    fn am_on_finish_line(&self, p: Point) -> bool {
        let square = self.track.get_tile(p);
        match square {
            GridTile::Outside => false,
            GridTile::Inside {
                contains_finish_line,
            } => *contains_finish_line,
            GridTile::Border {
                contains_finish_line,
                ..
            } => {
                if *contains_finish_line && self.in_bounds(p) {
                    true
                } else {
                    false
                }
            }
        }
    }

    pub fn simulate(mut self) -> Result<(Score, SimulationHistory), String> {
        let mut hist = SimulationHistory {
            history: Vec::new(),
            tps: TICKS_PER_SECOND,
        };

        let mut ticks = 0;
        // TODO: Maybe not all code execution errors should be returned here? - e.g. timeouts
        // should maybe not set `action`
        let mut action = self.code.execute(&self.make_environment())?;
        let mut passed_finish = false;

        while !passed_finish {
            // Here, we additionally check if ticks != 0 because it's the initial value of `action`
            if ticks % TICKS_PER_UPDATE == 0 && ticks != 0 {
                action = self.code.execute(&self.make_environment())?;
            }
            ticks += 1;

            let tick_start_pos = self.car.pos;

            self.car.speed = self.speed_after_tick(self.car.speed, action.acc);
            self.car.angle =
                self.angle_after_tick(self.car.angle, action.turning_speed);

            // TODO - Check I've got this the right way around
            // ^ Checked by @sharnoff - looks good
            let traveled = Point {
                x: self.car.angle.cos() * self.car.speed,
                y: self.car.angle.sin() * self.car.speed,
            };
            self.car.pos += traveled;

            hist.history.push(self.car);

            if self.hit_wall(tick_start_pos, self.car.pos) {
                let score = Score {
                    successful: false,
                    time: ticks,
                };

                return Ok((score, hist));
            }

            passed_finish = self.passed_finish_line(tick_start_pos, self.car.pos);
        }

        let score = Score {
            successful: true,
            time: ticks,
        };

        Ok((score, hist))
    }

    pub fn new(id: usize, code: Code, track: Racetrack) -> Self {
        let car = track.initial_car_state;

        Simulation {
            id,
            code,
            track,
            car,
        }
    }
}
