use crate::{Point4, Ray};

pub mod hypersphere;
pub use hypersphere::Hypersphere;

pub mod hyperplane;
pub use hyperplane::Hyperplane;

#[derive(Clone, Copy)]
pub struct Hit {
    pub t: f32,
    pub position: Point4,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray) -> Option<Hit>;
}
