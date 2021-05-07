//! Full simulation of the car's journey around the racetrack
//!
//! The simulation is based on individual "ticks", where we update the car's state at each tick,
//! corresponding to the previously-requested acceleration and steering. We also check that the car
//! has not crashed into a wall (and if it's crossed the finish line).
//!
//! User input is not requested every tick; instead it's only done once every `TICKS_PER_UPDATE`.
//! There's a number of other relevant constants defined here, all of which are briefly defined.

use crate::code::{CarEnvironment, Code};
use serde::Serialize;
use std::cmp;
use std::f32::consts::PI;

mod car;
mod point;
mod racetrack;

pub use car::Car;
pub use point::Point;
pub use racetrack::{GridTile, Racetrack};

/// A concrete measure of the duration of a tick -- used for scaling external constants (like
/// `Car::MAX_SPEED`, which is measured in units per tick)
const TICKS_PER_SECOND: i32 = 100;

/// The number of ticks until the user's code will be called to get updated actions
const TICKS_PER_UPDATE: i32 = 10;

/// The number of angles at which to check the distance to the wall. Controls the length of the
/// `dist_to_wall` field of [`CarEnvironment`].
const NUMBER_ANGLES_TO_CHECK: usize = 60;

/// The maximum number of ticks that a simulation is allowed to run for. If the user's code does
/// not complete within the alloted time, their score is marked as unsuccessful.
const TICK_LIMIT: i32 = 60000;

// Almost all the computation will be done in the Simulation Object

/// The core of the simulation
///
/// Notable methods here include:
/// * `new` to construct the simulation and its data
/// * `make_environment` for producing the `CarEnvironment` to pass to user code
/// * `simulate` to fully run the simulation
pub struct Simulation {
    code: Code,
    track: &'static Racetrack,

    /// The current state of the car, while simulating. This is updated on each tick
    car: Car,

    /// The total number of times a corner of the car must cross the finish line.
    ///
    /// This value is four times the number of circuits around the track that must be completed.
    laps: i32,
}

/// The result of a single attempt around the racetrack
///
/// `Score`s are ordered so that successful attempts come before unsuccessful ones, and ones with
/// quicker times around the track are ranked higher.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Serialize, Debug)]
pub struct Score {
    /// Whether the attempt was successful
    successful: bool,

    /// The number of ticks before the simulation ended - either by finishing, crashing into a
    /// wall, or timing out.
    time: i32,
}

impl Ord for Score {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.successful
            .cmp(&other.successful)
            // Reverse the ordering here so that shorter times are ranked as "greater"
            .then(other.time.cmp(&self.time))
    }
}

/// The full history of the car's state during the simulation
#[derive(Serialize, Debug)]
pub struct SimulationHistory {
    history: Vec<Car>,

    /// The number of ticks per second used for the simulation
    ///
    /// This is included for forwards-compatability, so that clients need not change if the tick
    /// rate changes here.
    tps: i32,
}

/// The full result of a simulation, including both the trajectory of the car and the user's
/// assigned score
#[derive(Serialize, Debug)]
pub struct SimulationData {
    pub history: SimulationHistory,
    pub score: Score,
}

impl Simulation {
    /// Constructs the information about the car passed to the user's code
    fn make_environment(&self) -> CarEnvironment {
        // Produces the distance from the car to wall at a given angle
        let f = |mut angle: f32| {
            let (x_lo, x_hi) = (0.0, self.track.width as f32 * self.track.tile_size);
            let (y_lo, y_hi) = (0.0, self.track.height as f32 * self.track.tile_size);

            // Get the angle in the range 0..2π
            if angle < 0.0 {
                // Not convinced as to why this works? try it on desmos!
                angle += (angle.abs() / (2.0 * PI)).ceil() * (2.0 * PI);
                assert!((0.0..(2.0 * PI)).contains(&angle));
            } else {
                angle = angle % (2.0 * PI);
            }

            // Helper booleans to indicate whether the angle is facing in that "sort" of direction.
            // It's essentially to tell us what quadrant the angle is in, but in a somewhat nicer
            // form.
            let upper = (0.0..PI).contains(&angle);
            let left = (PI / 2.0..3.0 * PI / 2.0).contains(&angle);
            let lower = (PI..2.0 * PI).contains(&angle);
            let right =
                (3.0 * PI / 2.0..2.0 * PI).contains(&angle) || (0.0..PI / 2.0).contains(&angle);

            // We'll set `end_point` to be the point that we collide with at the bounds of the
            // racetrack from this angle
            //
            // If we don't find any aplicable point, that's an error.
            let mut end_point = None;

            // A sample point on the line with the angle we're interested in -- exactly one unit's
            // distance in that direction, though that fact isn't used anywhere.
            let unit_point = Point {
                x: self.car.pos.x + angle.cos(),
                y: self.car.pos.y + angle.sin(),
            };

            if upper {
                let p1 = Point { x: x_lo, y: y_hi };
                let p2 = Point { x: x_hi, y: y_hi };
                if let Some(p) = intersection_of_2_lines(p1, p2, self.car.pos, unit_point) {
                    if x_lo <= p.x && p.x <= x_hi {
                        end_point = Some(p);
                    }
                }
            } else if lower {
                let p1 = Point { x: x_lo, y: y_lo };
                let p2 = Point { x: x_hi, y: y_lo };
                if let Some(p) = intersection_of_2_lines(p1, p2, self.car.pos, unit_point) {
                    if x_lo <= p.x && p.x <= x_hi {
                        end_point = Some(p);
                    }
                }
            }

            if left {
                let p1 = Point { x: x_lo, y: y_lo };
                let p2 = Point { x: x_lo, y: y_hi };
                if let Some(p) = intersection_of_2_lines(p1, p2, self.car.pos, unit_point) {
                    if y_lo <= p.y && p.y <= y_hi {
                        end_point = Some(p);
                    }
                }
            } else if right {
                let p1 = Point { x: x_hi, y: y_lo };
                let p2 = Point { x: x_hi, y: y_hi };
                if let Some(p) = intersection_of_2_lines(p1, p2, self.car.pos, unit_point) {
                    if y_lo <= p.y && p.y <= y_hi {
                        end_point = Some(p);
                    }
                }
            }

            let edge_point = end_point.expect("couldn't find a point on the edge of the track");
            let wall_point = self
                .passed_line(self.car.pos, edge_point, GridTile::try_border)
                .expect("couldn't find wall collision point");

            // Calculate the distance to the point on the wall
            let d = wall_point - self.car.pos;
            let dist = (d.x * d.x + d.y * d.y).sqrt();
            (dist * 16.0).round() / 16.0
        };

        // We want to have the angles exactly fill the semi-circle in front of the car; if we
        // defined `angle_delta` to be equal to "pi / num angles", we would be missing either the
        // left-most or right-most angle. So we subtract 1 to ensure that we include both
        // endpoints.
        let angle_delta = PI / (NUMBER_ANGLES_TO_CHECK as f32 - 1.0);

        // Per the requirements of `CarEnvironment`, the distances here are ordered from left-most
        // (i.e. greatest angle) to right-most (least angle). Directly left of the car corresponds
        // to an increase of π/2 from its angle, and to the right is a decrease of π/2.
        let base_angle = self.car.angle + (PI / 2.0);
        let dists = (0..NUMBER_ANGLES_TO_CHECK)
            .map(|i| f(base_angle - i as f32 * angle_delta))
            .collect();

        CarEnvironment {
            pos: self.car.pos,
            angle: self.car.angle,
            speed: self.car.speed / Car::MAX_SPEED,
            dist_to_wall: dists,
        }
    }

    /// Returns the position -- if applicable -- at which something traveling from `start` to `end`
    /// would pass a line given by `get_line`.
    ///
    /// This is made generic so that we can use the same function for wall collision detection,
    /// checking if the car has passed the finish line, AND producing the distances to the nearest
    /// walls for `CarEnvironment`.
    ///
    /// Because this is so crucial, it's intentionally thoroughly commented.
    fn passed_line<Func>(&self, start: Point, end: Point, get_line: Func) -> Option<Point>
    where
        Func: Fn(&GridTile) -> Option<(Point, Point)>,
    {
        use std::cmp::Ordering::{Equal, Greater, Less};

        // We're doing something reasonably complicated here. As we're going along, we store the
        // current index(es) of the tile we're looking at -- by `row` and `col`. We also have a
        // couple helper points: `current_point` and `next_point`, the latter of which is
        // calculated inside of the loop.
        //
        // `current_point` corresponds to a value that's *either* the starting point, or on the
        // edge of a tile on the line from `start` to `end. `next_point` is similar, except it is
        // either the END point, or on an edge.
        //
        // Both `current_point` and `next_point` are always guaranteed to be along the line from
        // `start` to `end`.
        //
        // We pick the value of `next_point` by examining which edge of the current tile the line
        // collides with -- essentially by checking whether the intersections with the edges of the
        // tile most near `end` are actually within the bounds of the other borders.
        let mut current_point = start;

        // Helper alias to make things readable.
        let tile_size = self.track.tile_size;

        let mut cur_col = (start.x / tile_size) as usize;
        let mut cur_row = (start.y / tile_size) as usize;

        // We can make the condition here exact because - once we get far enough along - we just
        // end up setting `next_point` to exactly the value of `end`.
        while current_point != end {
            // The bounds of the current tile, as defined by the four bounding lines
            let (x_lo, x_hi) = (cur_col as f32 * tile_size, (cur_col + 1) as f32 * tile_size);
            let (y_lo, y_hi) = (cur_row as f32 * tile_size, (cur_row + 1) as f32 * tile_size);

            // We'll now try to get the next point. This is a little bit funky -- because we don't
            // have labeled blocks, we'll just have a single-iteration loop. LLVM will compile this
            // down to the proper thing by realizing that there's no way to repeat the loop.
            let (next_row, next_col, next_point) = loop {
                // First up: check if the current tile contains the end point -- if it does, we're
                // done.
                if x_lo <= end.x && end.x <= x_hi && y_lo <= end.y && end.y <= y_hi {
                    // We return `cur_row` and `cur_col` because the indexes are only reset if the
                    // point we return isn't `end`.
                    break (cur_row, cur_col, end);
                }

                // Check the upper or lower edge of the tile, depending on which direction the line
                // goes
                match end.y.partial_cmp(&start.y) {
                    Some(Greater) => {
                        let p1 = Point { x: x_lo, y: y_hi };
                        let p2 = Point { x: x_hi, y: y_hi };

                        match intersection_of_2_lines(p1, p2, start, end) {
                            Some(p) if x_lo <= p.x && p.x <= x_hi => {
                                break (cur_row + 1, cur_col, p);
                            }
                            _ => (),
                        }
                    }
                    Some(Less) => {
                        let p1 = Point { x: x_lo, y: y_lo };
                        let p2 = Point { x: x_hi, y: y_lo };

                        match intersection_of_2_lines(p1, p2, start, end) {
                            Some(p) if x_lo <= p.x && p.x <= x_hi => {
                                break (cur_row - 1, cur_col, p);
                            }
                            _ => (),
                        }
                    }
                    Some(Equal) | None => (),
                }

                match end.x.partial_cmp(&start.x) {
                    Some(Greater) => {
                        let p1 = Point { x: x_hi, y: y_lo };
                        let p2 = Point { x: x_hi, y: y_hi };

                        match intersection_of_2_lines(p1, p2, start, end) {
                            Some(p) if y_lo <= p.y && p.y <= y_hi => {
                                break (cur_row, cur_col + 1, p);
                            }
                            _ => (),
                        }
                    }
                    Some(Less) => {
                        let p1 = Point { x: x_lo, y: y_lo };
                        let p2 = Point { x: x_lo, y: y_hi };
                        match intersection_of_2_lines(p1, p2, start, end) {
                            Some(p) if y_lo <= p.y && p.y <= y_hi => {
                                break (cur_row, cur_col - 1, p);
                            }
                            _ => (),
                        }
                    }
                    Some(Equal) | None => (),
                }

                panic!("start and end points are equal or incomparable");
            };

            let tile = &self.track.grid[cur_row][cur_col];

            if let Some((p1, p2)) = get_line(tile) {
                match intersection_of_2_lines(p1, p2, start, end) {
                    // At this point, we found a collision between the two lines. We now need to
                    // check that the collision actually occurs within the region we're looking at
                    // -- because it's certainly possible it doesn't.
                    //
                    // We have to check against `next_point` here instead of -- for example,
                    // `x/y_lo/hi` -- because it's possible to find points that exist on the line
                    // joining `start` to `end` that are technically beyond the assumed bounds.
                    Some(p) if p.inside_rectangle(current_point, next_point) => {
                        return Some(p);
                    }
                    // If we didn't find an applicable collision, we'll fall through to the next
                    // iteration, if it exists
                    _ => (),
                }
            }

            cur_col = next_col;
            cur_row = next_row;
            current_point = next_point;
        }

        // If we got to the end of the loop without finding an intersection point,
        None
    }

    // Checks the car goes over the finishline the correct number of times to finish the game
    fn passed_finish_line(&mut self, start: Point, end: Point) -> bool {
        let (f1, f2) = self.track.finish_line;

        // There's a few things we can exploit here. We'll assert that they're still true, just in
        // case they change in the future.
        //
        // Firstly, we know that the finish line is always horizontal:
        debug_assert!(f1.y == f2.y && f1.x != f2.x);
        // We also know that the car starts facing downwards:
        debug_assert!(self.track.initial_car_state.angle == 3.0 * PI / 2.0);
        //
        // From this, we can check if the point is going across the finish line in the correct
        // direction (if at all):
        let correct_direction = start.y > f1.y && end.y < f1.y;

        // We can then just use our handy-dandy `passed_line` function! Maybe it's overkill? Or
        // maybe it's *just fine*.
        let get_line = |tile: &GridTile| {
            if tile.contains_finish_line() {
                Some((f1, f2))
            } else {
                None
            }
        };

        let crossed = self.passed_line(start, end, get_line).is_some();

        if crossed && correct_direction && self.laps == 0 {
            true
        } else if crossed && correct_direction {
            self.laps -= 1;
            false
        } else if crossed && !correct_direction {
            self.laps += 1;
            false
        } else {
            false
        }
    }

    /// Returns whether a corner of the car traveling from `start` to `end` would hit the border
    /// wall of the track
    // TODO - make more intelligent decisions about when the car should die
    fn hit_wall(&self, start: Point, end: Point) -> bool {
        // Easy safeguard. If `passed_line` works correctly, though, this shouldn't ever be needed.
        if self.track.get_tile(end).is_outside() {
            return true;
        }

        self.passed_line(start, end, GridTile::try_border).is_some()
    }

    //TODO: Research air resistance
    fn speed_after_tick(&self, mut acc: f32) -> f32 {
        // Acceleration is intended to be between -1 and 1, with -1 indicating maximum decreasing
        // of speed and 1 indicating maximum increasing
        acc = acc.clamp(-1.0, 1.0);

        // There's separate bounds on acceleration and deceleration. We'll figure out how fast the
        // car is *actually* going:
        let actual_acc = if acc >= 0.0 {
            acc * self.car.max_acc()
        } else {
            acc * self.car.max_dec()
        };

        // Taking the minimum here isn't currently needed with the acceleration formula, but it
        // safeguards against future changes.
        (self.car.speed + actual_acc).min(Car::MAX_SPEED)
    }

    /// Runs the full simulation, returning only if: (a) the car finishes, (b) the car crashes,
    /// or (c) the tick limit is reached
    //
    // The users affect on the car happen at the start of the tick (before calculating new position)
    pub fn simulate(mut self) -> Result<(Score, SimulationHistory), String> {
        // If Car more than 1 unit wide will break wall collision - (as only check at the corners so
        //  in the situation below the car could drive straight over the x
        //  +---------+
        //  |         |   x
        //  +---------+
        assert!(Car::WIDTH < 1.0);

        let mut hist = SimulationHistory {
            history: vec![self.car.clone()],
            tps: TICKS_PER_SECOND,
        };

        let mut ticks = 0;
        // TODO: Maybe not all code execution errors should be returned here? - e.g. timeouts
        // should maybe not set `action`
        let mut action = self.code.execute(&self.make_environment())?;
        let mut passed_finish = false;

        //TODO do this in the code.rs
        while !passed_finish && (ticks < TICK_LIMIT) {
            // Here, we additionally check if ticks != 0 because it's the initial value of `action`
            if ticks % TICKS_PER_UPDATE == 0 && ticks != 0 {
                action = self.code.execute(&self.make_environment())?;
            }
            ticks += 1;

            let corners_start_pos = self.car.pos_of_corners();

            let new_speed = self.speed_after_tick(action.acc);

            // The speed of the car is per-tick, so the distance traveled in a single tick is just
            // the average speed for that tick. We'll assume the speed increases uniformly.
            let dist = (self.car.speed + new_speed) / 2.0;
            self.car.speed = new_speed;
            self.car.update_state(dist, action.steering);

            hist.history.push(self.car.clone());

            let corners_end_pos = self.car.pos_of_corners();

            for (s, f) in corners_start_pos.iter().zip(corners_end_pos.iter()) {
                if self.hit_wall(*s, *f) {
                    let score = Score {
                        successful: false,
                        time: ticks,
                    };

                    return Ok((score, hist));
                }
            }

            for (s, f) in corners_start_pos.iter().zip(corners_end_pos.iter()) {
                if self.passed_finish_line(*s, *f) {
                    passed_finish = true;
                }
            }
        }

        let score = Score {
            successful: passed_finish,
            time: ticks,
        };

        Ok((score, hist))
    }

    /// Creates a new `Simulation` object, given the user's code and the specified racetrack to use
    pub fn new(code: Code, track: &'static Racetrack) -> Self {
        Simulation {
            code,
            track,
            car: track.initial_car_state.clone(),
            laps: 2,
        }
    }
}

fn intersection_of_2_lines(s1: Point, e1: Point, s2: Point, e2: Point) -> Option<Point> {
    // Line s1 e1 represented a1x + b1y = c1
    let a1 = e1.y - s1.y;
    let b1 = s1.x - e1.x;
    let c1 = a1 * s1.x + b1 * s1.y;

    // Line s2 e2 represented as a2x + b2y = c2
    let a2 = e2.y - s2.y;
    let b2 = s2.x - e2.x;
    let c2 = a2 * s2.x + b2 * s2.y;

    let det = a1 * b2 - a2 * b1;

    if det.abs() <= 0.00000001 {
        None
    } else {
        let x = (b2 * c1 - b1 * c2) / det;
        let y = (a1 * c2 - a2 * c1) / det;
        Some(Point { x, y })
    }
}
