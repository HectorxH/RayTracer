use crate::objects::{Hit, Interval, Sphere, World};
use crate::ray::Ray;
use crate::vector::{Color, Point3};
use crate::view::{Camera, Screen};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::rc::Rc;

use anyhow::Result;

pub mod objects;
pub mod ray;
pub mod vector;
pub mod view;

const EPS: f64 = 1e-30;

pub fn raytrace(file: File, width: usize, aspect: f64) -> Result<()> {
    let mut writer = BufWriter::new(file);

    let screen = Screen {
        height: (width as f64 / aspect) as usize,
        width,
    };

    writer.write_fmt(format_args!(
        "P3\n{} {}\n255\n",
        screen.width, screen.height
    ))?;

    let world = World {
        objects: vec![
            Rc::new(Sphere {
                center: Point3::new(0.0, 0.0, -1.0),
                radius: 0.5,
            }),
            Rc::new(Sphere {
                center: Point3::new(0.0, -100.5, -1.0),
                radius: 100.0,
            }),
        ],
    };
    let camera = Camera::new(Point3::new(0.0, 0.0, 0.0), 1.0, 2.0, &screen);

    for i in 0..screen.height {
        print!("\rScanlines remaining: {}", (screen.height - i));
        for j in 0..screen.width {
            let pixel_center = camera.first_pixel + (i as f64 * camera.dv) + (j as f64 * camera.du);
            let ray_direction = pixel_center - camera.eye;

            let ray = Ray {
                origin: pixel_center,
                direction: ray_direction.into(),
            };

            let color = if let Some(hit) = world.hit(&ray, Interval::UNIVERSE) {
                Color::new(1.0, 1.0, 1.0).lerp(&hit.normal, 0.5)
            } else {
                ray.bg_color()
            };

            color.write_to(&mut writer)?;
        }
    }

    print!("\rDone!");
    println!("                               ");

    Ok(())
}
