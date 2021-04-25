use crate::code::{Code, ExecEnvironment};
use serde::Serialize;

mod point;
mod racetrack;

pub use point::Point;
pub use racetrack::{Car, GridTile, Racetrack, CAR_LENGTH, CAR_WIDTH};

// A tick is the unit on which thte simulation will update the world
const TICKS_PER_SECOND: i32 = 100;
// The number of ticks until the users code will be asked what it wants to do next
const TICKS_PER_UPDATE: i32 = 10;
// The number of checks/ unit dist along a line that the car travels to make sure it never goes out of bounds
const NUMBER_CHECKS_PER_UNIT_DIST: f32 = 10.0;
// The maximum error acceptable when giving the distance to the wall to the User
const ACCURACY_OF_DIST_TO_WALL :f32 = 0.001;
// The number of angles to check the distance to the wall at
// MUST - devide 360 in the ring on integers
const NUMBER_ANGLES_TO_CHECK :usize = 60;

// Almost all the computation will be done in the Simulation Object

pub struct Simulation {
    code: Code,
    track: &'static Racetrack,
    car: Car,
    // For i circuits to have to be done laps = 4 * i (as car has 4 corners)
    laps: i32,
}

//TODO - Made field public for score + sim hist pub for ex result
// TODO - added debug for ex result
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Debug)]
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
        let go_dist = | start: Point, dist: f32, angle :f32|{
            let traveled = Point {
                x: angle.cos() * dist,
                y: angle.sin() * dist,
            };
            start + traveled
        };

        let f = | angle :f32 | {
            let mut dist_traveled = 0.0;
            let mut precision = 1.0;
            let mut start = self.car.pos;
            let mut end = go_dist(start, precision, angle);

            while precision > ACCURACY_OF_DIST_TO_WALL{
                let hit = self.hit_wall(start, end);
                if hit {
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
        let angle_delta = 360/NUMBER_ANGLES_TO_CHECK;

        for i in 0..NUMBER_ANGLES_TO_CHECK {
            dists.push(f((i*angle_delta) as f32))
        }


        ExecEnvironment{
            car_currently : self.car,
            dist_to_wall :  dists,
        }
    }

    // Checks the car goes over the finishline the correct number of times to finish the game
    fn passed_finish_line(&mut self, start: Point, end: Point) -> bool {
        let (p1, p2) = self.track.finish_line;

        let intersection = Simulation::intersection_of_2_lines(start, end, p1, p2);

        //TODO : If p2.x = p1.x then it breaks FIX
        let ycheck = p1.y + (p2.y - p1.y) * (p1.x - start.x)/ (p2.x - p1.x);

        //Tells you if coming from the correct direction
        // TODO : may break if doesn't start at 0 angle
        let correct_direction = ycheck>= start.y;

        match intersection {
            None => false,
            Some(p) => {
                if Simulation::between_2_points(p1,p2, p) && Simulation::between_2_points(start, end, p){
                    if correct_direction && (self.laps == 0){
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
            },
        }
    }

    fn between_2_points(p1:Point, p2: Point, point_2_compare: Point) -> bool {
        let xbetween = p1.x >= point_2_compare.x && p2.x <= point_2_compare.x
                    || p1.x <= point_2_compare.x && p2.x >= point_2_compare.x;

        let ybetween = p1.y >= point_2_compare.y && p2.y <= point_2_compare.y
                    || p1.y <= point_2_compare.y && p2.y >= point_2_compare.y;

        xbetween && ybetween
    }

    fn intersection_of_2_lines(s1: Point, e1: Point, s2:Point, e2:Point) -> Option<Point> {
        // Line s1 e1 represented a1x + b1y = c1
        let a1 = s1.y - e1.y;
        let b1 = s1.x - e1.x;
        let c1 = a1*s1.x + b1*s1.y;

        // Line s2 e2 represented as a2x + b2y = c2
        let a2 = s2.y - e2.y;
        let b2 = s2.x - e2.x;
        let c2 = a2*s2.x + b2*s2.y;

        let det = a1*b2 - a2*b1;

        if det.abs() <= 0.00000001 {
            None
        } else {
            let x = (b2*c1 - b1*c2)/det;
            let y = (a1*c2 - a2*c1)/det;
            Some(Point{x, y})
        }
    }

    // TODO - make more intelligent decisions about when the car should die
    // ASSUMES - that the car is thinner than 1 unit
    fn hit_wall(&self, start: Point, end: Point) -> bool {
        let mut point_checking = start;
        let num_checks : i32 = ((end - start).length() * NUMBER_CHECKS_PER_UNIT_DIST) as i32;
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
        let car = self.car;
        let actual_acc = acc * car.max_acc();

        (car.speed + actual_acc).max(car.max_speed)
    }

    //TODO: Don't let them turn at any speed per tick like a god damn owl
    fn angle_after_tick(&self,turning_speed: f32) -> f32 {
        self.car.angle + turning_speed
    }

    // TODO - Probably should use more advanced line system
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

    // The users affect on the car happen at the start of the tick (before calculating new position)
    pub fn simulate(mut self) -> Result<(Score, SimulationHistory), String> {
        // If Car more than 1 unit wide will break wall collision - (as only check at the corners so
        //  in the situation below the car could drive straight over the x
        //  +---------+
        //  |         |   x
        //  +---------+
        assert!(CAR_WIDTH < 1.0);

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

            let start_pos = self.car.pos_of_corners();

            self.car.speed = self.speed_after_tick(action.acc);
            self.car.angle = self.angle_after_tick(action.turning_speed);

            self.car.pos += Point::new_polar(self.car.speed, self.car.angle);

            hist.history.push(self.car);

            let end_pos = self.car.pos_of_corners();

            for (s,f) in start_pos.iter().zip(end_pos.iter()) {
                if self.hit_wall(*s, *f) {
                    let score = Score {
                        successful: false,
                        time: ticks,
                    };

                    return Ok((score, hist));
                }
            }


            for (s,f) in start_pos.iter().zip(end_pos.iter()) {
                if self.passed_finish_line(*s, *f) {
                    passed_finish = true
                }
            }

        }

        let score = Score {
            successful: true,
            time: ticks,
        };

        Ok((score, hist))
    }

    pub fn new(code: Code, track: &'static Racetrack) -> Self {
        Simulation {
            code,
            track,
            car: track.initial_car_state,
            laps: 4*track.laps,
        }
    }
}
