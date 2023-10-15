use std::rc::Rc;

use crate::{material::Material, vector::Point3};

use super::{Hit, HitData, Interval};

#[derive(Debug)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub material: Material,
}

impl Hit for Sphere {
    fn hit(&self, ray: &crate::ray::Ray, t_range: Interval) -> Option<HitData> {
        let oc = ray.origin - self.center;

        let a = ray.direction.length_squared();
        let b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius.powi(2);

        let discriminant = b.powi(2) - a * c;

        let sqrt_disc = discriminant.sqrt();

        let mut t = (-b - sqrt_disc) / a;
        if !t_range.contains(t) {
            t += (2.0 * sqrt_disc) / a;
        }

        if t_range.contains(t) && discriminant >= 0.0 {
            let point = ray.at(t);
            let normal = ((point - self.center) / self.radius).into();

            Some(HitData {
                point,
                normal,
                material: &self.material,
                t,
            })
        } else {
            None
        }
    }
}
