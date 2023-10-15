use std::{fmt::Debug, rc::Rc};

use crate::{
    material::Material,
    ray::Ray,
    vector::{Point3, Vec3},
};

pub mod sphere;
pub use sphere::*;

pub trait Hit: Debug {
    fn hit(&self, ray: &Ray, t_range: Interval) -> Option<HitData>;
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Interval(pub f64, pub f64);

#[derive(Debug, Clone, Copy)]
pub struct HitData<'a> {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub material: &'a Material,
}

#[derive(Debug)]
pub struct World {
    pub objects: Vec<Rc<dyn Hit>>,
}

impl Interval {
    pub const UNIVERSE: Self = Self(f64::NEG_INFINITY, f64::INFINITY);
    pub const POSITIVE: Self = Self(0.0, f64::INFINITY);
    pub const NEGATIVE: Self = Self(f64::NEG_INFINITY, 0.0);
    pub const EMPTY: Self = Self(0.0, 0.0);

    pub fn contains(&self, value: f64) -> bool {
        self.0 < value && value < self.1
    }

    pub fn clamp(&self, value: f64) -> f64 {
        if value < self.0 {
            self.0
        } else if self.1 < value {
            self.1
        } else {
            value
        }
    }
}

impl Hit for World {
    fn hit(&self, ray: &Ray, t_range: Interval) -> Option<HitData> {
        let mut min_t = f64::INFINITY;
        let mut min_hit = None;
        for object in self.objects.iter() {
            if let Some(hit) = object.hit(ray, t_range) {
                if hit.t < min_t {
                    min_t = hit.t;
                    min_hit = Some(hit);
                }
            }
        }

        min_hit
    }
}
