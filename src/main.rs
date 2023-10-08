use std::{
    fs::{self, File},
    io::{BufWriter, Write},
};

use anyhow::Result;
use raytracer::{
    ray::Ray,
    vector::Point3,
    view::{Camera, Screen},
};

const IDEAL_ASPECT_RATIO: f64 = 16.0 / 9.0;
const WIDTH: usize = 400;

fn main() -> Result<()> {
    let args = std::env::args();
    let Some(file_name) = args.skip(1).next() else {
        panic!("No file_name was given.");
    };

    fs::create_dir_all("output")?;
    let file = File::create(format!("output/{}", file_name))?;
    let mut writer = BufWriter::new(file);

    let screen = Screen {
        height: (WIDTH as f64 / IDEAL_ASPECT_RATIO) as usize,
        width: WIDTH,
    };

    let camera = Camera::new(Point3::new(0.0, 0.0, 0.0), 1.0, 2.0, &screen);

    writer.write_fmt(format_args!(
        "P3\n{} {}\n255\n",
        screen.width, screen.height
    ))?;

    for i in 0..screen.height {
        print!("\rScanlines remaining: {}", (screen.height - i));
        for j in 0..screen.width {
            let pixel_center = camera.first_pixel + (i as f64 * camera.dv) + (j as f64 * camera.du);
            let ray_direction = pixel_center - camera.eye;

            let ray = Ray {
                origin: pixel_center,
                direction: ray_direction.into(),
            };

            ray.color().write_to(&mut writer)?;
        }
    }
    print!("\rDone!");
    println!("                               ");

    Ok(())
}
