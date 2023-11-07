use crate::{Hit, Hittable, Point4, Ray, Vec4};

#[derive(Clone, Copy)]
pub struct Hyperplane {
    pub center: Point4,
    pub normal: Vec4,
}

impl Hyperplane {
    pub fn new(center: Point4, normal: Vec4) -> Self {
        Hyperplane {
            center: center,
            normal: normal.normalized(),
        }
    }

    pub fn new_from_points(a: Point4, b: Point4, c: Point4, d: Point4) -> Self {
        Hyperplane::new(a, Vec4::cross(a - b, a - c, a - d))
    }
}

/*
 * DERIVATION OF HYPERPLANE INTERSECTION FUNCTION
 *
 * A hyperplane can be defined as all points
 * that are at right angles to the normal relative
 * to the center. Therefore, point P is on a
 * hyperplane iff:
 *   (P - C) * N = 0
 *
 * With the definition of our ray function:
 *   R(t) = O + Dt
 * We want to find the point along the ray that
 * satisfies the hyperplane definition, so we
 * want to solve:
 *   (R(t) - C) * N = 0
 *
 * So we do some algebra:
 *   (R(t) - C) * N = 0                          Given
 *   (Dt + O - C) * N = 0                        Substitute R(t) for Dt + O
 *   (Dt + u) * N = 0                            Let u = O - C
 *   (D * N)t + (u * N) = 0                      Distributive property of dot product
 *   (D * N)t = -(u * N)                         Move (u * N) to the other side
 *   t = -(u * N) / (D * N)                      Solve for t
*/

impl Hittable for Hyperplane {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        let u = ray.origin - self.center;

        // Normal doesn't actually matter, potential
        // future optimization.
        let numerator = -(u * self.normal);
        let denominator = ray.direction * self.normal;

        if denominator != 0.0 {
            let t = numerator / denominator;

            if t > 0.0 {
                let position = ray.at(t);

                Some(Hit {
                    t: t,
                    position: position,
                })
            } else {
                None
            }
        } else {
            None
        }
    }
}
