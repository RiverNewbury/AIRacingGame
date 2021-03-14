// Note: the size of the car really only makes sense when compared to the size of the tiles in a
// racetrack grid. The size of the car is probably unlikely to change, whereas the tile size is
// explicitly variable.

/// The absolute size length of the car
const CAR_LENGTH: f32 = 1.0;
/// The width of the car
const CAR_WIDTH: f32 = 0.3;

/// All of the information about the car at a particular point in time
#[derive(Copy, Clone)]
struct Car {
    /// The position of the car
    pos: Point,
    /// The angle the car is facing, anticlockwise from the positive y direction
    angle: f32,
    /// The current speed, in "unit distance per simulation tick", of the car
    speed: f32,
}
