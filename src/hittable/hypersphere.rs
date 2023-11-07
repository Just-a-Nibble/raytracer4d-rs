use crate::{Hit, Hittable, Point4, Ray};

#[derive(Clone, Copy)]
pub struct Hypersphere {
    pub center: Point4,
    pub radius: f32,
}

impl Hypersphere {
    pub fn new(center: Point4, radius: f32) -> Self {
        Hypersphere { center, radius }
    }
}

/*
 * DERIVATION OF HYPERSPHERE INTERSECTION FUNCTION
 *
 * A hypersphere can be defined as all the points
 * of distance r from center C. Therefore, point P
 * is on a hypersphere iff:
 *   |P - C| = r
 *
 * With the definition of our ray function:
 *   R(t) = O + Dt
 * We want to find the point along the ray that
 * satisfies the hypersphere definition, so we
 * want to solve:
 *   |R(t) - C| = r
 *
 * So we do some algebra:
 *   |R(t) - C| = r                                Given
 *   sqrt((R(t) - C) * (R(t) - C)) = r             Definition of vector magnitude
 *   (R(t) - C) * (R(t) - C) = r^2                 Square both sides
 *   (Dt + O - C) * (Dt + O - C) = r^2             Substitute R(t) for Dt + O
 *   (Dt + u) * (Dt + u) = r^2                     Let u = O - C
 *   (D * D)t^2 + 2(D * u)t + (u * u) = r^2        Distributive property of dot product
 *   (D * D)t^2 + 2(D * u)t + (u * u) - r^2 = 0    Move r^2 to the other side
 *                                                 Note that all vectors have been dotted,
 *                                                   so we're working with only scalar constants.
 *   at^2 + bt + c = 0                             Replace scalars with a,b,c
 *     where a = D * D,                            Now, we have a quadratic equation for which
 *           b = 2(D * u),                           t values will result in intersection with
 *           c = (u * u) - r^2                     the defined hypersphere.
 *   t = (-b +/- sqrt(b^2 - 4ac)) / 2a             Using the quadratic formula, we can find those values
*/

impl Hittable for Hypersphere {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        let u = ray.origin - self.center;

        let a = ray.direction * ray.direction;
        let b = 2.0 * (ray.direction * u);
        let c = (u * u) - (self.radius * self.radius);

        // We're calculating the discriminant first because
        // we want to catch imaginary solution (no hits)
        // before we panic.
        let discriminant = (b * b) - (4.0 * a * c);
        if discriminant > 0.0 {
            // We're only calculating the minus root, because
            // we only care about the closest point.
            let t = (-b - discriminant.sqrt()) / (2.0 * a);

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
