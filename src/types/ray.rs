use crate::{Point4, Vec4};

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Point4,
    pub direction: Vec4,
}

impl Ray {
    pub fn new(origin: Point4, direction: Vec4) -> Ray {
        // This is to ensure that the ray's direction
        // is of length 1. It isn't necessary, and has
        // a major performance drawback.
        // We may want to remove in the future.

        Ray {
            origin,
            direction: direction.normalized(),
        }
    }

    pub fn at(&self, t: f32) -> Point4 {
        self.origin + (self.direction * t)
    }
}
