
use std::rc::Rc;
use std::cell::RefCell;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{ Document, Window, HtmlElement, HtmlCanvasElement, CanvasRenderingContext2d, MouseEvent };

use crate::timer::Timer;
use crate::world::World;
use crate::space::Space;
use crate::cells::CellType;
use crate::input::InputTracker;
use crate::{ REFRESH, CELL_WIDTH, CELL_HEIGHT, alert, log };


pub fn init_dom(world: Rc<RefCell<World>>) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let canvas = document
        .get_element_by_id("space").unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| ()).unwrap();

    canvas.set_width(world.borrow().space.get_width() * CELL_WIDTH);
    canvas.set_height(world.borrow().space.get_height() * CELL_HEIGHT);

    init_button_events(&document, world.clone());
    init_mouse_events(&canvas, world.clone());
    init_draw_events(&window, canvas, world.clone());
}

fn init_button_events(document: &Document, world: Rc<RefCell<World>>) {
    {
        let world = world.clone();
        let cb = Closure::wrap(Box::new(move || {
            world.borrow_mut().toggle_run();
        }) as Box<dyn FnMut()>);

        document
            .get_element_by_id("play-pause").unwrap()
            .dyn_ref::<HtmlElement>().unwrap()
            .set_onclick(Some(cb.as_ref().unchecked_ref()));

        cb.forget();
    }

    let container = document
        .get_element_by_id("select-types").unwrap()
        .dyn_into::<HtmlElement>().unwrap();

    create_select_button(&document, &container, world.clone(), "Erase", CellType::Empty);
    create_select_button(&document, &container, world.clone(), "Rock", CellType::Rock);
    create_select_button(&document, &container, world.clone(), "Sand", CellType::Sand);
    create_select_button(&document, &container, world.clone(), "Water", CellType::Water);
    create_select_button(&document, &container, world.clone(), "Oil", CellType::Oil);
    create_select_button(&document, &container, world.clone(), "Propane", CellType::Propane);
    create_select_button(&document, &container, world.clone(), "Fire", CellType::Fire)

}

fn create_select_button(document: &Document, container: &HtmlElement, world: Rc<RefCell<World>>, name: &str, cell_type: CellType) {
    container.append_child(&create_button(&document, name, move || {
        world.borrow_mut().input.update_selected_type(cell_type);
    })).unwrap();
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

fn init_mouse_events(canvas: &HtmlCanvasElement, world: Rc<RefCell<World>>) {
    {
        let world = world.clone();
        let cb = Closure::wrap(Box::new(move |e: MouseEvent| {
            let x = e.offset_x() / CELL_WIDTH as i32;
            let y = e.offset_y() / CELL_HEIGHT as i32;
            world.borrow_mut().input.update_pos(x, y);
            world.borrow_mut().input.update_down(true);
        }) as Box<dyn FnMut(MouseEvent)>);

        canvas.set_onmousedown(Some(cb.as_ref().unchecked_ref()));
        cb.forget();
    }

    {
        let world = world.clone();
        let cb = Closure::wrap(Box::new(move |_: MouseEvent| {
            world.borrow_mut().input.update_down(false);
        }) as Box<dyn FnMut(MouseEvent)>);

        canvas.set_onmouseup(Some(cb.as_ref().unchecked_ref()));
        cb.forget();
    }

    let cb = Closure::wrap(Box::new(move |e: MouseEvent| {
        if world.borrow().input.is_down() {
            let x = e.offset_x() / CELL_WIDTH as i32;
            let y = e.offset_y() / CELL_HEIGHT as i32;
            world.borrow_mut().input.update_pos(x, y);
        }
    }) as Box<dyn FnMut(MouseEvent)>);

    canvas.set_onmousemove(Some(cb.as_ref().unchecked_ref()));
    cb.forget();
}

fn init_draw_events(window: &Window, canvas: HtmlCanvasElement, world: Rc<RefCell<World>>) { 
    let mut drawing = false;
    //let mut timer = Timer::start();

    let render_frame = Closure::wrap(Box::new(move || {
        if world.borrow().is_running() && !drawing {
            drawing = true;
            //log(format!("{}", timer.interval()).as_ref());
            world.borrow_mut().advance_simulation();
            draw_canvas(&canvas, &world.borrow().space);
            drawing = false;
        }
    }) as Box<dyn FnMut()>);

    //window.request_animation_frame(render_frame.as_ref().unchecked_ref());
    window.set_interval_with_callback_and_timeout_and_arguments_0(render_frame.as_ref().unchecked_ref(), REFRESH).unwrap();

    render_frame.forget();
}

fn draw_canvas(canvas: &HtmlCanvasElement, space: &Space) {
    let context = canvas
        .get_context("2d").unwrap().unwrap()
        .dyn_into::<CanvasRenderingContext2d>().unwrap();

    context.clear_rect(0.0, 0.0, (space.get_width() * CELL_WIDTH) as f64, (space.get_height() * CELL_HEIGHT) as f64);

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
        CellType::Oil => "#007777",
        CellType::Propane => "#77FFFF",
        CellType::Fire => "#FF3300",
    }
}

