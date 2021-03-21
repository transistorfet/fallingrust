
use crate::cells::{ Cell, CellType, CellTypeProperties };

use crate::{ log, rand };

macro_rules! swap {
    ( $x:expr, $y:expr ) => {
        let tmp = $x;
        $x = $y;
        $y = tmp;
    }
}


pub struct Space {
    width: u32,
    height: u32,
    generation: u8,
    cells: Vec<Cell>,
}

impl Space {
    pub fn new(width: u32, height: u32) -> Self {
        let length = width * height;
        let mut cells = Vec::with_capacity(length as usize);

        for _ in 0..length {
            cells.push(Cell::empty());
        }

        Space {
            width: width,
            height: height,
            generation: 0,
            cells: cells,
        }
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn get_generation(&self) -> u8 {
        self.generation
    }

    pub fn get_cell_type(&self, x: u32, y: u32) -> CellType {
        let i = self.get_index(x, y);
        self.cells[i].cell_type
    }

    pub fn add(&mut self, x: i32, y: i32, cell_type: CellType) {
        if let Some(i) = self.checked_index(x, y) {
            if self.cells[i].cell_type == CellType::Empty || cell_type == CellType::Empty {
                self.init_cell(i, cell_type);
            }
        }
    }

    pub fn init_cell(&mut self, i: usize, cell_type: CellType) {
        self.cells[i].cell_type = cell_type;

        match cell_type {
            CellType::Fire => self.cells[i].temp = (rand() * 1000.0) as i16,
            _ => {},
        }
    }

/*
    fn set_cell(&mut self, i: usize, cell: &Cell) {
        self.cells[i] = *cell;
        self.cells[i].generation = self.generation;
    }

    fn get_square(&self, x: i32, y: i32) -> [[Cell; 2]; 2] {
        let cell1a = self.get_cell_at(self.get_index(x as u32, y as u32));
        let cell1b = self.get_cell_at(self.get_index(x as u32 + 1, y as u32));
        let cell2a = self.get_cell_at(self.get_index(x as u32, y as u32 + 1));
        let cell2b = self.get_cell_at(self.get_index(x as u32 + 1, y as u32 + 1));
        [[ cell1a, cell1b ], [ cell2a, cell2b ]]
    }

    fn get_props_square(&self, square: &[[Cell; 2]; 2]) -> [[&'static CellTypeProperties; 2]; 2] {
        [
            [ CellType::get_properties(square[0][0].cell_type), CellType::get_properties(square[0][1].cell_type) ],
            [ CellType::get_properties(square[1][0].cell_type), CellType::get_properties(square[1][1].cell_type) ]
        ]
    }

    fn set_square(&mut self, x: i32, y: i32, square: &[[Cell; 2]; 2]) {
        self.set_cell(self.get_index(x as u32, y as u32), &square[0][0]);
        self.set_cell(self.get_index(x as u32 + 1, y as u32), &square[0][1]);
        self.set_cell(self.get_index(x as u32, y as u32 + 1), &square[1][0]);
        self.set_cell(self.get_index(x as u32 + 1, y as u32 + 1), &square[1][1]);
    }

    pub fn tick(&mut self) {
        self.generation  = self.generation + 1;
        let start = if self.generation % 2 == 0 { 0 } else { 1 };

        for y in (start..(self.height as i32 - start)).step_by(2) {
            for x in (start..(self.width as i32 - start)).step_by(2) {
                // The 2 x 2 grid start at (x, y) needs to be evaluated

                let mut square = self.get_square(x, y);

                // Falling downwards rule (Gravity)
                let props = self.get_props_square(&square);
                if props[0][0].density > props[1][0].density {
                    swap!(square[0][0], square[1][0]);
                }
                if props[0][1].density > props[1][1].density {
                    swap!(square[0][1], square[1][1]);
                }

                // Falling sideways rule (Sand)
                let props = self.get_props_square(&square);
                if props[0][0].density > props[1][1].density {
                    swap!(square[0][0], square[1][1]);
                }
                if props[0][1].density > props[1][0].density {
                    swap!(square[0][1], square[1][0]);
                }
                // Moving sideways rule (Water)
                let props = self.get_props_square(&square);
                if (square[0][0].cell_type == CellType::Water || square[0][0].cell_type == CellType::Empty)
                  && (square[0][1].cell_type == CellType::Water || square[0][1].cell_type == CellType::Empty) {
                    swap!(square[0][0], square[0][1]);
                }
                if (square[1][0].cell_type == CellType::Water || square[1][0].cell_type == CellType::Empty)
                  && (square[1][1].cell_type == CellType::Water || square[1][1].cell_type == CellType::Empty) {
                    swap!(square[1][0], square[1][1]);
                }

                self.set_square(x, y, &square);
            }
        }
    }
*/

    pub fn tick(&mut self) {
        self.generation  = self.generation + 1;

        for y in (0..(self.height as i32)).rev() {
            for x in 0..(self.width as i32) {
                // Sweeping left to right to left removes the bias in the x direction, but not the y direction
                // I'm not yet sure why the bias exists in the first place, but I suspect it has to do with the generation/update tracking
                //let x = if y % 2 == 0 { x } else { self.width as i32 - x - 1 };

                let i = self.get_index(x as u32, y as u32);
                let cell = self.get_cell_at(i);

                // Only simulate cells that haven't already been moved
                if self.generation != cell.generation {
                    self.simulate_cell(x, y, cell);
                    self.cells[i].generation = self.generation;
                }
            }
        }
    }

    pub fn simulate_cell(&mut self, x: i32, y: i32, cell: Cell) {
        let i = self.get_index(x as u32, y as u32);

        match cell.cell_type {
            CellType::Empty => { },

            // Fixed Solids
            CellType::Rock => {

            },

            // Powdered Solids
            CellType::Sand => {
                self.move_granular(x, y);
            },

            // Liquids
            CellType::Oil |
            CellType::Water => {
                self.move_liquid(x, y);
            },

            // Gases
            CellType::Propane => {
                self.move_gas(x, y);
            },

            CellType::Fire => {
                self.cells[i].temp -= (rand() * 5.0) as i16;
                if self.cells[i].temp < 10 {
                    self.cells[i].cell_type = CellType::Empty;
                    self.cells[i].temp = 0;
                }

                let mut cells_lit = 0;
                for dx in -1..2 {
                    for dy in -1..2 {
                        if let Some(ni) = self.checked_index(x + dx, y + dy) {
                            let dest = CellType::get_properties(self.get_cell_type_at(ni));
                            if dest.flammable {
                                self.init_cell(ni, CellType::Fire);
                                cells_lit += 1;
                                if cells_lit >= 1 {
                                    break;
                                }
                            }
                        }
                    }
                }

                if rand() < 0.50 {
                    self.move_gas(x, y);
                }
            },
        }
    }

    fn move_granular(&mut self, x: i32, y: i32) {
        let i = self.get_index(x as u32, y as u32);
        let d = if rand() > 0.5 { 1 } else { -1 };

        let check = vec!(
            (x, y + 1),
            (x + d, y + 1),
        );

        self.check_swap_from_list(i, check, Space::check_density);
    }

    fn move_liquid(&mut self, x: i32, y: i32) {
        let i = self.get_index(x as u32, y as u32);
        let d = if rand() > 0.5 { 1 } else { -1 };

        let check = vec!(
            (x, y + 1),
            (x + d, y + 1),
            (x - d, y + 1),
            (x + d, y),
            (x - d, y),
        );

        self.check_swap_from_list(i, check, Space::check_density);
    }

    fn move_gas(&mut self, x: i32, y: i32) {
        let i = self.get_index(x as u32, y as u32);
        let dx = random_modifier();
        let dy = random_modifier();

        let check = vec!(
            (x + dx, y + dy),
        );

        self.check_swap_from_list(i, check, Space::check_density);
    }


    fn check_density(source: &CellTypeProperties, dest: &CellTypeProperties) -> bool {
        source.density > dest.density
    }

    fn check_swap_from_list(&mut self, i: usize, list: Vec<(i32, i32)>, can_move: fn(&CellTypeProperties, &CellTypeProperties) -> bool) {
        for (x, y) in list.iter() {
            if let Some(ni) = self.checked_index(*x, *y) {
                let i_prop = CellType::get_properties(self.get_cell_type_at(i));
                let ni_prop = CellType::get_properties(self.get_cell_type_at(ni));
                if can_move(&i_prop, &ni_prop) {
                    self.swap_cells(i, ni);
                    break;
                }
            } 
        }

        // Update the generation for the current cell if it wont move
        self.cells[i].generation = self.generation;
    }


    fn checked_index(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
            None
        } else {
            Some(self.get_index(x as u32, y as u32))
        }
    }

    fn get_index(&self, x: u32, y: u32) -> usize {
        (x + (y * self.width)) as usize
    }

    fn get_cell_at(&self, i: usize) -> Cell {
        self.cells[i]
    }

    fn get_cell_type_at(&self, i: usize) -> CellType {
        self.cells[i].cell_type
    }

    fn update_cell(&mut self, i: usize, cell_type: CellType) {
        self.cells[i].cell_type = cell_type;
        self.cells[i].generation = self.generation;
    }

    fn swap_cells(&mut self, i: usize, j: usize) {
        let i_cell = self.cells[i];
        self.cells[i] = self.cells[j];
        self.cells[j] = i_cell;

        self.cells[i].generation = self.generation;
        //self.cells[j].generation = self.generation;
    }
}


fn random_modifier() -> i32 {
    match rand() {
        x if x < 0.33 => 1,
        x if x < 0.66 => -1,
        _ => 0,
    }
}

