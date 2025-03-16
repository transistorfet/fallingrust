// This file handles the user interface for our simulation.
// It connects our Rust code to the browser's DOM and handles rendering and user input.

// Import standard library components for reference counting and interior mutability
use std::rc::Rc;
use std::cell::RefCell;

// Import WebAssembly and browser-related modules
use wasm_bindgen::prelude::*;           // For JavaScript interop
use wasm_bindgen::JsCast;               // For type casting between JavaScript and Rust types
use web_sys::{ Document, Window, HtmlElement, HtmlCanvasElement, CanvasRenderingContext2d, MouseEvent, TouchEvent };

// Import our game modules
use crate::world::World;
use crate::space::Space;
use crate::cells::CellType;
use crate::{ REFRESH, CELL_WIDTH, CELL_HEIGHT }; // Constants from lib.rs

// These imports are marked with #[allow(unused_imports)] to silence warnings
// about them not being used directly (they might be used in commented-out code)
#[allow(unused_imports)]
use crate::timer::Timer;
#[allow(unused_imports)]
use crate::{ alert, log};

// Main function to initialize the DOM and set up all event handlers
// Takes a reference-counted, refcell-wrapped World instance that will be shared
// among event handlers
pub fn init_dom(world: Rc<RefCell<World>>) {
    // Get references to the browser's window and document objects
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    // Get the canvas element where we'll draw the simulation
    let canvas = document
        .get_element_by_id("space").unwrap()       // Find element with id="space"
        .dyn_into::<HtmlCanvasElement>()           // Cast it to canvas element type
        .map_err(|_| ()).unwrap();                 // Handle errors

    // Set canvas dimensions based on simulation size and cell dimensions
    canvas.set_width(world.borrow().space.get_width() * CELL_WIDTH);
    canvas.set_height(world.borrow().space.get_height() * CELL_HEIGHT);

    // Initialize all event handlers
    init_button_events(&document, world.clone());  // UI buttons (play/pause, cell type selection)
    init_mouse_events(&canvas, world.clone());     // Mouse input on canvas
    init_touch_events(&canvas, world.clone());     // Touch input for mobile devices
    init_draw_events(&window, canvas, world.clone()); // Animation loop for rendering
}

// Sets up event handlers for UI buttons and creates cell type selection buttons
fn init_button_events(document: &Document, world: Rc<RefCell<World>>) {
    // Set up the play/pause button
    {
        let world = world.clone(); // Clone the Rc to create a new reference
        register_click(document, "play-pause", move || {
            world.borrow_mut().toggle_run(); // Toggle simulation state when button is clicked
        });
    }

    // Get the container for cell type selection buttons
    let container = document
        .get_element_by_id("select-types").unwrap()
        .dyn_into::<HtmlElement>().unwrap();

    // Create a button for erasing (placing Empty cells)
    create_select_button(&document, &container, world.clone(), "Erase", CellType::Empty);
    
    // Create buttons for each cell type
    for cell_type in CellType::iter() {
        let props = CellType::get_properties(*cell_type);
        create_select_button(&document, &container, world.clone(), props.name, *cell_type);
    }
}

// Creates a button for selecting a specific cell type
fn create_select_button(document: &Document, container: &HtmlElement, world: Rc<RefCell<World>>, name: &str, cell_type: CellType) {
    // Create a button with the given name that updates the selected cell type when clicked
    container.append_child(&create_button(&document, name, move || {
        world.borrow_mut().input.update_selected_type(cell_type);
    })).unwrap();
}

// Helper function to create a button element with a click handler
fn create_button<F>(document: &Document, name: &str, f: F) -> HtmlElement 
    where F: 'static + Fn() -> () { // Function type constraint: must be static and take no arguments
    // Wrap the Rust closure in a form that can be called from JavaScript
    let cb = Closure::wrap(Box::new(f) as Box<dyn FnMut()>);

    // Create a new button element
    let el = document
        .create_element("button").unwrap()
        .dyn_into::<HtmlElement>().unwrap();

    // Set the button text and click handler
    el.set_inner_html(name);
    el.set_onclick(Some(cb.as_ref().unchecked_ref()));

    // Leaks memory! But necessary for the callback to remain valid
    // 'forget' tells Rust not to clean up this closure when it goes out of scope
    cb.forget();
    el // Return the button element
}

// Attaches a click handler to an existing DOM element by ID
fn register_click<F>(document: &Document, element_id: &str, f: F) 
    where F: 'static + Fn() -> () {
    // Wrap the Rust closure in a form that can be called from JavaScript
    let cb = Closure::wrap(Box::new(f) as Box<dyn FnMut()>);

    // Find the element by ID and set its click handler
    document
        .get_element_by_id(element_id).unwrap()
        .dyn_ref::<HtmlElement>().unwrap()
        .set_onclick(Some(cb.as_ref().unchecked_ref()));

    // Leaks memory! But necessary for the callback to remain valid
    cb.forget();
}

// Sets up mouse event handlers on the canvas
fn init_mouse_events(canvas: &HtmlCanvasElement, world: Rc<RefCell<World>>) {
    // Mouse down (button press) handler
    {
        let world = world.clone();
        let cb = Closure::wrap(Box::new(move |e: MouseEvent| {
            // Convert pixel coordinates to grid coordinates by dividing by cell size
            let x = e.offset_x() / CELL_WIDTH as i32;
            let y = e.offset_y() / CELL_HEIGHT as i32;
            world.borrow_mut().input.update_pos(x, y);
            world.borrow_mut().input.update_down(true);
        }) as Box<dyn FnMut(MouseEvent)>);

        canvas.set_onmousedown(Some(cb.as_ref().unchecked_ref()));
        cb.forget();
    }

    // Mouse up (button release) handler
    {
        let world = world.clone();
        let cb = Closure::wrap(Box::new(move |_: MouseEvent| {
            world.borrow_mut().input.update_down(false);
        }) as Box<dyn FnMut(MouseEvent)>);

        canvas.set_onmouseup(Some(cb.as_ref().unchecked_ref()));
        cb.forget();
    }

    // Mouse move handler
    {
        let cb = Closure::wrap(Box::new(move |e: MouseEvent| {
            // Only update position if mouse button is down (dragging)
            if world.borrow().input.is_down() {
                let x = e.offset_x() / CELL_WIDTH as i32;
                let y = e.offset_y() / CELL_HEIGHT as i32;
                world.borrow_mut().input.update_pos(x, y);
            }
        }) as Box<dyn FnMut(MouseEvent)>);

        canvas.set_onmousemove(Some(cb.as_ref().unchecked_ref()));
        cb.forget();
    }
}

// Sets up touch event handlers on the canvas (for mobile devices)
fn init_touch_events(canvas: &HtmlCanvasElement, world: Rc<RefCell<World>>) {
    // Touch start handler (finger touches screen)
    {
        let world = world.clone();
        let cb = Closure::wrap(Box::new(move |e: TouchEvent| {
            // Get position of first touch point
            let x = e.target_touches().get(0).unwrap().client_x() / CELL_WIDTH as i32;
            let y = e.target_touches().get(0).unwrap().client_y() / CELL_HEIGHT as i32;
            world.borrow_mut().input.update_pos(x, y);
            world.borrow_mut().input.update_down(true);
        }) as Box<dyn FnMut(TouchEvent)>);

        canvas.set_ontouchstart(Some(cb.as_ref().unchecked_ref()));
        cb.forget();
    }

    // Touch end handler (finger lifts off screen)
    {
        let world = world.clone();
        let cb = Closure::wrap(Box::new(move |_: TouchEvent| {
            world.borrow_mut().input.update_down(false);
        }) as Box<dyn FnMut(TouchEvent)>);

        canvas.set_ontouchend(Some(cb.as_ref().unchecked_ref()));
        cb.forget();
    }

    // Touch move handler (finger moves while touching screen)
    {
        let cb = Closure::wrap(Box::new(move |e: TouchEvent| {
            if world.borrow().input.is_down() {
                let x = e.target_touches().get(0).unwrap().client_x() / CELL_WIDTH as i32;
                let y = e.target_touches().get(0).unwrap().client_y() / CELL_HEIGHT as i32;
                world.borrow_mut().input.update_pos(x, y);
            }
        }) as Box<dyn FnMut(TouchEvent)>);

        canvas.set_ontouchmove(Some(cb.as_ref().unchecked_ref()));
        cb.forget();
    }
}

// Sets up the animation loop for rendering the simulation
fn init_draw_events(window: &Window, canvas: HtmlCanvasElement, world: Rc<RefCell<World>>) { 
    let mut drawing = false;  // Flag to prevent overlapping draw operations
    //let mut timer = Timer::start();  // Uncommented timer for performance measurement

    // This closure will be called repeatedly to update and render the simulation
    let render_frame = Closure::wrap(Box::new(move || {
        // Only proceed if simulation is running and not currently drawing
        if world.borrow().is_running() && !drawing {
            drawing = true;  // Set flag to prevent concurrent updates
            //log(format!("{}", timer.interval()).as_ref());  // Uncommented timing code
            
            // Advance the simulation by one step
            world.borrow_mut().advance_simulation();
            
            // Draw the current state to the canvas
            draw_canvas(&canvas, &world.borrow().space);
            
            drawing = false;  // Clear flag to allow next frame
        }
    }) as Box<dyn FnMut()>);

    // Set up the animation/simulation loop
    //window.request_animation_frame(render_frame.as_ref().unchecked_ref());  // Uncommented code for requestAnimationFrame
    
    // Instead, use setInterval for a consistent frame rate
    window.set_interval_with_callback_and_timeout_and_arguments_0(
        render_frame.as_ref().unchecked_ref(),  // The callback to execute
        REFRESH  // How often to run (in milliseconds)
    ).unwrap();

    // Leaks memory! But necessary for the callback to remain valid
    render_frame.forget();
}

// Draws the current simulation state to the canvas
fn draw_canvas(canvas: &HtmlCanvasElement, space: &Space) {
    // Get the 2D rendering context from the canvas
    let context = canvas
        .get_context("2d").unwrap().unwrap()
        .dyn_into::<CanvasRenderingContext2d>().unwrap();

    // Clear the canvas
    context.clear_rect(0.0, 0.0, 
        (space.get_width() * CELL_WIDTH) as f64, 
        (space.get_height() * CELL_HEIGHT) as f64);

    // Batch drawing by cell type for better performance
    // Rather than changing color for each cell, we draw all cells of the same type at once
    for draw_type in CellType::iter() {
        // Set the fill color for this cell type
        context.set_fill_style(&JsValue::from(cell_type_to_colour(*draw_type)));
        context.begin_path();
        
        // Find all cells of this type and add them to the current path
        for y in 0..space.get_height() {
            for x in 0..space.get_width() {
                let cell_type = space.get_cell_type(x, y);
                if cell_type == *draw_type {
                    context.rect(
                        (x * CELL_WIDTH) as f64, 
                        (y * CELL_HEIGHT) as f64, 
                        CELL_WIDTH as f64, 
                        CELL_HEIGHT as f64
                    );
                }
            }
        }
        // Fill all cells of this type at once
        context.fill();
    }
}

// Converts a cell type to its display color
fn cell_type_to_colour(cell_type: CellType) -> &'static str {
    match cell_type {
        CellType::Empty => "#FFFFFF",      // White
        CellType::Rock => "#000000",       // Black
        CellType::Wood => "#606040",       // Dark olive
        CellType::Sand => "#886611",       // Brown
        CellType::Gunpowder => "#666666",  // Dark gray
        CellType::Water => "#0000FF",      // Blue
        CellType::Oil => "#007777",        // Teal
        CellType::Propane => "#77FFFF",    // Light cyan
        CellType::Fire => "#FF3300",       // Orange-red
        CellType::Lava => "#993300",       // Dark orange
        CellType::Acid => "#009966",       // Green
    }
}

