use crate::vector::{Color, Point3, Vec3};

#[derive(Debug, Default, Clone, Copy)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }

    pub fn bg_color(&self) -> Color {
        let unit_dir = self.direction.normalized();
        let a = 0.5 * (unit_dir.y + 1.0);

        Color::new(1.0, 1.0, 1.0).lerp(&Color::new(0.5, 0.7, 1.0), a)
    }

    pub fn hit_color(&self) -> Color {
        Color::new(1.0, 0.0, 0.0)
    }
}
