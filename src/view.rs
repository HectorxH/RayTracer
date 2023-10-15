#![allow(dead_code)]

use crate::{
    ray::Ray,
    vector::{Point3, Vec3},
};

pub struct Screen {
    pub height: usize,
    pub width: usize,
}

pub struct Viewport {
    pub height: f64,
    pub width: f64,
}

pub struct Camera {
    pub eye: Point3,
    pub look_at: Point3,
    pub up: Vec3,
    pub viewport: Viewport,
    pub top_left: Point3,
    pub first_pixel: Point3,
    pub du: Vec3,
    pub dv: Vec3,
    pub defocus_angle: f64,
    pub defocus_u: Vec3,
    pub defocus_v: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
}

impl Camera {
    pub fn new(
        eye: Point3,
        look_at: Point3,
        up: Vec3,
        vfov: f64,
        defocus_angle: f64,
        focus_dist: f64,
        screen: &Screen,
    ) -> Self {
        let h = f64::tan(vfov / 2.0);
        let viewport = Viewport {
            height: 2.0 * h * focus_dist,
            width: 2.0 * h * focus_dist * (screen.width as f64 / screen.height as f64),
        };

        let w = Vec3::from(eye - look_at).normalized();
        let u = up.cross(&w).normalized();
        let v = w.cross(&u);

        let viewport_u = u * viewport.width;
        let viewport_v = v * -viewport.height;

        let du = viewport_u / screen.width as f64;
        let dv = viewport_v / screen.height as f64;

        let top_left = eye - w * focus_dist - (viewport_u + viewport_v) / 2.0;
        let first_pixel = top_left + (du + dv) / 2.0;

        let defocus_radius = focus_dist * f64::tan(defocus_angle / 2.0);
        let defocus_u = u * defocus_radius;
        let defocus_v = v * defocus_radius;

        Self {
            eye,
            look_at,
            up,
            viewport,
            top_left,
            first_pixel,
            du,
            dv,
            defocus_angle,
            defocus_u,
            defocus_v,
            w,
            u,
            v,
        }
    }

    pub fn ray_at(&self, i: usize, j: usize) -> Ray {
        let pixel_center = self.first_pixel + (i as f64 * self.dv) + (j as f64 * self.du);
        let origin = if self.defocus_angle > 0.0 {
            self.eye + self.defocus_disk_sample()
        } else {
            self.eye
        };
        let ray_direction = pixel_center - origin;

        Ray {
            origin,
            direction: ray_direction.into(),
        }
    }

    pub fn sample_at(&self, i: usize, j: usize) -> Ray {
        let delta = self.dv * (fastrand::f64() - 0.5) + self.du * (fastrand::f64() - 0.5);
        let mut ray = self.ray_at(i, j);
        ray.direction += delta;
        ray
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let v = Vec3::random_in_disc();
        Point3::from(v.x * self.defocus_u + v.y * self.defocus_v)
    }
}
