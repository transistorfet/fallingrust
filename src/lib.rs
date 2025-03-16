// This is the main entry point for our WebAssembly-based falling sand simulation.
// In Rust, the lib.rs file serves as the primary file for libraries.

// Import standard library components we need:
// Rc (Reference Counted) allows multiple parts of our code to share ownership of data
use std::rc::Rc;
// RefCell provides interior mutability - a way to mutate data even when there are multiple references to it
use std::cell::RefCell;

// Import WebAssembly bindings which allow our Rust code to interact with JavaScript
use wasm_bindgen::prelude::*;

// These 'mod' declarations tell Rust about our other code modules.
// Each module is in its own separate file with the same name.
mod cells;     // Defines the different types of cells/particles in our simulation
mod space;     // Manages the grid where all our cells live
mod input;     // Handles user inputs like mouse clicks and movements
mod world;     // Represents the entire game world state
mod simulator; // Contains the core simulation logic
mod ui;        // Handles the user interface elements

// This attribute tells the Rust compiler not to warn about unused code in the timer module
#[allow(dead_code)]
mod timer;     // Provides timing functionality for the simulation

// This imports the World struct from our world module, making it accessible in this file
use crate::world::World;

// These 'static' variables define constants for our simulation.
// They are global values that can be accessed from anywhere in the code.
pub static REFRESH: i32 = 10;        // Controls how often the simulation updates (in milliseconds)
pub static SPACE_WIDTH: u32 = 1024;   // Width of the simulation space in cells
pub static SPACE_HEIGHT: u32 = 640;  // Height of the simulation space in cells
pub static CELL_WIDTH: u32 = 3;      // Width of each cell in pixels when rendered
pub static CELL_HEIGHT: u32 = 3;     // Height of each cell in pixels when rendered

// The #[wasm_bindgen] attribute exposes the following definitions to JavaScript
#[wasm_bindgen]
extern {
    // These declarations allow us to call JavaScript functions from our Rust code
    
    // Allows our Rust code to trigger browser alerts
    fn alert(s: &str);
    
    // Allows our Rust code to write messages to the browser's console
    // The js_namespace attribute specifies which JavaScript object the function belongs to
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// This is the main entry point function that will be called from JavaScript
// when our application starts
#[wasm_bindgen]
pub fn run() {
    // Log a welcome message to the browser console
    log("Welcome to Falling Rust!");

    // Create a new World object with our specified dimensions
    // Rc and RefCell allow us to share this world object between different parts of our code
    // while still being able to modify it
    let world = Rc::new(RefCell::new(World::new(SPACE_WIDTH, SPACE_HEIGHT)));

    // Initialize the DOM (Document Object Model) for our user interface
    // We pass our world object so the UI can interact with it
    ui::init_dom(world);
}

// A helper function that provides a random number generator
// This calls JavaScript's Math.random() function to get a random float between 0 and 1
pub fn rand() -> f64 {
    js_sys::Math::random()
}

// This macro provides a convenient way to print debug messages to the browser console
// Macros in Rust are a way to define reusable code patterns
#[macro_export]
macro_rules! debug_print {
    // If called with no arguments, just print a newline
    () => {
        log("\n");
    };
    // If called with arguments, format them as a string and log the result
    // The $($arg:tt)* syntax captures all the arguments passed to the macro
    ($($arg:tt)*) => {{
        log(&format!($($arg)*));
    }};
}

