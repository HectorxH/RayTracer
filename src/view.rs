#![allow(dead_code)]

use crate::vector::{Point3, Vec3};

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
    pub viewport: Viewport,
    pub top_left: Point3,
    pub first_pixel: Point3,
    pub du: Vec3,
    pub dv: Vec3,
}

impl Camera {
    pub fn new(eye: Point3, focal_length: f64, vh: f64, screen: &Screen) -> Self {
        let viewport = Viewport {
            height: vh,
            width: 2.0 * (screen.width as f64 / screen.height as f64),
        };

        let u = Vec3::new(viewport.width, 0.0, 0.0);
        let v = Vec3::new(0.0, -viewport.height, 0.0);

        let du = u / screen.width as f64;
        let dv = v / screen.height as f64;

        let top_left = eye - Vec3::new(0.0, 0.0, focal_length) - (u + v) / 2.0;
        let first_pixel = top_left + (du + dv) / 2.0;

        Self {
            eye,
            viewport,
            top_left,
            first_pixel,
            du,
            dv,
        }
    }
}
