//! This file defines the simulation space (grid) that contains all the cells.
//! It manages the 2D grid layout and provides methods to access and manipulate cells.

// Import the Cell and CellType from our cells module
use crate::cells::{ Cell, CellType };

/// The Space struct represents the entire simulation grid
/// It keeps track of dimensions, all cells, and the current simulation generation
pub struct Space {
    /// Width of the grid in cells
    width: u32,
    /// Height of the grid in cells
    height: u32,
    /// Current simulation step/generation (used for update tracking)
    generation: u8,
    /// A flat vector storing all cells in row-major order
    cells: Vec<Cell>,
}

impl Space {
    /// Creates a new Space with the given dimensions
    /// Initializes all cells as empty (air)
    pub fn new(width: u32, height: u32) -> Self {
        // Calculate the total number of cells needed
        let length = width * height;
        // Preallocate memory for performance (with_capacity avoids reallocations)
        let mut cells = Vec::with_capacity(length as usize);

        // Fill the space with empty cells
        for _ in 0..length {
            cells.push(Cell::empty());
        }

        // Create and return the Space struct
        Space {
            width: width,
            height: height,
            generation: 0,       // Start at generation 0
            cells: cells,        // Our vector of cells
        }
    }

    /// Returns the width of the space
    pub fn get_width(&self) -> u32 {
        self.width
    }

    /// Returns the height of the space
    pub fn get_height(&self) -> u32 {
        self.height
    }

    /// Returns the current generation/tick of the simulation
    /// This is used to track which cells have been updated in the current simulation step
    pub fn get_generation(&self) -> u8 {
        self.generation
    }

    /// Increments the generation counter for the simulation
    /// This is called at the end of each simulation step
    pub fn increment_generation(&mut self) {
        self.generation += 1;
    }

    /// Updates the generation of a specific cell to match the current simulation generation
    /// This marks the cell as "updated" for the current simulation step
    pub fn update_cell_generation(&mut self, i: usize) {
        self.cells[i].generation = self.generation;
    }

    /// Checks if a cell needs updating in the current simulation step
    /// Returns true if the cell's generation doesn't match the current simulation generation
    pub fn cell_needs_updating(&self, i: usize) -> bool {
        self.generation != self.cells[i].generation
    }

    /// Converts 2D coordinates to a 1D array index
    /// This is used to access cells in our flat vector
    pub fn get_index(&self, x: u32, y: u32) -> usize {
        (x + (y * self.width)) as usize
    }

    /// Converts 2D coordinates to a 1D array index, with bounds checking
    /// Returns None if the coordinates are outside the grid boundaries
    pub fn get_index_checked(&self, x: i32, y: i32) -> Option<usize> {
        // Check if the coordinates are within bounds
        if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
            None    // Return None if outside bounds
        } else {
            // Convert to unsigned and get the index
            Some(self.get_index(x as u32, y as u32))
        }
    }

    /// Adds a cell of the specified type at the given coordinates
    /// Used for placing cells with the mouse
    pub fn add(&mut self, x: i32, y: i32, cell_type: CellType) {
        let mut created = 0;
        // Loop through a 5x5 grid centered at (x,y)
        for dy in -2..=2 {   // -2, -1, 0, 1, 2
            for dx in -2..=2 {   // -2, -1, 0, 1, 2
                // Check if this position is within bounds
                if let Some(i) = self.get_index_checked(x + dx, y + dy) {
                    // Only replace every other cell, and only if the target is empty
                    // or we're trying to erase (place empty cells)
                    if created % 2 == 0 && (self.cells[i].cell_type == CellType::Empty || cell_type == CellType::Empty) {
                        self.cells[i].init(cell_type);
                    }
                    created += 1;
                }
            }
        }
    }

    /// Gets the cell type at the specified coordinates
    pub fn get_cell_type(&self, x: u32, y: u32) -> CellType {
        let i = self.get_index(x, y);
        self.cells[i].cell_type
    }

    /// Gets a mutable reference to the cell at the specified index
    pub fn get_cell_at<'a>(&'a mut self, i: usize) -> &'a mut Cell {
        &mut self.cells[i]
    }

    /// Gets the cell type at the specified index
    pub fn get_cell_type_at(&self, i: usize) -> CellType {
        self.cells[i].cell_type
    }

    /// Swaps two cells in the grid
    /// Used by the simulation algorithm to move cells
    pub fn swap_cells(&mut self, i: usize, j: usize) {
        // Store the cell at index i
        let i_cell = self.cells[i];
        // Replace cell at index i with cell at index j
        self.cells[i] = self.cells[j];
        // Replace cell at index j with the stored cell
        self.cells[j] = i_cell;

        // Mark the cell at index i as updated in this generation
        self.cells[i].generation = self.generation;
    }

    /// Sets a cell at the specified index to be a copy of the provided cell
    pub fn set_cell(&mut self, i: usize, cell: &Cell) {
        // Copy the cell to the specified index
        self.cells[i] = *cell;

        // Mark the cell as updated in this generation
        self.cells[i].generation = self.generation;
    }
}

