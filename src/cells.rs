//! This file defines the different types of cells (particles) in our falling sand simulation
//! and their properties like density, temperature behavior, and special characteristics.

// Import the random number generator function from our main library
use crate::rand;

/// This struct defines all the properties that each type of cell can have.
/// The #[derive] attribute automatically implements several traits for our struct:
/// - Copy: Allows the struct to be copied by value instead of moved
/// - Clone: Provides a method to explicitly create a copy
/// - Debug: Allows the struct to be printed with {:?} format
/// - PartialEq: Allows comparison with == and !=
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CellTypeProperties {
    /// The display name of this cell type (static lifetime means it lives for the entire program)
    pub name: &'static str,
    /// The enum variant representing this cell type
    pub cell_type: CellType,
    /// How dense this cell is (affects falling behavior)
    pub density: f64,
    /// How quickly this cell heats up or cools down
    pub temp_coefficient: f32,
    /// Whether this cell can catch fire
    pub flammable: bool,
    /// Whether this cell can be dissolved by acid
    pub dissolvable: bool,
}

/// This enum defines all the different types of cells in our simulation.
/// Each variant represents a different material or element.
/// The derive attributes work the same as for the struct above.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CellType {
    /// Empty space (air)
    Empty,
    /// Solid rock that doesn't move
    Rock,
    /// Solid wood that can burn
    Wood,
    /// Granular material that falls
    Sand,
    /// Explosive material that can ignite
    Gunpowder,
    /// Liquid that flows
    Water,
    /// Flammable liquid that's less dense than water
    Oil,
    /// Flammable gas that rises
    Propane,
    /// Flame that can spread to flammable materials
    Fire,
    /// Hot liquid that can ignite things
    Lava,
    /// Dissolves materials it touches
    Acid,
}

/// This static array contains all the cell types (except Empty) for easy iteration
/// Static means this array exists for the entire program and has a fixed size
static CELL_TYPES: [CellType; 10] = [
    CellType::Rock,
    CellType::Wood,
    CellType::Sand,
    CellType::Gunpowder,
    CellType::Water,
    CellType::Oil,
    CellType::Propane,
    CellType::Fire,
    CellType::Lava,
    CellType::Acid,
];

/// This static array defines the specific properties for each cell type
/// The array must have an entry for each CellType variant, in the same order
/// as the enum definition above
static CELL_PROPERTIES: [CellTypeProperties; 11] = [
    // Empty cells have no density and minimal temperature effects
    CellTypeProperties {
        name: "Empty",
        cell_type: CellType::Empty,
        density: 0.0,                // No density (lightest)
        temp_coefficient: 0.1,       // Changes temperature very slowly
        flammable: false,            // Can't catch fire
        dissolvable: false,          // Can't be dissolved
    },
    // Rock is solid and stable
    CellTypeProperties {
        name: "Rock",
        cell_type: CellType::Rock,
        density: 3.0,                // High density (heavy)
        temp_coefficient: 0.1,       // Changes temperature very slowly
        flammable: false,            // Can't catch fire
        dissolvable: true,           // Can be dissolved by acid
    },
    // Wood is solid but can burn
    CellTypeProperties {
        name: "Wood",
        cell_type: CellType::Wood,
        density: 3.0,                // High density (heavy)
        temp_coefficient: 0.8,       // Changes temperature moderately
        flammable: true,             // Can catch fire
        dissolvable: true,           // Can be dissolved by acid
    },
    // Sand behaves like a flowing solid
    CellTypeProperties {
        name: "Sand",
        cell_type: CellType::Sand,
        density: 3.0,                // High density (heavy)
        temp_coefficient: 0.1,       // Changes temperature very slowly
        flammable: false,            // Can't catch fire
        dissolvable: true,           // Can be dissolved by acid
    },
    // Gunpowder is explosive when heated
    CellTypeProperties {
        name: "Gunpowder",
        cell_type: CellType::Gunpowder,
        density: 3.0,                // High density (heavy)
        temp_coefficient: 20.0,      // Heats up very quickly
        flammable: true,             // Can catch fire (and explode)
        dissolvable: false,          // Can't be dissolved by acid
    },
    // Water is a flowing liquid
    CellTypeProperties {
        name: "Water",
        cell_type: CellType::Water,
        density: 1.0,                // Medium density
        temp_coefficient: 0.4,       // Changes temperature slowly
        flammable: false,            // Can't catch fire
        dissolvable: false,          // Can't be dissolved by acid
    },
    // Oil is flammable and floats on water
    CellTypeProperties {
        name: "Oil",
        cell_type: CellType::Oil,
        density: 0.8,                // Less dense than water (floats on water)
        temp_coefficient: 10.0,      // Heats up quickly
        flammable: true,             // Can catch fire
        dissolvable: false,          // Can't be dissolved by acid
    },
    // Propane is a rising gas that's highly flammable
    CellTypeProperties {
        name: "Propane",
        cell_type: CellType::Propane,
        density: 0.1,                // Very low density (rises)
        temp_coefficient: 200.0,     // Heats up extremely quickly
        flammable: true,             // Can catch fire
        dissolvable: false,          // Can't be dissolved by acid
    },
    // Fire spreads to flammable materials and rises
    CellTypeProperties {
        name: "Fire",
        cell_type: CellType::Fire,
        density: 0.01,               // Extremely low density (rises quickly)
        temp_coefficient: 1.0,       // Normal temperature behavior
        flammable: false,            // Can't catch fire (it is fire)
        dissolvable: false,          // Can't be dissolved by acid
    },
    // Lava is hot and heavy
    CellTypeProperties {
        name: "Lava",
        cell_type: CellType::Lava,
        density: 3.0,                // High density (heavy)
        temp_coefficient: 100.0,     // Maintains high temperature
        flammable: false,            // Can't catch fire
        dissolvable: true,           // Can be dissolved by acid
    },
    // Acid dissolves other materials
    CellTypeProperties {
        name: "Acid",
        cell_type: CellType::Acid,
        density: 1.2,                // Slightly more dense than water (sinks in water)
        temp_coefficient: 0.1,       // Changes temperature very slowly
        flammable: false,            // Can't catch fire
        dissolvable: false,          // Can't be dissolved by acid (itself)
    },
];

/// This implementation block adds methods to the CellType enum
impl CellType {
    /// Creates a random cell type, with a bias toward Empty cells
    /// This is used for debugging or for special effects in the simulation
    pub fn random() -> CellType {
        // Generate a random number between 0 and 19 using our rand() function
        // Then use pattern matching to determine which cell type to return
        match (rand() * 20.0) as u32 {
            0 => CellType::Rock,
            1 => CellType::Sand,
            2 => CellType::Water,
            _ => CellType::Empty,    // Most of the time (17/20 chance), return Empty
        }
    }

    /// Returns an iterator over all cell types (except Empty)
    /// The 'a lifetime parameter indicates how long the returned iterator is valid
    pub fn iter<'a>() -> std::slice::Iter<'a, CellType> {
        CELL_TYPES.iter()    // Return an iterator to the CELL_TYPES array
    }

    /// Gets the properties for a given cell type
    /// The 'a lifetime parameter indicates how long the returned reference is valid
    pub fn get_properties<'a>(cell_type: CellType) -> &'a CellTypeProperties {
        // Look up the properties in the CELL_PROPERTIES array using the enum value as an index
        &CELL_PROPERTIES[cell_type as usize]
    }
}

/// This struct represents an actual cell in the simulation grid
/// Each position in our grid contains one of these cells
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Cell {
    /// What kind of material this cell is
    pub cell_type: CellType,
    /// Used to track updates in the simulation
    pub generation: u8,
    /// The temperature of this cell, affects behavior
    pub temp: f32,
}

/// This implementation block adds methods to the Cell struct
impl Cell {
    /// Creates a new empty cell (air)
    pub fn empty() -> Cell {
        Cell {
            cell_type: CellType::Empty,
            generation: 0,               // Starting generation
            temp: 20.0,                  // Room temperature in Celsius
        }
    }

    /// Creates a cell with a random type
    /// The #[allow(dead_code)] attribute silences warnings about this function not being used
    #[allow(dead_code)]
    pub fn random() -> Cell {
        Cell {
            cell_type: CellType::random(),   // Use the random() function from CellType
            temp: 20.0,                      // Room temperature
            generation: 0,                   // Starting generation
        }
    }

    /// Initializes a cell with a specific type, setting appropriate properties
    pub fn init(&mut self, cell_type: CellType) {
        self.cell_type = cell_type;

        // Set initial temperature based on cell type
        match cell_type {
            CellType::Lava => self.temp = 1000.0 + rand() as f32 * 1000.0,  // Lava is very hot (1000-2000°C)
            CellType::Fire => self.temp = rand() as f32 * 1000.0,           // Fire varies in temperature (0-1000°C)
            _ => self.temp = 20.0,                                          // Everything else starts at room temperature
        }
    }

    /// Gets the properties for this cell's type
    /// Convenience method that delegates to CellType::get_properties
    pub fn get_properties<'a>(&self) -> &'a CellTypeProperties {
        CellType::get_properties(self.cell_type)
    }
}

