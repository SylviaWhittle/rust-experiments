use byteorder::{LittleEndian, WriteBytesExt};
use image::{Rgb, RgbImage};
use plotters::prelude::*;
use std::f64::consts;
use std::fs;
use std::io::Write;
use std::path::Path;

const OUT_FILE_NAME: &'static str = "./image.png";
const SCALING: u32 = 1000;

fn filter_vectors(
    vec1: Vec<f64>,
    vec2: Vec<f64>,
    lower_lim: f64,
    upper_lim: f64,
) -> (Vec<f64>, Vec<f64>) {
    let mut res1 = Vec::new();
    let mut res2 = Vec::new();
    for (v1, v2) in vec1.into_iter().zip(vec2.into_iter()) {
        if v1.abs() >= lower_lim / 2.0
            && v1.abs() < upper_lim / 2.0
            && v2.abs() >= lower_lim / 2.0
            && v2.abs() < upper_lim / 2.0
        {
            res1.push(v1 + upper_lim / 2.0);
            res2.push(v2 + upper_lim / 2.0);
        }
    }
    (res1, res2)
}

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initial conditions
    let x0: f64 = 1.7679;
    let p0: f64 = 1.7679;

    // Parameters

    let r: f64 = 1.00;
    // Kicking Frequency
    let q: f64 = 3.00;
    // Strength
    // let k: f64 = -1.14;
    // Kick frequency
    let tau = 2.0 * consts::PI * r / q;

    // End time
    let end_time = 10000000;

    // Create arrays
    let mut x: Vec<f64> = vec![0.0; end_time];
    let mut p: Vec<f64> = vec![0.0; end_time];

    for j in (1100..1101).step_by(3) {
        let k: f64 = -(j as f64 / 1000.0 as f64);
        println!("k: {k}");

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

        for i in 1..end_time {
            // println!("i: {i}");
            let x_prev = x[i - 1];
            // println!("x prev: {}", x_prev);
            let p_prev = p[i - 1];
            // println!("p prev: {}", p_prev);

            x[i] = x_prev * cos_tau + (p_prev + k * (sqrt_2 * x_prev).sin()) * sin_tau;
            p[i] = (p_prev + k * (sqrt_2 * x_prev).sin()) * cos_tau - x_prev * sin_tau;
        }

        println!("Finished simulation");
        // println!("x:");
        // println!("{:?}", x);
        // println!("p:");
        // println!("{:?}", p);

        println!("Saving results");
        let data_directory: &Path = Path::new("./data/");

        let mut buffer = Vec::new();
        for &value in &x {
            buffer.write_f64::<LittleEndian>(value)?;
        }
        for &value in &p {
            buffer.write_f64::<LittleEndian>(value)?;
        }
        if !Path::new(data_directory).exists() {
            fs::create_dir(data_directory)?;
        }
        fs::write(
            data_directory.join(Path::new(&format!("data_k-{}.bin", j))),
            &buffer,
        )?;

        // println!("x: {:?}", x);
        // println!("p: {:?}", p);
    }

    // a default (black) image containing Rgb values
    let img_size = 5000;
    let mut img = RgbImage::new(img_size + 1, img_size + 1);
    let mapping_maximum = 8.0;
    // println!("x: {:?}", x);
    println!("len x: {:?}", x.len());
    // let filtered_x = filter_vector(x, 0.0, mapping_maximum);
    // let filtered_p = filter_vector(p, 0.0, mapping_maximum);
    let (filtered_x, filtered_p) = filter_vectors(x, p, 0.0, mapping_maximum);
    // println!("filtered x: {:?}", filtered_x);
    println!("len filtered x: {:?}", filtered_x.len());
    // println!("filtered p: {:?}", filtered_p);
    let multiplier = img_size / mapping_maximum as u32;
    let filtered_x = convert_to_u32(filtered_x, multiplier);
    let filtered_p = convert_to_u32(filtered_p, multiplier);
    // println!("integer filtered x: {:?}", filtered_x);
    // println!("integer filtered p: {:?}", filtered_p);
    for (val_x, val_p) in filtered_x.iter().zip(filtered_p.iter()) {
        img.put_pixel(*val_x, *val_p, image::Rgb([255, 255, 255]));
    }

    img.save(Path::new("./output.png"))?;

    Ok(())

    // println!("Ploting results");

    // let mut max_x: f64 = x[0];
    // for x_val in &x {
    //     if x_val > &max_x {
    //         max_x = *x_val;
    //     }
    // }

    // println!("Max x: {max_x}");

    // let mut max_p: f64 = p[0];
    // for p_val in &p {
    //     if p_val > &max_p {
    //         max_p = *p_val;
    //     }
    // }

    // println!("Max p: {max_p}");

    // let root = BitMapBackend::new(
    //     OUT_FILE_NAME,
    //     (max_x.ceil() as u32 * SCALING, max_p.ceil() as u32 * SCALING),
    // )
    // .into_drawing_area();
    // root.fill(&WHITE)?;

    // let mut chart = ChartBuilder::on(&root)
    //     .margin(20)
    //     .x_label_area_size(10)
    //     .y_label_area_size(10)
    //     .build_cartesian_2d(-2.1f64..0.6f64, -1.2f64..1.2f64)?;

    // chart
    //     .configure_mesh()
    //     .disable_x_mesh()
    //     .disable_y_mesh()
    //     .draw()?;

    // let plotting_area = chart.plotting_area();

    // for (val_x, val_p) in x.iter().zip(p.iter()) {
    //     plotting_area.draw_pixel(
    //         (
    //             ((*val_x * SCALING as f64) as u32).into(),
    //             ((*val_p * SCALING as f64) as u32).into(),
    //         ),
    //         &BLACK,
    //     );
    // }

    // Ok(())
}
