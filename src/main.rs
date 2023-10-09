use std::fs::{self, File};

use anyhow::Result;
use raytracer::raytrace;

const IDEAL_ASPECT_RATIO: f64 = 16.0 / 9.0;
const WIDTH: usize = 800;

fn main() -> Result<()> {
    let args = std::env::args();
    let Some(file_name) = args.skip(1).next() else {
        panic!("No file_name was given.");
    };

    fs::create_dir_all("output")?;
    let file = File::create(format!("output/{}", file_name))?;

    raytrace(file, WIDTH, IDEAL_ASPECT_RATIO)?;

    Ok(())
}
