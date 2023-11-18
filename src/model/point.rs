use std::ops::{Add, Mul, Sub};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

pub trait Positioned {
    fn position(&self) -> Point;
}

impl Point {
    pub fn distance(&self, rhs: Point) -> f32 {
        ((self.x - rhs.x).powi(2) + (self.y - rhs.y).powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Point {
        let len = self.distance(Point::default());
        Point {
            x: (self.x / len),
            y: (self.y / len),
        }
    }
}

impl Add<Point> for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<Point> for Point {
    type Output = Point;
    fn sub(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<f32> for Point {
    type Output = Point;
    fn mul(self, rhs: f32) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
