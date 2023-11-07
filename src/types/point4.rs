use std::ops::{Add, Sub};

use crate::Vec4;

#[derive(Clone, Copy)]
pub struct Point4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Point4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Point4 { x, y, z, w }
    }
}

impl Add<Vec4> for Point4 {
    type Output = Point4;

    fn add(self, rhs: Vec4) -> Self::Output {
        Point4 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl Sub<Vec4> for Point4 {
    type Output = Point4;

    fn sub(self, rhs: Vec4) -> Self::Output {
        Point4 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl Sub for Point4 {
    type Output = Vec4;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec4 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}
