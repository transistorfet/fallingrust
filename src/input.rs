// This file handles user input for the simulation, like mouse clicks and movements.
// It keeps track of mouse state and which cell type the user has selected.

// Import the CellType enum from our cells module
use crate::cells::CellType;

// The InputTracker struct keeps track of the user's input state
pub struct InputTracker {
    down: bool,               // Whether the mouse button is currently pressed
    x: i32,                   // Current mouse X coordinate in the grid
    y: i32,                   // Current mouse Y coordinate in the grid
    selected_type: CellType,  // The type of cell the user wants to place
}

impl InputTracker {
    // Creates a new InputTracker with default values
    pub fn new() -> InputTracker {
        InputTracker {
            down: false,              // Start with mouse button up
            x: 0,                     // Initial X position
            y: 0,                     // Initial Y position
            selected_type: CellType::Sand, // Default to Sand cell type
        }
    }

    // Updates whether the mouse button is pressed or not
    pub fn update_down(&mut self, down: bool) {
        self.down = down;
    }

    // Updates the current mouse position in grid coordinates
    pub fn update_pos(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    // Changes the selected cell type that will be placed when clicking
    pub fn update_selected_type(&mut self, cell_type: CellType) {
        self.selected_type = cell_type;
    }

    // Returns whether the mouse button is currently down
    pub fn is_down(&self) -> bool {
        self.down
    }

    // Returns the current mouse position, but only if the mouse button is down
    // If the mouse button is up, returns None
    // This is an example of the Option type in Rust, which represents either Some value or None
    pub fn get_pos(&self) -> Option<(i32, i32)> {
        match self.down {
            true => Some((self.x, self.y)),   // If mouse is down, return the position
            false => None                      // If mouse is up, return None
        }
    }

    // Returns the currently selected cell type
    pub fn get_selected_type(&self) -> CellType {
        self.selected_type
    }
}

