//! This file contains the core simulation logic for our falling sand game.
//! It defines how different cell types behave and interact with each other.

use std::convert::TryInto;

// Import utility functions and macros
#[allow(unused_imports)]
use crate::{ log, rand, debug_print };
// Import our game modules
use crate::space::Space;
use crate::cells::{ Cell, CellType, CellTypeProperties };

/// The Simulator trait defines a common interface for different simulation approaches
/// Any struct that implements this trait can be used as the simulation engine
pub trait Simulator {
    /// The tick method advances the simulation by one time step
    fn tick(&mut self, space: &mut Space);
}

/// SwappingSim is our main simulation implementation
/// It uses a simple "swapping" approach where cells can exchange places with their neighbors
pub struct SwappingSim;

/// Implement the Simulator trait for SwappingSim
impl Simulator for SwappingSim {
    fn tick(&mut self, space: &mut Space) {
        // Delegate to our main simulation method
        self.advance(space);
    }
}

impl SwappingSim {
    /// The main simulation method that advances the simulation by one time step
    pub fn advance(&mut self, space: &mut Space) {
        // Increment the generation counter (used to track which cells have been updated)
        space.increment_generation();

        // Process the grid from bottom to top (important for gravity simulation)
        for y in (0..(space.get_height() as i32)).rev() {
            for x in 0..(space.get_width() as i32) {
                // Alternate the direction we scan each row to reduce directional bias
                // Sweeping left to right to left removes the bias in the x direction, but not the y direction
                // I'm not yet sure why the bias exists in the first place, but I suspect it has to do with the generation/update tracking
                let x = if y % 2 == 0 { x } else { space.get_width() as i32 - x - 1 };

                // Get the index of this cell in our flat array
                let i = space.get_index(x as u32, y as u32);

                // Only simulate cells that haven't already been moved this generation
                // This prevents a cell from being updated multiple times in one tick
                if space.cell_needs_updating(i) {
                    // Apply the simulation rules to this cell
                    self.simulate_cell(space, x, y);
                    // Mark this cell as updated for this generation
                    space.update_cell_generation(i);
                }
            }
        }
    }

    /// Simulates behavior for a single cell based on its type
    pub fn simulate_cell(&mut self, space: &mut Space, x: i32, y: i32) {
        // Get the index and cell at the current position
        let i = space.get_index(x as u32, y as u32);
        let cell = space.get_cell_at(i);

        // Apply different behavior based on the cell type
        match cell.cell_type {
            // Empty space doesn't do anything
            CellType::Empty => { },

            // Fixed solid materials (don't move)
            CellType::Wood |
            CellType::Rock => {
                // These don't move, so no action needed
            },

            // Granular materials (like sand - fall down and pile up)
            CellType::Sand |
            CellType::Gunpowder => {
                self.move_granular(space, x, y);
            },

            // Liquids (flow down and spread out)
            CellType::Oil |
            CellType::Water => {
                self.move_liquid(space, x, y);
            },

            // Gases (rise and spread)
            CellType::Propane => {
                self.move_gas(space, x, y);
            },

            // Acid dissolves other materials it touches
            CellType::Acid => {
                let mut expend = false;
                // Check each neighboring cell
                self.foreach_neighbour(space, x, y, |cell, props| {
                    // If the neighbor can be dissolved and we pass a random check
                    if props.dissolvable && rand() < 0.005 {
                        // Dissolve the neighbor (turn it into empty space)
                        cell.init(CellType::Empty);
                        // Mark that the acid was used up
                        expend = true;
                    }
                });

                // If the acid was used to dissolve something, it gets used up
                if expend {
                    let cell = space.get_cell_at(i);
                    cell.init(CellType::Empty);
                } else {
                    // Otherwise, it behaves like a liquid
                    self.move_liquid(space, x, y);
                }
            },

            // Lava is hot and can ignite things
            CellType::Lava => {
                // Lava cools down over time
                cell.temp -= rand() as f32 * 5.0;
                // If it cools enough, it turns into rock
                if cell.temp < 10.0 {
                    cell.cell_type = CellType::Rock;
                }

                // Lava can ignite neighboring flammable materials
                self.ignite_neighbours(cell.temp, space, x, y);
                // Lava also flows like a liquid
                self.move_liquid(space, x, y);
            },

            // Fire burns and spreads
            CellType::Fire => {
                // Fire loses heat over time
                cell.temp -= rand() as f32 * 5.0;
                // If it cools enough, it goes out
                if cell.temp < 10.0 {
                    cell.cell_type = CellType::Empty;
                    cell.temp = 0.0;
                }

                // Hot fire has a chance to spawn new fire cells nearby
                if cell.temp > 300.0 && rand() < 0.4 {
                    // Reduce temperature when spreading
                    cell.temp = cell.temp * 3.0 / 5.0;
                    let cell = *cell;
                    // Create a new fire cell in a nearby position
                    self.spawn_new(space, x, y, cell);
                }

                // Get the updated cell and try to ignite neighbors
                let cell = space.get_cell_at(i);
                self.ignite_neighbours(cell.temp, space, x, y);

                // Fire also has a chance to rise like a gas
                if rand() < 0.25 {
                    self.move_gas(space, x, y);
                }
            },
        }
    }

    /// Handles movement for granular materials like sand
    fn move_granular(&mut self, space: &mut Space, x: i32, y: i32) {
        let i = space.get_index(x as u32, y as u32);
        // Randomly choose left or right for the diagonal movement
        let d = if rand() > 0.5 { 1 } else { -1 };

        // Check these positions in order: directly below, diagonally below
        let check = vec!(
            (x, y + 1),           // Directly below
            (x + d, y + 1),       // Diagonally below
        );

        // Try to swap with cells in these positions if possible
        self.check_swap_from_list(space, i, check, SwappingSim::check_density);
    }

    /// Handles movement for liquids like water and oil
    fn move_liquid(&mut self, space: &mut Space, x: i32, y: i32) {
        let i = space.get_index(x as u32, y as u32);
        // Randomly choose left or right for the horizontal movement
        let d = if rand() > 0.5 { 1 } else { -1 };

        // Check these positions in order: below, diagonal, opposite diagonal, sides
        let check = vec!(
            (x, y + 1),           // Directly below
            (x + d, y + 1),       // Diagonally below
            (x - d, y + 1),       // Other diagonal
            (x + d, y),           // Side
            (x - d, y),           // Other side
        );

        // Try to swap with cells in these positions if possible
        self.check_swap_from_list(space, i, check, SwappingSim::check_density);
    }

    /// Handles movement for gases like propane
    fn move_gas(&mut self, space: &mut Space, x: i32, y: i32) {
        let i = space.get_index(x as u32, y as u32);
        // Get random horizontal and vertical movement
        let dx = random_modifier();
        let dy = random_modifier();

        // Try to move in the random direction
        let check = vec!(
            (x + dx, y + dy),     // Random direction
        );

        // Try to swap with cells in these positions if possible
        self.check_swap_from_list(space, i, check, SwappingSim::check_density);
    }

    /// Creates a new cell at the specified position
    fn spawn_new(&mut self, space: &mut Space, x: i32, y: i32, cell: Cell) {
        // Randomly choose directions
        let dx = if rand() > 0.5 { 1 } else { -1 };
        let dy = if rand() > 0.5 { 1 } else { -1 };

        // Check these diagonal positions
        let list = vec![
            (x + dx, y + dy),
            (x - dx, y + dy),
            (x + dx, y - dy),
            (x - dx, y - dy),
        ];

        // Try each position until we find an empty one
        for (x, y) in list.iter() {
            if let Some(ni) = space.get_index_checked(*x, *y) {
                if space.get_cell_type_at(ni) == CellType::Empty {
                    // Place the new cell here
                    space.set_cell(ni, &cell);
                    break;
                }
            }
        }
    }

    /// Attempts to ignite flammable neighbors around hot cells
    fn ignite_neighbours(&mut self, temp: f32, space: &mut Space, x: i32, y: i32) {
        /*
        // This code is commented out in the original - it would simulate heat transfer
        let i = space.get_index(x as u32, y as u32);

        let coefficient = CellType::get_properties(space.get_cell_type_at(i)).temp_coefficient;
        let mut total_temp = 0.0;
        self.foreach_neighbour(space, x, y, |cell, props| {
            let change = (temp - cell.temp) * 0.1 * (props.temp_coefficient / coefficient);
            cell.temp += change;
            total_temp += change;
        });

        let cell = space.get_cell_at(i);
        cell.temp -= total_temp;
        */

        // Check each neighboring cell
        self.foreach_neighbour(space, x, y, |cell, props| {
            //if props.flammable && cell.temp > 100.0 {
            // If the neighbor is flammable and we pass a random check
            if props.flammable && rand() < 0.50 {
                // Ignite it (turn it into fire)
                cell.init(CellType::Fire);
            }
        });
    }

    /// Checks if a cell can move to another position based on density
    fn check_density(source: &CellTypeProperties, dest: &CellTypeProperties) -> bool {
        source.density > dest.density
    }

    /// Tries to swap the cell at index i with one of the cells in the provided list
    fn check_swap_from_list(&mut self, space: &mut Space, i: usize, list: Vec<(i32, i32)>, can_move: fn(&CellTypeProperties, &CellTypeProperties) -> bool) {
        for (x, y) in list.iter() {
            // Check if this position is within bounds
            if let Some(ni) = space.get_index_checked(*x, *y) {
                // Get properties of both cells
                let i_prop = CellType::get_properties(space.get_cell_type_at(i));
                let ni_prop = CellType::get_properties(space.get_cell_type_at(ni));
                // Check if the swap is allowed
                if can_move(&i_prop, &ni_prop) {
                    // Swap the cells
                    space.swap_cells(i, ni);
                    break;
                }
            } 
        }
    }

    /// Applies a function to each neighbor of a cell
    fn foreach_neighbour<F>(&mut self, space: &mut Space, x: i32, y: i32, mut f: F) 
        where F: FnMut(&mut Cell, &CellTypeProperties) {
        // Check all 8 surrounding cells
        for dx in -1..=1 {
            for dy in -1..=1 {
                // Skip the center cell (which is the cell we're currently processing)
                if dx == x && dy == y {
                    continue;
                }

                // Get the index of this neighbor if it's within bounds
                if let Some(ni) = space.get_index_checked(x + dx, y + dy) {
                    // Get the cell and its properties
                    let dest_cell = space.get_cell_at(ni);
                    let dest_props = CellType::get_properties(dest_cell.cell_type);
                    // Apply the provided function to this neighbor
                    f(dest_cell, dest_props);
                }
            }
        }
    }
}

/// Defines a pattern to match against for cellular automaton rules
enum MatchCell {
    /// Matches exactly the specified cell type
    Exact(CellType),
    /// Matches any cell type
    Any,
}

/// Defines how to modify a cell for cellular automaton rules
enum ModifyCell {
    /// Keep the cell type the same
    Same,
    /// Change the cell type to the specified type
    Type(CellType),
}

impl MatchCell {
    /// Checks if this pattern matches the given cell
    fn match_cell(&self, cell: Cell) -> bool {
        match self {
            MatchCell::Exact(cell_type) => {
                *cell_type == cell.cell_type
            },
            MatchCell::Any => true,
        }
    }
}

impl ModifyCell {
    /// Applies the modification to the given cell
    fn set_cell(&self, mut cell: Cell) -> Cell {
        match self {
            ModifyCell::Type(cell_type) => {
                cell.cell_type = *cell_type
            },
            ModifyCell::Same => { },
        }
        cell
    }
}

/// A 2x2 neighborhood of cells
type NeighbourhoodCells = [Cell; 4];
/// A pattern to match against a 2x2 neighborhood
type NeighbourhoodPattern = [MatchCell; 4];
/// A set of modifications to apply to a 2x2 neighborhood
type NeighbourhoodModifier = [ModifyCell; 4];

/// A rule for cellular automaton simulation
struct CellRule {
    /// Probability (0-1) that this rule will be applied when matched
    probability: f64,
    /// Pattern to match against
    if_nb: NeighbourhoodPattern,
    /// Modifications to apply when matched
    then_nb: NeighbourhoodModifier,
}

impl CellRule {
    /// Creates a new cell rule
    const fn new(probability: f64, if_nb: NeighbourhoodPattern, then_nb: NeighbourhoodModifier) -> CellRule {
        CellRule {
            probability,
            if_nb,
            then_nb,
        }
    }

    /// Checks if this rule's pattern matches the given neighborhood
    fn match_if(&self, nb: NeighbourhoodCells) -> bool {
        self.if_nb.iter()
            .zip(nb.iter())
            .all(|(pattern, cell)| pattern.match_cell(*cell) && rand() < self.probability)
    }

    /// Applies this rule's modifications to the given neighborhood
    fn set_then(&self, nb: NeighbourhoodCells) -> NeighbourhoodCells {
        self.then_nb.iter()
            .zip(nb.iter())
            .map(|(pattern, cell)| pattern.set_cell(*cell))
            .collect::<Vec<Cell>>()
            .try_into()
            .unwrap()
    }
}

const RULES: &[CellRule] = &[
    CellRule::new(1.0, [
        MatchCell::Exact(CellType::Sand), MatchCell::Any,
        MatchCell::Exact(CellType::Empty), MatchCell::Any,
    ],
    [
        ModifyCell::Type(CellType::Empty), ModifyCell::Same,
        ModifyCell::Type(CellType::Sand), ModifyCell::Same,
    ]),
    CellRule::new(1.0, [
        MatchCell::Any, MatchCell::Exact(CellType::Sand),
        MatchCell::Any, MatchCell::Exact(CellType::Empty),
    ],
    [
        ModifyCell::Same, ModifyCell::Type(CellType::Empty),
        ModifyCell::Same, ModifyCell::Type(CellType::Sand),
    ]),
    CellRule::new(1.0, [
        MatchCell::Exact(CellType::Sand), MatchCell::Any,
        MatchCell::Exact(CellType::Sand), MatchCell::Exact(CellType::Empty),
    ],
    [
        ModifyCell::Type(CellType::Empty), ModifyCell::Same,
        ModifyCell::Type(CellType::Sand),  ModifyCell::Type(CellType::Sand),
    ]),
    CellRule::new(1.0, [
        MatchCell::Any,                    MatchCell::Exact(CellType::Sand),
        MatchCell::Exact(CellType::Empty), MatchCell::Exact(CellType::Sand),
    ],
    [
        ModifyCell::Same,                 ModifyCell::Type(CellType::Empty),
        ModifyCell::Type(CellType::Sand), ModifyCell::Type(CellType::Sand),
    ]),
];

/// Alternative simulation implementation using cellular automaton rules
pub struct CellularSim;

impl Simulator for CellularSim {
    fn tick(&mut self, space: &mut Space) {
        self.advance(space);
    }
}

impl CellularSim {
    /// Advances the cellular automaton simulation by one step
    pub fn advance(&mut self, space: &mut Space) {
        space.increment_generation();
        let start = if space.get_generation() % 2 == 0 { 0 } else { 1 };

        for y in (start..(space.get_height() as i32 - start)).step_by(2) {
            for x in (start..(space.get_width() as i32 - start)).step_by(2) {
                // The 2 x 2 grid start at (x, y) needs to be evaluated

                let mut square = self.get_neighbourhood(space, x, y);

                for rule in RULES {
                    if rule.match_if(square) {
                        square = rule.set_then(square);
                        break;
                    }
                }

/*
                // Falling downwards rule (Gravity)
                if square[0][0].1.density > square[1][0].1.density {
                    swap!(square[0][0], square[1][0]);
                }
                if square[0][1].1.density > square[1][1].1.density {
                    swap!(square[0][1], square[1][1]);
                }

                // Falling sideways rule (Sand)
                if square[0][0].1.density > square[1][1].1.density {
                    swap!(square[0][0], square[1][1]);
                }
                if square[0][1].1.density > square[1][0].1.density {
                    swap!(square[0][1], square[1][0]);
                }
                // Moving sideways rule (Water)
                if (square[0][0].0.cell_type == CellType::Water || square[0][0].0.cell_type == CellType::Empty)
                  && (square[0][1].0.cell_type == CellType::Water || square[0][1].0.cell_type == CellType::Empty) {
                    swap!(square[0][0], square[0][1]);
                }
                if (square[1][0].0.cell_type == CellType::Water || square[1][0].0.cell_type == CellType::Empty)
                  && (square[1][1].0.cell_type == CellType::Water || square[1][1].0.cell_type == CellType::Empty) {
                    swap!(square[1][0], square[1][1]);
                }
*/

                self.set_neighbourhood(space, x, y, &square);
            }
        }
    }

    /// Gets the 2x2 neighborhood of cells at the specified position
    fn get_neighbourhood(&self, space: &mut Space, x: i32, y: i32) -> [Cell; 4] {
        let cell1a = *space.get_cell_at(space.get_index(x as u32, y as u32));
        let cell1b = *space.get_cell_at(space.get_index(x as u32 + 1, y as u32));
        let cell2a = *space.get_cell_at(space.get_index(x as u32, y as u32 + 1));
        let cell2b = *space.get_cell_at(space.get_index(x as u32 + 1, y as u32 + 1));
        [ cell1a, cell1b, cell2a, cell2b ]
    }

    /// Sets the 2x2 neighborhood of cells at the specified position
    fn set_neighbourhood(&self, space: &mut Space, x: i32, y: i32, square: &[Cell; 4]) {
        space.set_cell(space.get_index(x as u32, y as u32), &square[0]);
        space.set_cell(space.get_index(x as u32 + 1, y as u32), &square[1]);
        space.set_cell(space.get_index(x as u32, y as u32 + 1), &square[2]);
        space.set_cell(space.get_index(x as u32 + 1, y as u32 + 1), &square[3]);
    }
}

/// Generates a random value for adding variation to simulation
fn random_modifier() -> i32 {
    match rand() {
        x if x < 0.33 => 1,
        x if x < 0.66 => -1,
        _ => 0,
    }
}

