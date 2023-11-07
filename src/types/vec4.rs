use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Copy)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Vec4 { x, y, z, w }
    }

    pub fn magnitude(&self) -> f32 {
        (*self * *self).sqrt()
    }

    pub fn normalized(&self) -> Self {
        *self / self.magnitude()
    }

    pub fn cross(a: Vec4, b: Vec4, c: Vec4) -> Self {
        /*
         * | i  j  k  l|
         * |ax ay az aw|     |ay az aw|     |ax az aw|    |ax ay aw|    |ax ay az|
         * |bx by bz bw| = i*|by bz bw| - j*|bx bz bw| + k|bx by bw| - l|bx by bz|
         * |cx cy cz cw|     |cy cz cw|     |cx cz cw|    |cx cy cw|    |cx cy cz|
         */

        let c_xy = (b.x * c.y) - (b.y * c.x);
        let c_yz = (b.y * c.z) - (b.z * c.y);
        let c_xz = (b.x * c.z) - (b.z * c.x);
        let c_xw = (b.x * c.w) - (b.w * c.x);
        let c_yw = (b.y * c.w) - (b.w * c.y);
        let c_zw = (b.z * c.w) - (b.w * c.z);

        Vec4 {
            x: a.y * c_zw - a.z * c_yw + a.w * c_yz,
            y: a.x * c_zw - a.z * c_xw + a.w * c_xz,
            z: a.x * c_yw - a.y * c_xw + a.w * c_xy,
            w: a.x * c_yz - a.y * c_xz + a.z * c_xy,
        }
    }
}

impl Add for Vec4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec4 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl Sub for Vec4 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec4 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl Neg for Vec4 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec4 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Mul<f32> for Vec4 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec4 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl Div<f32> for Vec4 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Vec4 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

impl Mul for Vec4 {
    type Output = f32;

    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }
}
