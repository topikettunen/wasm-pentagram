mod utils;
mod float_iter;

use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub struct Pentagram {}

impl Pentagram {
    pub fn draw(ctx: web_sys::CanvasRenderingContext2d,
                x: f64,
                y: f64,
                radius: f64,
                rotate: f64,
                _circle: bool) -> () {
        ctx.set_line_width(20.0);
        ctx.begin_path();

        let start = 0.0;
        let stop = 4.0 * f64::consts::PI;
        let step = (4.0 * f64::consts::PI) / 5.0;

        for i in float_iter::FloatIterator::new_with_step(start, stop, step) {
            ctx.line_to(
                x + radius * (i as f64 + rotate).cos(),
                y + radius * (i as f64 + rotate).sin()
            );
        }
        
        ctx.move_to(x + radius, y);
        ctx.arc(x, y, radius, 0 as f64, f64::consts::PI * 2 as f64).unwrap();
        
        ctx.stroke()
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    utils::set_panic_hook();
    
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    Pentagram::draw(ctx, 250.0, 250.0, 200.0, f64::consts::PI / 2.0, true)
}
