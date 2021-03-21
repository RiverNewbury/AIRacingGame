use crate::code::Code;
use crate::code::ExecEnvironment;
use serde::Serialize;
use std::cmp::max;

mod racetrack;

pub use racetrack::Racetrack;
pub use racetrack::Point;
pub use racetrack::Car;

// A tick is the unit on which thte simulation will update the world
const TICKS_PER_SECOND: i32 = 100;
// The number of ticks until the users code will be asked what it wants to do next
const TICKS_PER_UPDATE: i32 = 10;
// The number of checks along a line that the car travels to make sure it never goes out of bounds
const NUMBER_CHECKS = 10;

// Almost all the computation will be done in the Simulation Object

pub struct Simulation {
    id: usize,  //For keeping track of which call should be returned to who
    code: Code,
    track: Racetrack,
    car: Car,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Score {
    sucessful: bool,
    time: i32 // In terms of ticks
}
// Serialization doesn't like Car or racetrack
#[derive(Serialize)]
pub struct SimulationHistory{
    track: Racetrack,
    history: Vec<Car>,
    tps: i32 // Ticks per second used for this simulation
}

impl Simulation {
    fn make_environment(&self) -> ExecEnvironment{
        todo!()
    }

    // TODO - shouldn't need to know where you started and finished - not very clever
    // Currently literally only checks that the car is going at a positive speed and is on the finish line
    // Currently doesn't check starting possition as that'd probably mean that as soon as the car moves it'd have finished
    fn passed_finish_line(&self, start: Pos, end: Pos) -> bool{
        let mut point_checking = start.clone()
        let delta = Pos::new(change.x / NUMBER_CHECKS, change.y/ NUMBER_CHECKS)

        for i in 0..(NUMBER_CHECKS){
            point_checking = point_checking.add(delta)
            if self.am_on_finish_line(point_checking) && self.car.speed > 0{
                true
            }
        }
        false
    }

    // TODO - make more inteligent decisions about when the car should die
    fn hit_wall(&self, start : Pos, change : Pos) -> bool{
        let mut point_checking = start.clone()
        let delta = Pos::new(change.x / NUMBER_CHECKS, change.y/ NUMBER_CHECKS)

        for i in 0..(NUMBER_CHECKS+1){
            if !self.in_bounds(point_checking) {
                true
            }
            point_checking = point_checking.add(delta)
        }
        false
    }

    //TODO: Add f to car to define the max acc depending on current speed
    fn speed_after_tick(&self, starting_speed:f32, acc: f32, final_speed: f32) -> f32{
        let car = self.car;

        let actual_acc = acc * car.max_acc;

        (car.speed + actual_acc).max(car.max_speed.max(final_speed))
    }
    //TODO: Don't let them turn at any speed per tick like a god damn owl
    fn angle_after_tick(&self, starting_angle:f32, turning_speed: f32, final_angle: f32) -> f32{
        (starting_angle + turning_speed).max(final_angle)
    }


    // TODO -
    // I hate this - it's terrible coding please kill me
    //  in_bounds and am_on_finish_line could be combined
    fn in_bounds(&self, point: Pos) -> bool{
        let square = self.track.grid[point.x as usize][point.y as usize]
        match square {
            Outside => false
            Inside(_) => true
            Border((p1, p2), _) => {
                // True/ false will tell us which side of line on
                // TODO - Will be sad if point is one line - probably bad
                let b = ((point.x - p1.x) * (p2.y - p1.y) - (point.y - p1.y)* (p2.x - p1.x)) < 0;
                // Now need to work out which neighbours are on the same side of the square to check to see if they're inside
                let center = Pos{
                    x : point.x as usize as f32 + 0.5,
                    y : point.y as usize as f32 + 0.5
                };

                let up = Pos::new(center.x, center.y + 1);
                let left = Pos::new(center.x - 1, center.y);
                let down = Pos::new(center.x, center.y - 1);
                let right = Pos::new(center.x + 1, center.y);

                let mut on_inside = false;

                if check_if_on_correct_side_of_line(up, b) && (up == Inside){
                    on_inside = true;
                }
                if check_if_on_correct_side_of_line(left, b) && (left == Inside){
                    on_inside = true;
                }
                if check_if_on_correct_side_of_line(down, b) && (down == Inside){
                    on_inside = true;
                }
                if check_if_on_correct_side_of_line(right, b) && (right == Inside){
                    on_inside = true;
                }

                on_inside
            }
        }
    }

    fn am_on_finish_line(&self, p : Pos){
        let square = self.track.grid[p.x as usize][p.y as usize]
        match square {
            Outside => false
            Inside(b) => b
            Border((p1, p2),  b) => {
                if b && self.in_bounds(p) {
                    true
                } else {
                    false
                }
            }
        }
    }


    fn check_if_on_correct_side_of_line(p: Pos, b: bool) -> bool {
        (b) == (((p.x - p1.x) * (p2.y - p1.y) - (p.y - p1.y)* (p2.x - p1.x)) < 0)
    }

    pub fn simulate(self) -> Result<(Score, SimulationHistory), String> {
        let mut hist = SimulationHistory{
            track :self.track,
            history: (Vec::new()),
            tps: TICKS_PER_SECOND
        };

        let mut ticks = 0;
        let mut what_to_do = self.code.execute(&self.make_environment());
        let mut passed_finish = false;

        while !passed_finish {
            if (ticks%TICKS_PER_UPDATE == 0){
                what_to_do = self.code.execute(&self.make_environment());
            }
            ticks += 1;
            // Check 0<= acc <= 1>
            match what_to_do {
                Some(w) => {
                    self.car.speed = speed_after_tick(self.car.speed, w.acc, w.final_speed)
                    self.car.angle = angle_after_tick(self.car.angle, w.turning_speed, w.final_angle)
                    let start_pos = self.car.pos.clone;
                    // TODO - Check I've got this the right way around
                    let traveled = Point {
                        x: self.car.angle.cos() * self.car.speed,
                        y: self.car.angle.sin() * self.car.speed
                    };
                    self.car.pos.add(traveled);

                    //I think this is a stack not a queue might be difficult
                    hist.history.push(self.car);

                    if self.hit_wall(start_pos, self.car.pos){
                       let score = Score{
                           sucessful: false,
                           time: 0_f32
                       }
                       (score, hist)
                    }
                    passed_finish = self.passed_finish_line(start_pos, self.car.pos)
                }

                Err(w) => Err(w)
            }
        }
    }

    pub fn new(id: usize, code: Code, track: Racetrack) -> Result<Self, String> {
        let car = track.initial_car_state;

        Ok(Simulation {
            id,
            code,
            track,
            car
        })
    }
}
