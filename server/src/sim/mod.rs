use crate::code::{Code, ExecEnvironment};
use serde::Serialize;
use std::f32::consts::PI;

mod car;
mod point;
mod racetrack;

pub use car::Car;
pub use point::Point;
pub use racetrack::{GridTile, Racetrack};

// A tick is the unit on which thte simulation will update the world
const TICKS_PER_SECOND: i32 = 100;
// The number of ticks until the users code will be asked what it wants to do next
const TICKS_PER_UPDATE: i32 = 10;
// The number of checks/ unit dist along a line that the car travels to make sure it never goes out of bounds
const NUMBER_CHECKS_PER_UNIT_DIST: f32 = 10.0;
// The maximum error acceptable when giving the distance to the wall to the User
const ACCURACY_OF_DIST_TO_WALL: f32 = 0.001;
// The number of angles to check the distance to the wall at
const NUMBER_ANGLES_TO_CHECK: usize = 60;
// Emergency Tick Limit
const TICK_LIMIT: i32 = 60000;

// Almost all the computation will be done in the Simulation Object

pub struct Simulation {
    code: Code,
    track: &'static Racetrack,
    car: Car,
    // For i circuits to have to be done laps = 4 * i (as car has 4 corners)
    laps: i32,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Debug)]
pub struct Score {
    successful: bool,
    time: i32, // In terms of ticks
}

#[derive(Serialize, Debug)]
pub struct SimulationHistory {
    history: Vec<Car>,
    tps: i32, // Ticks per second used for this simulation
}

#[derive(Serialize, Debug)]
pub struct SimulationData {
    pub history: SimulationHistory,
    pub score: Score,
}

impl Simulation {
    fn make_environment(&self) -> ExecEnvironment {
        let go_dist = |start: Point, dist: f32, angle: f32| {
            let traveled = Point::new_polar(dist, angle);
            start + traveled
        };

        let f = |angle: f32| {
            let mut dist_traveled = 0.0;
            let mut precision = 1.0;
            let mut start = self.car.pos;
            let mut end = go_dist(start, precision, angle);

            while precision > ACCURACY_OF_DIST_TO_WALL {
                let hit = self.hit_wall(start, end);
                if !hit {
                    dist_traveled += precision;
                    precision *= 2.0;
                    start = end;
                } else {
                    precision /= 2.0;
                }
                end = go_dist(start, precision, angle);
            }

            dist_traveled
        };
        let mut dists = Vec::with_capacity(NUMBER_ANGLES_TO_CHECK);
        let angle_delta = PI / (NUMBER_ANGLES_TO_CHECK as f32 - 1.0);

        let base_angle = self.car.angle - (PI / 2.0);
        for i in (0..NUMBER_ANGLES_TO_CHECK).rev() {
            dists.push(f(base_angle + i as f32 * angle_delta))
        }

        ExecEnvironment {
            car_currently: self.car.clone(),
            dist_to_wall: dists,
        }
    }

    fn passed_finish_line2(&self, start: Point, end: Point) -> bool {
        let mut point_checking = start;
        let num_checks: i32 = ((end - start).length() * NUMBER_CHECKS_PER_UNIT_DIST) as i32;
        let delta = (end - start) / (num_checks as f32);
        for i in 0..num_checks {
            point_checking += delta;
            if self.am_on_finish_line(point_checking) && self.car.speed > 0.0 {
                return true;
            }
        }
        false
    }

    fn am_on_finish_line(&self, p: Point) -> bool {
        let square = self.track.get_tile(p);
        match square {
            GridTile::Outside => false,
            GridTile::Inside {
                contains_finish_line,
            } => contains_finish_line,
            GridTile::Border {
                contains_finish_line,
                ..
            } => {
                if contains_finish_line && self.in_bounds(p) {
                    true
                } else {
                    false
                }
            }
        }
    }

    // Checks the car goes over the finishline the correct number of times to finish the game
    fn passed_finish_line(&mut self, start: Point, end: Point) -> bool {
        let (p1, p2) = self.track.finish_line;

        let intersection = Simulation::intersection_of_2_lines(start, end, p1, p2);
        //TODO : If p2.x = p1.x then it breaks FIX
        let ycheck = p1.y + (p2.y - p1.y) * (p1.x - start.x) / (p2.x - p1.x);

        //Tells you if coming from the correct direction
        // TODO : may break if doesn't start at 180 angle
        let correct_direction = ycheck <= start.y;
        match intersection {
            None => false,
            Some(p) => {
                if Simulation::between_2_points(p1, p2, p)
                    && Simulation::between_2_points(start, end, p)
                {
                    if correct_direction && (self.laps == 0) {
                        true
                    } else if correct_direction {
                        self.laps -= 1;
                        false
                    } else {
                        self.laps += 1;
                        false
                    }
                } else {
                    false
                }
            }
        }
    }

    fn between_2_points(p1: Point, p2: Point, point_2_compare: Point) -> bool {
        let xbetween = p1.x >= point_2_compare.x && p2.x <= point_2_compare.x
            || p1.x <= point_2_compare.x && p2.x >= point_2_compare.x;

        let ybetween = p1.y >= point_2_compare.y && p2.y <= point_2_compare.y
            || p1.y <= point_2_compare.y && p2.y >= point_2_compare.y;

        xbetween && ybetween
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

    // TODO - make more intelligent decisions about when the car should die
    // ASSUMES - that the car is thinner than 1 unit
    fn hit_wall(&self, start: Point, end: Point) -> bool {
        let mut point_checking = start;
        let num_checks: i32 = ((end - start).length() * NUMBER_CHECKS_PER_UNIT_DIST) as i32;
        let delta = (end - start) / (num_checks as f32);

        for _i in 0..=num_checks {
            if !self.in_bounds(point_checking) {
                return true;
            }
            point_checking += delta;
        }
        false
    }

    //TODO: Research air resistance
    fn speed_after_tick(&self, acc: f32) -> f32 {
        let car = &self.car;
        let actual_acc = acc * car.max_acc();

        (car.speed + actual_acc).min(Car::MAX_SPEED)
    }

    // TODO - Probably should use more advanced line system
    // ALL border tiles are assumed to be in the racetrack
    // TODO - fix this ^
    fn in_bounds(&self, point: Point) -> bool {
        let square = self.track.get_tile(point);
        match square {
            GridTile::Outside => false,
            GridTile::Inside { .. } => true,
            GridTile::Border { .. } => true,
        }
    }

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

            let start_pos = self.car.pos_of_corners();

            let new_speed = self.speed_after_tick(action.acc);

            // The speed of the car is per-tick, so the distance traveled in a single tick is just
            // the average speed for that tick. We'll assume the speed increases uniformly.
            let dist = (self.car.speed + new_speed) / 2.0;
            self.car.speed = new_speed;
            self.car.update_state(dist, action.turning_speed);

            hist.history.push(self.car.clone());

            let end_pos = self.car.pos_of_corners();

            //print!("{:?}", hist.history[hist.history.len() - 1]);

            for (s, f) in start_pos.iter().zip(end_pos.iter()) {
                if self.hit_wall(*s, *f) {
                    print!("{:?}, {:?}", *s, *f);
                    let score = Score {
                        successful: false,
                        time: ticks,
                    };

                    return Ok((score, hist));
                }
            }

            for (s, f) in start_pos.iter().zip(end_pos.iter()) {
                if self.passed_finish_line(*s, *f) {
                    passed_finish = true
                }
            }
        }

        let score = Score {
            successful: passed_finish,
            time: ticks,
        };

        Ok((score, hist))
    }

    pub fn new(code: Code, track: &'static Racetrack) -> Self {
        Simulation {
            code,
            track,
            car: track.initial_car_state.clone(),
            laps: 0,
        }
    }
}
