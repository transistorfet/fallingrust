
use crate::cells::{ Cell, CellType };

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

    pub fn increment_generation(&mut self) {
        self.generation += 1;
    }

    pub fn update_cell_generation(&mut self, i: usize) {
        self.cells[i].generation = self.generation;
    }

    pub fn cell_needs_updating(&self, i: usize) -> bool {
        self.generation != self.cells[i].generation
    }


    pub fn get_index(&self, x: u32, y: u32) -> usize {
        (x + (y * self.width)) as usize
    }

    pub fn get_index_checked(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
            None
        } else {
            Some(self.get_index(x as u32, y as u32))
        }
    }

    pub fn add(&mut self, x: i32, y: i32, cell_type: CellType) {
        if let Some(i) = self.get_index_checked(x, y) {
            if self.cells[i].cell_type == CellType::Empty || cell_type == CellType::Empty {
                self.cells[i].init(cell_type);
            }
        }
    }

    pub fn get_cell_type(&self, x: u32, y: u32) -> CellType {
        let i = self.get_index(x, y);
        self.cells[i].cell_type
    }



    pub fn get_cell_at<'a>(&'a mut self, i: usize) -> &'a mut Cell {
        &mut self.cells[i]
    }

    pub fn get_cell_type_at(&self, i: usize) -> CellType {
        self.cells[i].cell_type
    }

    pub fn swap_cells(&mut self, i: usize, j: usize) {
        let i_cell = self.cells[i];
        self.cells[i] = self.cells[j];
        self.cells[j] = i_cell;

        self.cells[i].generation = self.generation;
    }

    pub fn set_cell(&mut self, i: usize, cell: &Cell) {
        self.cells[i] = *cell;

        self.cells[i].generation = self.generation;
    }
}

