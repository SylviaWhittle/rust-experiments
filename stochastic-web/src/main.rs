use byteorder::{LittleEndian, WriteBytesExt};
use image::{Rgb, RgbImage};
use plotters::prelude::*;
use std::f64::consts;
use std::fs;
use std::io::Write;
use std::path::Path;

fn convert_to_u32(vec: Vec<f64>, multiplier: u32) -> Vec<u32> {
    vec.into_iter()
        .map(|x| {
            if x < 0.0 {
                panic!("Negative value detected!")
            }
            (x * multiplier as f64).round() as u32
        })
        .collect()
}

fn simulate_kicked_pendula_low_ram_usage() -> Result<(), Box<dyn std::error::Error>> {
    // Initial conditions
    let x0: f64 = 1.7679;
    let p0: f64 = 1.7679;

    // Parameters

    let r: f64 = 1.00;
    // Kicking Frequency
    let q: f64 = 3.00;
    // Strength
    let k: f64 = -1.1;
    // Kick frequency
    let tau = 2.0 * consts::PI * r / q;

    // End time
    let end_time = 1000000000;

    // Create arrays
    let mut x: Vec<f64> = vec![0.0; end_time];
    let mut p: Vec<f64> = vec![0.0; end_time];

    // Construct image
    // a default (black) image containing Rgb values
    let img_size = 5000;
    let mut img = RgbImage::new(img_size + 1, img_size + 1);
    let mapping_maximum = 8.0;

    // Set initial points
    x[0] = x0;
    p[0] = p0;

    // Create a poincare section of position and momentum using a derived direct kick-to-kick mapping
    println!("cos tau: {}", tau.cos());
    let cos_tau = tau.cos();
    println!("sin tau: {}", tau.sin());
    let sin_tau = tau.sin();
    let sqrt_2 = 2.0_f64.sqrt();
    println!("sqrt 2: {}", sqrt_2);

    let mut x_prev: f64 = x[0];
    let mut p_prev: f64 = p[0];
    let mut x_this: f64;
    let mut p_this: f64;
    for i in 1..end_time {
        x_this = x_prev * cos_tau + (p_prev + k * (sqrt_2 * x_prev).sin()) * sin_tau;
        p_this = (p_prev + k * (sqrt_2 * x_prev).sin()) * cos_tau - x_prev * sin_tau;

        if x_this.abs() >= 0.0
            && x_this.abs() < mapping_maximum / 2.0
            && p_this.abs() >= 0.0
            && p_this.abs() < mapping_maximum / 2.0
        {
            x[i] = x_this;
            p[i] = p_this;
        }

        x_prev = x_this;
        p_prev = p_this;
    }

    println!("Finished simulation");

    println!("Scaling x and p vectors to fit the image");
    // Move -r -> r range to 0 -> 2r range to display in an image
    for i in 0..x.len() {
        x[i] += mapping_maximum / 2.0;
        p[i] += mapping_maximum / 2.0;
    }

    let multiplier = img_size / mapping_maximum as u32;
    let x_u32 = convert_to_u32(x, multiplier);
    let p_u32 = convert_to_u32(p, multiplier);

    // Fill image with white
    println!("Filling image with white");
    for j in 0..img_size {
        for i in 0..img_size {
            img.put_pixel(j, i, image::Rgb([255, 255, 255]))
        }
    }
    println!("Filled image with white");
    println!("Painting pixel data");
    for (val_x, val_p) in x_u32.iter().zip(p_u32.iter()) {
        img.put_pixel(*val_x, *val_p, image::Rgb([0, 0, 0]));
    }
    println!("Finished painting pixel data");

    println!("Saving image");
    img.save(Path::new("./output.png"))?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = simulate_kicked_pendula_low_ram_usage();

    Ok(())
}
