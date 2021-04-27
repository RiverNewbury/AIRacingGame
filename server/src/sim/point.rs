//! Wrapper module for the [`Point`] type

use pyo3::prelude::pyclass;
use serde::Serialize;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// An (x, y) pair, used to represent points within the region allocated to the racetrack
#[pyclass]
#[derive(Copy, Clone, Serialize, Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    /// Produces a new point with the x-coordinate increased by the given amount
    pub fn add_x(self, x_inc: f32) -> Self {
        Point {
            x: self.x + x_inc,
            ..self
        }
    }

    /// Produces a new point with the y-coordinate increased by the given amount
    pub fn add_y(self, y_inc: f32) -> Self {
        Point {
            y: self.y + y_inc,
            ..self
        }
    }
    // Gives euclidean distance from point to (0,0)
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    // Produces a new point radius away from the origin at an angle, anticlockwise from +ive x
    pub fn new_polar(radius: f32, angle: f32) -> Point {
        Point {
            x: angle.cos() * radius,
            y: angle.sin() * radius,
        }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Point) {
        *self = *self + rhs;
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Point {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Point) {
        *self = *self - rhs;
    }
}

// Scalar multiplication
impl Mul<f32> for Point {
    type Output = Point;

    fn mul(self, rhs: f32) -> Point {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl MulAssign<f32> for Point {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl Div<f32> for Point {
    type Output = Point;

    fn div(self, rhs: f32) -> Point {
        Point {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl DivAssign<f32> for Point {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}
