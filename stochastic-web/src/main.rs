use byteorder::{LittleEndian, WriteBytesExt};
use plotters::prelude::*;
use std::f64::consts;
use std::fs;
use std::io::Write;
use std::path::Path;

const OUT_FILE_NAME: &'static str = "./image.png";
const SCALING: u32 = 1000;

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
    let end_time = 80000000;

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
