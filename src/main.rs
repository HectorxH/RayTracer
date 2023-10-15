use std::fs::{self, File};

use anyhow::Result;
use raytracer::raytrace;

const IDEAL_ASPECT_RATIO: f64 = 16.0 / 9.0;
const WIDTH: usize = 800;

const SAMPLE_SIZE: usize = 16;

fn main() -> Result<()> {
    let mut args = std::env::args();
    let Some(file_name) = args.nth(1) else {
        panic!("No file_name was given.");
    };

    fs::create_dir_all("output")?;
    let file = File::create(format!("output/{}", file_name))?;

    raytrace(file, WIDTH, IDEAL_ASPECT_RATIO, SAMPLE_SIZE)?;

    Ok(())
}
