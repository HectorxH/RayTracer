use crate::{
    objects::{Hit, Interval, World},
    vector::{Color, Point3, Vec3},
    EPS,
};

#[derive(Debug, Default, Clone, Copy)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    const MAX_BOUNCES: usize = 50;
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }

    pub fn color(self, world: &World) -> Color {
        let mut ray = self;
        let mut color = Color::new(1.0, 1.0, 1.0);
        for _ in 0..Self::MAX_BOUNCES {
            if let Some(hit) = world.hit(&ray, Interval(EPS, f64::INFINITY)) {
                let attenuation = hit.material.scatter(&mut ray, hit);
                color *= attenuation;
            } else {
                return color * ray.bg_color();
            }
        }
        Color::new(0.0, 0.0, 0.0)
    }

    pub fn bg_color(&self) -> Color {
        let unit_dir = self.direction.normalized();
        let a = 0.5 * (unit_dir.y + 1.0);

        Color::new(1.0, 1.0, 1.0).lerp(&Color::new(0.5, 0.7, 1.0), a)
    }
}
