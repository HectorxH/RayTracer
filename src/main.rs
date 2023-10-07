fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for i in 0..image_height {
        for j in 0..image_width {
            let r = j as f32 / (image_width - 1) as f32;
            let g = i as f32 / (image_height - 1) as f32;
            let b = 0.0;

            let r = (r * 255.999) as i32;
            let g = (g * 255.999) as i32;
            let b = (b * 255.999) as i32;

            println!("{} {} {}", r, g, b);
        }
    }
}
