//! Wrapper module for the [`Car`] type

use pyo3::prelude::pyclass;
use serde::Serialize;
use std::f32::consts::PI;

use super::{Point, TICKS_PER_SECOND};

/// All of the information about the car at a particular point in time
#[pyclass]
#[derive(Clone, Serialize, Debug)]
pub struct Car {
    /// The position of the car
    pub pos: Point,
    /// The angle the car is facing, anticlockwise from the positive x direction - in radians
    pub angle: f32,
    /// The current speed, in "unit distance per simulation tick", of the car
    pub speed: f32,
}

impl Car {
    // Note: the size of the car really only makes sense when compared to the size of the tiles in
    // a racetrack grid. The size of the car is probably unlikely to change, whereas the tile size
    // is explicitly variable.
    /// The absolute size length of the car
    pub const LENGTH: f32 = 1.0;
    /// The width of the car
    pub const WIDTH: f32 = 0.3 * Self::LENGTH;

    /// The maximum allowed speed of a car, in units per tick
    pub const MAX_SPEED: f32 = 10.0 / TICKS_PER_SECOND as f32;

    /// The maximum forward acceleration of the car, in units per tick, per tick
    ///
    /// The [`max_acc`] method uses linear scaling so that the *actual* maximum forward
    /// acceleratoin is equal to this value at a speed of zero, and approaches zero as the car's
    /// speed approaches `MAX_SPEED`
    ///
    /// [`max_acc`]: Self::max_acc
    const MAX_ACC: f32 = 0.5 * Car::MAX_SPEED / TICKS_PER_SECOND as f32;

    /// The maximum (backward) deceleration of the car, in units per tick, per tick
    ///
    /// Like `CAR_MAX_ACC`, this also has linear scaling provided by [`max_dec`](Self::max_dec)
    const MAX_DEC: f32 = 0.3 * Car::MAX_SPEED / TICKS_PER_SECOND as f32;

    /// The maximum acceleration of the car, scaled so that the maximum allowed acceleration
    /// approaches zero as the car's speed approaches `MAX_SPEED`
    pub fn max_acc(&self) -> f32 {
        (1.0 - self.speed / Self::MAX_SPEED) * Self::MAX_ACC
    }

    /// The maximum deceleration of the car, scaled so that the maximum allowd deceleration
    /// approaches zero as the car's speed approaches zero
    pub fn max_dec(&self) -> f32 {
        self.speed / Self::MAX_SPEED * Self::MAX_DEC
    }

    pub fn pos_of_corners(&self) -> Vec<Point> {
        // Let's imagine the car going upwards; its angle will be PI/2. We want to determine where
        // the corners are. We can use the the front/back and left/right offset from one corner to
        // determine all the rest, so let's just focus on getting the position of the front-right
        // corner, relative to all the rest.
        //
        // So let's look at the following diagram of the car. The asterisk gives us the center of
        // the car:
        //          +---+ displacement from center to center right
        //          |   |
        //      +---+===+ -+
        //      |   |   |  | displacement from center to front center
        //      |   |   |  |
        //      |   |   |  |
        //      |   *   | -+
        //      |       |
        //      |       |
        //      |       |
        //      +-------+
        //
        // If we calculate these as `Point`s, we'll get the x and y components separately, so we
        // can add all the positive and negative combinations to get the four corners of the car.
        //
        // Getting these displacements is actualy pretty simple! Let's go through them. Looking at
        // these with the above diagram in mind makes quick checks nice for us :)
        let to_front = Point {
            x: self.angle.cos() * Car::LENGTH,
            y: self.angle.sin() * Car::LENGTH,
        };

        // The angle here is rotated clockwise by a quarter-turn -- i.e. PI/2. We could use that
        // directly, but we know from our trig identities that:
        //
        //   cos(θ - π/2) =   cos(π/2 - θ) =   sin(θ), and
        //   sin(θ - π/2) = - sin(π/2 - θ) = - cos(θ).
        //
        // So we get:
        let to_right = Point {
            x: self.angle.sin() * Car::WIDTH,
            y: -self.angle.cos() * Car::WIDTH,
        };

        vec![
            // Front right
            self.pos + to_front + to_right,
            // Front left
            self.pos + to_front - to_right,
            // Back right
            self.pos - to_front + to_right,
            // Back left
            self.pos - to_front - to_right,
        ]
    }

    /// Updates the position and angle of the car corresponding to traveling the provided distance
    /// with the given wheel angle.
    ///
    /// The wheel angle will be clamped to between -1 and 1, where -1 is all the way to the left
    /// and 1 is all the way to the right.
    pub fn update_state(&mut self, distance: f32, mut wheel_angle: f32) {
        wheel_angle = wheel_angle.clamp(-1.0, 1.0);

        // This wheel angle isn't very useful right now. We'll put it into radians, from -pi/2 to
        // pi/2:
        wheel_angle = wheel_angle * PI / 4.0;

        // We're now dealing with a pretty complicated situation. Because this is so complicated,
        // we'll first figure out what the updated position of the car *would* be, if we started at
        // the origin, and had an angle of pi/2 (i.e. travelling towards positive y)
        let unrotated_pos_shift: Point;
        let angle_change: f32;

        if wheel_angle == 0.0 {
            // 0 is a special case. We just go straight ahead, which in this case is increasing y,
            // because we said that we're pretending the car has an angle of pi/2:
            unrotated_pos_shift = Point {
                x: 0.0,
                y: distance,
            };
            angle_change = 0.0;
        } else if wheel_angle > 0.0 {
            // If the angle is greater than 0, the car is turning to the right.
            //
            // We'll go through the full example for this one; the final branch is just this but
            // mirrored.
            //
            // So our car is turning to the right, with the wheels at some degree - call that θ for
            // now. Instead of dealing with the whole "front wheels turn, back wheels stationary"
            // thing, we'll just pretend like both the front and the back wheels are turning by
            // θ/2 - that should be roughly the same. We'll also assume that there's perfect
            // traction, and that we're actually a motorcycle.
            //
            // This leaves us with two angled points:
            //  * the front wheel, turned up and to the right; and
            //  * the back wheel, turned up and to the left.
            // Both have an angle of θ/2 between them the line and the y-axis, and we know that the
            // car will be travelling on the circle that passes through both of these points at the
            // correct angle(s).
            //
            // The center of the circle is to the right of the center of the car, and we can
            // calculate the radius 'r' by:
            //
            //   r * sin(θ/2) = Car::WIDTH / 2
            //
            //   => r = Car::WIDTH / (2 * sin(θ/2))
            //
            // Drawing a simple diagram of the problem should illuminate the above.
            //
            // With the radius of the circle and the distance along that circle we're going, we can
            // *really* simply find the new point! The car is travelling a distance 'd' along a
            // circle with radius 'r'. Let's call the angle between the start and end points α.
            //
            // Because of the definition of radians, we know that travelling a distance 'd' along
            // this circle will result in an angle of 'α'! So the new points are just:
            //   x´ = r *(1 - cos(α))
            //   y´ = r * sin(α)
            // Remember that the x coordinate has to be flipped because the circle is to the right
            // of the car.
            let r = Car::WIDTH / (2.0 * f32::sin(wheel_angle / 2.0));
            let alpha = distance / r;

            unrotated_pos_shift = Point {
                x: r * (1.0 - alpha.cos()),
                y: r * alpha.sin(),
            };

            // The angle change ends up being α -- almost. Our angles are reversed in the input, so
            // we're actually making the angle of the car more negative here.
            angle_change = -alpha;
        } else {
            // Angle greater than zero - mostly the same as above. We'll invert the sign right here
            // to make the math nicer.
            wheel_angle = wheel_angle.abs();

            // r is same as above, and so is alpha
            let r = Car::WIDTH / (2.0 * f32::sin(wheel_angle / 2.0));
            let alpha = distance / r;

            // But the final coordinates are just a bit different. Again, shouldn't be too tricky
            // to see if you have a diagram in front of you.
            unrotated_pos_shift = Point {
                x: r * (alpha.cos() - 1.0),
                y: r * alpha.sin(),
            };
            angle_change = alpha;
        }

        // Now with our "unrotated" position shift, we need to figure out where the car is
        // *actually* going. This is essentially just applying a rotation to this vector, which
        // shouldn't be too bad.
        //
        // The result of a 2D rotation about the origin by some angle θ is:
        //
        //   x' = x*cos(θ) - y*sin(θ)
        //   y' = x*sin(θ) + y*cos(θ)
        //
        // (https://en.wikipedia.org/wiki/Rotation_matrix)
        //
        // We can rotate around the origin because this vector purely manages the *change* in the
        // position, which starts at zero.

        // use t, x, y as shorthands so this looks cleaner:
        let t = self.angle;
        let Point { x, y } = unrotated_pos_shift;

        let pos_change = Point {
            x: x * t.cos() - y * t.sin(),
            y: x * t.sin() + y * t.cos(),
        };

        // And then we finally update! Easy as pie :P
        self.pos += pos_change;
        self.angle += angle_change;
    }
}
