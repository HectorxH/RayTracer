use crate::vector::Vec3;
use crate::Color;
use crate::{objects::HitData, ray::Ray};

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Lambertian { albedo: Color },
    Metallic { albedo: Color, fuzz: f64 },
    Dielectric { ir: f64 },
}

impl Material {
    pub fn scatter(&self, r: &mut Ray, hit: HitData) -> Color {
        match self {
            Material::Lambertian { albedo } => {
                r.direction = hit.normal + Vec3::random_unit();
                r.origin = hit.point;

                if r.direction.is_zero() {
                    r.direction = hit.normal;
                }

                *albedo
            }
            Material::Metallic { albedo, fuzz } => {
                r.direction =
                    r.direction.normalized().reflect(&hit.normal) + Vec3::random_unit() * fuzz;
                r.origin = hit.point;
                *albedo
            }
            Material::Dielectric { ir } => {
                let mut coef = *ir;
                let mut n = hit.normal;
                let dir = r.direction.normalized();
                let mut cos = dir.dot(&n);
                if cos < 0.0 {
                    coef = coef.recip();
                    n *= -1.0;
                    cos *= -1.0;
                }
                let sin = f64::sqrt(1.0 - cos * cos);

                if coef * sin > 1.0 || reflectance(cos, coef) > fastrand::f64() {
                    r.direction = dir.reflect(&hit.normal);
                } else {
                    r.direction = dir.refract(&n, coef);
                }
                r.origin = hit.point;

                Color::new(1.0, 1.0, 1.0)
            }
        }
    }
}

fn reflectance(cos: f64, coef: f64) -> f64 {
    let r0 = (1.0 - coef) / (1.0 + coef);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cos).powi(5)
}
