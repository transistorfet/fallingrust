
use crate::cells::{ Cell, CellType, CellTypeProperties };

use crate::{ log, rand };


pub struct Space {
    width: u32,
    height: u32,
    run: bool,
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
            run: true,
            generation: 0,
            cells: cells,
        }
    }

    pub fn get_cell_type(&self, x: u32, y: u32) -> CellType {
        let i = self.get_index(x, y);
        self.cells[i].cell_type
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn toggle_run(&mut self) {
        self.run = !self.run;
    }

    pub fn is_running(&self) -> bool {
        self.run
    }

    pub fn add(&mut self, x: i32, y: i32, cell_type: CellType) {
        if let Some(i) = self.checked_index(x, y) {
            if self.cells[i].cell_type == CellType::Empty || cell_type == CellType::Empty {
                self.cells[i].cell_type = cell_type;
            }
        }
    }

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
                let d = if rand() > 0.5 { 1 } else { -1 };

                let check = vec!(
                    (x, y + 1),
                    (x + d, y + 1),
                );

                self.check_swap_from_list(i, check, Space::check_density);
            },

            // Liquids
            CellType::Oil |
            CellType::Water => {
                let d = if rand() > 0.5 { 1 } else { -1 };

                let check = vec!(
                    (x, y + 1),
                    (x + d, y + 1),
                    (x - d, y + 1),
                    (x + d, y),
                    (x - d, y),
                );

                self.check_swap_from_list(i, check, Space::check_density);
            },

            // Gases
            CellType::Propane => {
                let dx = random_modifier();
                let dy = random_modifier();

                let check = vec!(
                    (x + dx, y + dy),
                );

                self.check_swap_from_list(i, check, Space::check_empty);
            },
        }
    }

    fn check_density(source: &CellTypeProperties, dest: &CellTypeProperties) -> bool {
        source.density > dest.density
    }

    fn check_empty(source: &CellTypeProperties, dest: &CellTypeProperties) -> bool {
        dest.cell_type == CellType::Empty
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
        self.cells[j].generation = self.generation;
    }
}


fn random_modifier() -> i32 {
    match rand() {
        x if x < 0.33 => 1,
        x if x < 0.66 => -1,
        _ => 0,
    }
}

