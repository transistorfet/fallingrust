
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{ Document, Window, HtmlElement, HtmlCanvasElement, CanvasRenderingContext2d, MouseEvent };

mod cells;
mod space;
mod input;
mod misc;

use crate::space::Space;
use crate::cells::CellType;
use crate::input::InputTracker;
use crate::misc::{ shared, Shared, Timer };

/*
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
*/

#[wasm_bindgen]
extern {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}


static REFRESH: i32 = 10;

static SPACE_WIDTH: u32 = 320;
static SPACE_HEIGHT: u32 = 160;
static CELL_WIDTH: u32 = 4;
static CELL_HEIGHT: u32 = 4;


#[wasm_bindgen]
pub fn run() {
    log("Welcome to Falling Rust!");

    let space = shared(Space::new(SPACE_WIDTH, SPACE_HEIGHT));
    let input = shared(InputTracker::new());

    init_dom(space, input);
}

fn init_dom(space: Shared<Space>, input: Shared<InputTracker>) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let canvas = document
        .get_element_by_id("space").unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| ()).unwrap();

    canvas.set_width(SPACE_WIDTH * CELL_WIDTH);
    canvas.set_height(SPACE_HEIGHT * CELL_HEIGHT);

    init_button_events(&document, space.clone(), input.clone());
    init_mouse_events(&canvas, input.clone());
    init_draw_events(&window, canvas, space.clone(), input.clone());
}

fn init_button_events(document: &Document, space: Shared<Space>, input: Shared<InputTracker>) {
    let cb = Closure::wrap(Box::new(move || {
        space.borrow_mut().toggle_run();
    }) as Box<dyn FnMut()>);

    document
        .get_element_by_id("play-pause").unwrap()
        .dyn_ref::<HtmlElement>().unwrap()
        .set_onclick(Some(cb.as_ref().unchecked_ref()));

    cb.forget();

    let container = document
        .get_element_by_id("select-types").unwrap()
        .dyn_into::<HtmlElement>().unwrap();

    create_select_button(&document, &container, input.clone(), "Rock", CellType::Rock);
    create_select_button(&document, &container, input.clone(), "Sand", CellType::Sand);
    create_select_button(&document, &container, input.clone(), "Water", CellType::Water);

}

fn create_select_button(document: &Document, container: &HtmlElement, input: Shared<InputTracker>, name: &str, cell_type: CellType) {
    container.append_child(&create_button(&document, name, move || {
        input.borrow_mut().update_selected_type(cell_type);
    }));
}

fn create_button<F>(document: &Document, name: &str, f: F) -> HtmlElement where F: 'static + Fn() -> () {
    let cb = Closure::wrap(Box::new(f) as Box<dyn FnMut()>);

    let el = document
        .create_element("button").unwrap()
        .dyn_into::<HtmlElement>().unwrap();

    el.set_inner_html(name);
    el.set_onclick(Some(cb.as_ref().unchecked_ref()));

    cb.forget();
    el
}

fn init_mouse_events(canvas: &HtmlCanvasElement, input: Shared<InputTracker>) {
    {
        let input = input.clone();
        let cb = Closure::wrap(Box::new(move |e: MouseEvent| {
            let x = e.offset_x() / CELL_WIDTH as i32;
            let y = e.offset_y() / CELL_HEIGHT as i32;
            input.borrow_mut().update_pos(x, y);
            input.borrow_mut().update_down(true);
        }) as Box<dyn FnMut(MouseEvent)>);

        canvas.set_onmousedown(Some(cb.as_ref().unchecked_ref()));
        cb.forget();
    }

    {
        let input = input.clone();
        let cb = Closure::wrap(Box::new(move |_: MouseEvent| {
            input.borrow_mut().update_down(false);
        }) as Box<dyn FnMut(MouseEvent)>);

        canvas.set_onmouseup(Some(cb.as_ref().unchecked_ref()));
        cb.forget();
    }

    let cb = Closure::wrap(Box::new(move |e: MouseEvent| {
        if input.borrow().is_down() {
            let x = e.offset_x() / CELL_WIDTH as i32;
            let y = e.offset_y() / CELL_HEIGHT as i32;
            input.borrow_mut().update_pos(x, y);
        }
    }) as Box<dyn FnMut(MouseEvent)>);

    canvas.set_onmousemove(Some(cb.as_ref().unchecked_ref()));
    cb.forget();
}

fn init_draw_events(window: &Window, canvas: HtmlCanvasElement, space: Shared<Space>, input: Shared<InputTracker>) { 
    let mut drawing = false;
    let mut timer = Timer::start();

    let render_frame = Closure::wrap(Box::new(move || {
        if space.borrow().is_running() && !drawing {
            drawing = true;
            log(format!("{}", timer.interval()).as_ref());
            advance_simulation(&mut space.borrow_mut(), &input);
            draw_canvas(&canvas, &space.borrow());
            drawing = false;
        }
    }) as Box<dyn FnMut()>);

    //window.request_animation_frame(render_frame.as_ref().unchecked_ref());
    window.set_interval_with_callback_and_timeout_and_arguments_0(render_frame.as_ref().unchecked_ref(), REFRESH).unwrap();

    render_frame.forget();
}

fn advance_simulation(space: &mut Space, input: &Shared<InputTracker>) {
    if let Some((x, y)) = input.borrow().get_pos() {
        space.add(x, y, input.borrow().get_selected_type());
    }
    space.tick();
}

fn draw_canvas(canvas: &HtmlCanvasElement, space: &Space) {
    let context = canvas
        .get_context("2d").unwrap().unwrap()
        .dyn_into::<CanvasRenderingContext2d>().unwrap();

    for draw_type in CellType::iter() {
        // Setting the style once for each type, rather than changing it for each cell in one pass through the cells, speeds up the drawing quite a bit
        context.set_fill_style(&JsValue::from(cell_type_to_colour(*draw_type)));
        context.begin_path();
        for y in 0..space.get_height() {
            for x in 0..space.get_width() {
                let cell_type = space.get_cell_type(x, y);
                if cell_type == *draw_type {
                    context.rect((x * CELL_WIDTH) as f64, (y * CELL_HEIGHT) as f64, CELL_WIDTH as f64, CELL_HEIGHT as f64);
                }
            }
        }
        context.fill();
    }
}

fn cell_type_to_colour(cell_type: CellType) -> &'static str {
    match cell_type {
        CellType::Empty => "#FFFFFF",
        CellType::Rock => "#000000",
        CellType::Sand => "#886611",
        CellType::Water => "#0000FF",
    }
}

pub fn rand() -> f64 {
    js_sys::Math::random()
}

