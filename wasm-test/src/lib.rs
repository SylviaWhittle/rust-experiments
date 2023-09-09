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
}
