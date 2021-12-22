use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
pub fn start(width: u32) {
    const X_LENGTH: f64 = 960.0;
    const Y_LENGTH: f64 = 560.0;
    const PADDING: f64 = 20.0;

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
    canvas.style().set_property("border", "solid").unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    
    context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);

    context.begin_path();

    context.line_to(PADDING, Y_LENGTH + PADDING);
    context.line_to(X_LENGTH + PADDING, Y_LENGTH + PADDING);
    context.move_to(PADDING, Y_LENGTH + PADDING);
    context.line_to(PADDING, PADDING);
    context.move_to(PADDING, Y_LENGTH + PADDING);

    let mut cl = Vec::new();
    cl.push(width);

    for i in Collatz::new(width).into_iter() {
        cl.push(i);
    }

    let max = *cl.iter().max().unwrap() as f64;
    let scale_x = Y_LENGTH / max;

    let len = cl.len() as f64;
    let scale_y = X_LENGTH / (len - 1.0);

    for (i, item) in cl.into_iter().enumerate() {
        context.line_to(
            PADDING + i as f64 * scale_y,
            PADDING + Y_LENGTH - (scale_x * item as f64),
        );
        log!("{}, {}", i, item);
    }

    context.stroke();
}

struct Collatz {
    seed: u32,
}

impl Collatz {
    fn new(seed: u32) -> Collatz {
        Collatz { seed }
    }
}

impl Iterator for Collatz {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.seed % 2 == 0 {
            self.seed /= 2;
            Some(self.seed)
        } else if self.seed == 1 {
            None
        } else {
            self.seed = self.seed * 3 + 1;
            Some(self.seed)
        }
    }
}
