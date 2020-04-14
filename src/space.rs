
use crate::cells::{ CellType, Cell };

use crate::{ log, rand };


pub struct Space {
    width: u32,
    height: u32,
    run: bool,
    generation: u32,
    cells: Vec<Cell>,
}

impl Space {
    pub fn new(width: u32, height: u32) -> Self {
        let length = width * height;
        let mut cells = Vec::with_capacity(length as usize);

        for _ in 0..length {
            cells.push(Cell::random());
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
        let i = self.get_index(x as u32, y as u32);
        self.cells[i].cell_type = cell_type;
    }

    pub fn tick(&mut self) {
        self.generation  = self.generation + 1;

        for y in (0..(self.height as i32)).rev() {
            for x in (0..(self.width as i32)).rev() {
                let i = self.get_index(x as u32, y as u32);
                let cell = self.get_cell_at(i);

                // Only simulate cells that haven't already been moved
                if self.generation != cell.generation {
                    self.simulate_cell(x, y, cell.cell_type);
                }
            }
        }
    }

    pub fn simulate_cell(&mut self, x: i32, y: i32, cell_type: CellType) {
        let i = self.get_index(x as u32, y as u32);

        match cell_type {
            CellType::Empty => { },

            CellType::Rock => {
                //if let Some(ni) = self.checked_index_is_empty(x, y + 1) {
                //    self.swap_cells(i, ni);
                //}
            },

            CellType::Sand => {
                let d = if rand() > 0.5 { 1 } else { -1 };

                let check = vec!(
                    (x, y + 1),
                    (x + d, y + 1),
                    (x - d, y + 1),
                    //(x + d, y),
                    //(x - d, y),
                );

                self.check_swap_from_list(i, check);
            },

            CellType::Water => {
                let d = if rand() > 0.5 { 1 } else { -1 };

                let check = vec!(
                    (x, y + 1),
                    (x + d, y + 1),
                    (x - d, y + 1),
                    (x + d, y),
                    (x - d, y),
                );

                self.check_swap_from_list(i, check);
            },

        }
    }

    fn check_swap_from_list(&mut self, i: usize, check: Vec<(i32, i32)>) {
        for (x, y) in check.iter() {
            //if let Some(ni) = self.checked_index_is_empty(*x, *y) {
            if let Some(ni) = self.checked_index(*x, *y) {
                let i_prop = CellType::get_properties(self.get_cell_type_at(i));
                let ni_prop = CellType::get_properties(self.get_cell_type_at(ni));
                if i_prop.density > ni_prop.density {
                    self.swap_cells(i, ni);
                    break;
                }
            } 
        }
    }


    fn checked_index(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
            None
        } else {
            Some(self.get_index(x as u32, y as u32))
        }
    }

    fn checked_index_is_empty(&self, x: i32, y: i32) -> Option<usize> {
        match self.checked_index(x, y) {
            Some(i) => {
                if self.get_cell_type_at(i) == CellType::Empty {
                    Some(i)
                } else {
                    None
                }
            }
            None => None
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
        self.cells[i] = Cell { cell_type, generation: self.generation };
    }

    fn swap_cells(&mut self, i: usize, j: usize) {
        let i_cell_type = self.cells[i].cell_type;
        self.cells[i] = Cell { cell_type: self.cells[j].cell_type, generation: self.generation };
        self.cells[j] = Cell { cell_type: i_cell_type, generation: self.generation };
    }
}

