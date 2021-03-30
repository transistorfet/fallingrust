
use std::rc::Rc;
use std::cell::RefCell;

use wasm_bindgen::prelude::*;

mod cells;
mod space;
mod input;
mod world;
mod simulator;
mod ui;

#[allow(dead_code)]
mod timer;

use crate::world::World;

pub static REFRESH: i32 = 10;
pub static SPACE_WIDTH: u32 = 320;
pub static SPACE_HEIGHT: u32 = 160;
pub static CELL_WIDTH: u32 = 3;
pub static CELL_HEIGHT: u32 = 3;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn run() {
    log("Welcome to Falling Rust!");

    let world = Rc::new(RefCell::new(World::new(SPACE_WIDTH, SPACE_HEIGHT)));

    ui::init_dom(world);
}

pub fn rand() -> f64 {
    js_sys::Math::random()
}

