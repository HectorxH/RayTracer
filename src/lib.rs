use crate::material::Material;
use crate::objects::{Sphere, World};
use crate::vector::{Color, Point3, Vec3};
use crate::view::{Camera, Screen};
use std::f64::consts::PI;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::rc::Rc;

use anyhow::Result;

mod material;
mod objects;
mod ray;
mod vector;
mod view;

const EPS: f64 = 0.00001;

pub fn raytrace(file: File, width: usize, aspect: f64, sample_size: usize) -> Result<()> {
    let mut writer = BufWriter::new(file);

    let screen = Screen {
        height: (width as f64 / aspect) as usize,
        width,
    };

    writer.write_fmt(format_args!(
        "P3\n{} {}\n255\n",
        screen.width, screen.height
    ))?;

    let materials = [
        Material::Lambertian {
            albedo: Color::new(0.8, 0.8, 0.0),
        },
        Material::Lambertian {
            albedo: Color::new(0.1, 0.2, 0.5),
        },
        Material::Dielectric { ir: 1.5 },
        Material::Metallic {
            albedo: Color::new(0.8, 0.6, 0.2),
            fuzz: 0.0,
        },
    ];

    let world = World {
        objects: vec![
            Rc::new(Sphere {
                center: Point3::new(0.0, -100.5, -1.0),
                radius: 100.0,
                material: materials[0],
            }),
            Rc::new(Sphere {
                center: Point3::new(0.0, 0.0, -1.0),
                radius: 0.5,
                material: materials[1],
            }),
            Rc::new(Sphere {
                center: Point3::new(-1.0, 0.0, -1.0),
                radius: 0.5,
                material: materials[2],
            }),
            Rc::new(Sphere {
                center: Point3::new(-1.0, 0.0, -1.0),
                radius: -0.4,
                material: materials[2],
            }),
            Rc::new(Sphere {
                center: Point3::new(1.0, 0.0, -1.0),
                radius: 0.5,
                material: materials[3],
            }),
        ],
    };
    let camera = Camera::new(
        Point3::new(-2.0, 2.0, 1.0),
        Point3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        f64::to_radians(20.0),
        f64::to_radians(10.0),
        3.4,
        &screen,
    );

    for i in 0..screen.height {
        print!("\rScanlines remaining: {}", (screen.height - i));
        for j in 0..screen.width {
            let mut color = Color::default();

            let ray = camera.ray_at(i, j);
            color += ray.color(&world);

            for _ in 0..(sample_size - 1) {
                let ray = camera.sample_at(i, j);
                color += ray.color(&world);
            }

            color /= sample_size as f64;

            color.write_to(&mut writer)?;
        }

        print!("                               ");
    }
    println!("\rDone!                             ");

    Ok(())
}
