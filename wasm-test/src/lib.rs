use rand::Rng;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>()?;
    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    draw_random_points(&context);

    Ok(())
}

fn draw_random_points(context: &CanvasRenderingContext2d) {
    let mut rng = rand::thread_rng();

    for _ in 0..100 {
        let x: f64 = rng.gen_range(0.0..800.0);
        let y: f64 = rng.gen_range(0.0..800.0);

        context.begin_path();
        context
            .arc(x, y, 2.0, 0.0, std::f64::consts::PI * 2.0)
            .unwrap();
        context.fill();
    }

    let data = vec![
        vec![0.1, 0.2, 0.3],
        vec![0.4, 0.5, 0.6],
        vec![0.7, 0.8, 0.9],
    ];

    draw_2d_vec(context, &data);
}

fn draw_2d_vec(context: &CanvasRenderingContext2d, data: &[Vec<f64>]) {
    let square_size = 20.0;
    for (row_index, row) in data.iter().enumerate() {
        for (col_index, value) in row.iter().enumerate() {
            let x: f64 = col_index as f64 * square_size;
            let y: f64 = row_index as f64 * square_size;

            context.set_fill_style(&JsValue::from_str(&format!("rgba(0, 0, 0, {}", value)));
            context.fill_rect(x, y, square_size, square_size)
        }
    }
}
