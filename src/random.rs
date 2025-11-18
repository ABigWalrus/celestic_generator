use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use rand::{Rng, SeedableRng, rng, rngs};

use crate::perlin::Perlin;

pub fn create_noise_png() {
    let path = Path::new(r"./test.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let width = 255;
    let height = 255;
    let mut encoder = png::Encoder::new(w, width, height); // Width is 2 pixels and height is 1.
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455)); // 1.0 / 2.2, scaled by 100000
    encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2)); // 1.0 / 2.2, unscaled, but rounded
    let source_chromaticities = png::SourceChromaticities::new(
        // Using unscaled instantiation here
        (0.31270, 0.32900),
        (0.64000, 0.33000),
        (0.30000, 0.60000),
        (0.15000, 0.06000),
    );
    encoder.set_source_chromaticities(source_chromaticities);
    let mut writer = encoder.write_header().unwrap();

    // let seed: i32 = 42;
    // let rng = rngs::StdRng::from_seed(seed.to_le_bytes());
    let mut rng = rng();
    let mut data = Vec::with_capacity(width as usize * height as usize * 4);
    for y in 0..height {
        for x in 0..width {
            let random: f32 = rng.random();
            for i in 0..3 {
                // println!("{}", random);
                data.push((random * 255.0) as u8);
            }
            data.push(255);
        }
    }
    writer.write_image_data(&data).unwrap();
}

pub fn create_perlin_noise_png() {
    let path = Path::new(r"./perlin.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let width = 255;
    let height = 255;
    let mut encoder = png::Encoder::new(w, width, height); // Width is 2 pixels and height is 1.
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455)); // 1.0 / 2.2, scaled by 100000
    encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2)); // 1.0 / 2.2, unscaled, but rounded
    let source_chromaticities = png::SourceChromaticities::new(
        // Using unscaled instantiation here
        (0.31270, 0.32900),
        (0.64000, 0.33000),
        (0.30000, 0.60000),
        (0.15000, 0.06000),
    );
    encoder.set_source_chromaticities(source_chromaticities);
    let mut writer = encoder.write_header().unwrap();

    // let seed: i32 = 42;
    // let rng = rngs::StdRng::from_seed(seed.to_le_bytes());
    // let mut rng = rng();
    let mut data = Vec::with_capacity(width as usize * height as usize * 4);
    let perlin = Perlin::new();
    for y in 0..height {
        for x in 0..width {
            // let random: f32 = rng.random();
            let noise = perlin
                .noise(x as f64 / width as f64, y as f64 / height as f64, 0.0)
                .abs();
            println!("{}", noise);
            for i in 0..3 {
                // println!("{}", random);
                data.push((noise * 255.0) as u8);
            }
            data.push(255);
        }
    }
    writer.write_image_data(&data).unwrap();
}
