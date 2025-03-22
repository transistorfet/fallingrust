//! This file defines the World struct, which brings together all the components
//! of our simulation: the space (grid), input handling, and simulation logic.

// Import the components we need for our world
/// The grid where cells live
use crate::space::Space;
/// Tracks user input
use crate::input::InputTracker;
/// Simulation algorithms
use crate::simulator::{ Simulator, SwappingSim, CellularSim };

/// The World struct is the main container for our simulation
/// It coordinates all the different parts and represents the entire game state
pub struct World {
    /// Whether the simulation is currently running or paused
    pub run: bool,
    /// The grid containing all our cells
    pub space: Space,
    /// Tracks user mouse input
    pub input: InputTracker,
    /// The simulation algorithm to use (boxed trait object)
    simulator: Box<dyn Simulator>,
}

impl World {
    /// Creates a new world with the given dimensions
    pub fn new(width: u32, height: u32) -> World {
        World {
            run: true,                        // Start with the simulation running
            space: Space::new(width, height), // Create a new empty space with the given dimensions
            input: InputTracker::new(),       // Initialize input tracking
            simulator: Box::new(SwappingSim { }), // Use the SwappingSim algorithm
            //simulator: Box::new(CellularSim { }), // Alternative simulator (commented out)
        }
    }

    /// Toggles whether the simulation is running or paused
    pub fn toggle_run(&mut self) {
        self.run = !self.run; // Flip the boolean value
    }

    /// Returns whether the simulation is currently running
    pub fn is_running(&self) -> bool {
        self.run
    }

    /// Advances the simulation by one step
    pub fn advance_simulation(&mut self) {
        // If the mouse is down, get its position and add cells
        if let Some((x, y)) = self.input.get_pos() {
            // Add a small offset every other frame for a nicer drawing effect
            let offset = if self.space.get_generation() % 2 == 0 { 0 } else { 1 };
            // Add cells of the selected type at the mouse position
            self.space.add(x + offset, y, self.input.get_selected_type());
        }
        
        // Run one tick of the simulation using the current simulator
        self.simulator.tick(&mut self.space);
    }
}

