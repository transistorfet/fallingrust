
#[allow(unused_imports)]
use crate::{ log, rand };
use crate::space::Space;
use crate::cells::{ Cell, CellType, CellTypeProperties };


pub trait Simulator {
    fn tick(&mut self, space: &mut Space);
}


pub struct SwappingSim;

impl Simulator for SwappingSim {
    fn tick(&mut self, space: &mut Space) {
        self.advance(space);
    }
}

impl SwappingSim {
    pub fn advance(&mut self, space: &mut Space) {
        space.increment_generation();

        for y in (0..(space.get_height() as i32)).rev() {
            for x in 0..(space.get_width() as i32) {
                // Sweeping left to right to left removes the bias in the x direction, but not the y direction
                // I'm not yet sure why the bias exists in the first place, but I suspect it has to do with the generation/update tracking
                let x = if y % 2 == 0 { x } else { space.get_width() as i32 - x - 1 };

                let i = space.get_index(x as u32, y as u32);

                // Only simulate cells that haven't already been moved
                if space.cell_needs_updating(i) {
                    self.simulate_cell(space, x, y);
                    space.update_cell_generation(i);
                }
            }
        }
    }

    pub fn simulate_cell(&mut self, space: &mut Space, x: i32, y: i32) {
        let i = space.get_index(x as u32, y as u32);
        let cell = space.get_cell_at(i);

        match cell.cell_type {
            CellType::Empty => { },

            // Fixed Solids
            CellType::Rock => {

            },

            // Powdered Solids
            CellType::Sand => {
                self.move_granular(space, x, y);
            },

            // Liquids
            CellType::Oil |
            CellType::Water => {
                self.move_liquid(space, x, y);
            },

            // Gases
            CellType::Propane => {
                self.move_gas(space, x, y);
            },

            CellType::Fire => {
                cell.temp -= (rand() * 5.0) as i16;
                if cell.temp < 10 {
                    cell.cell_type = CellType::Empty;
                    cell.temp = 0;
                }

                if cell.temp > 300 && rand() < 0.4 {
                    cell.temp = cell.temp * 3 / 5;
                    let cell = *cell;
                    self.spawn_new(space, x, y, cell);
                }

                self.ignite_neighbours(space, x, y);

                if rand() < 0.25 {
                    self.move_gas(space, x, y);
                }
            },
        }
    }

    fn move_granular(&mut self, space: &mut Space, x: i32, y: i32) {
        let i = space.get_index(x as u32, y as u32);
        let d = if rand() > 0.5 { 1 } else { -1 };

        let check = vec!(
            (x, y + 1),
            (x + d, y + 1),
        );

        self.check_swap_from_list(space, i, check, SwappingSim::check_density);
    }

    fn move_liquid(&mut self, space: &mut Space, x: i32, y: i32) {
        let i = space.get_index(x as u32, y as u32);
        let d = if rand() > 0.5 { 1 } else { -1 };

        let check = vec!(
            (x, y + 1),
            (x + d, y + 1),
            (x - d, y + 1),
            (x + d, y),
            (x - d, y),
        );

        self.check_swap_from_list(space, i, check, SwappingSim::check_density);
    }

    fn move_gas(&mut self, space: &mut Space, x: i32, y: i32) {
        let i = space.get_index(x as u32, y as u32);
        let dx = random_modifier();
        let dy = random_modifier();

        let check = vec!(
            (x + dx, y + dy),
        );

        self.check_swap_from_list(space, i, check, SwappingSim::check_density);
    }

    fn spawn_new(&mut self, space: &mut Space, x: i32, y: i32, cell: Cell) {
        let dx = if rand() > 0.5 { 1 } else { -1 };
        let dy = if rand() > 0.5 { 1 } else { -1 };

        let list = vec![
            (x + dx, y + dy),
            (x - dx, y + dy),
            (x + dx, y - dy),
            (x - dx, y - dy),
        ];

        for (x, y) in list.iter() {
            if let Some(ni) = space.get_index_checked(*x, *y) {
                if space.get_cell_type_at(ni) == CellType::Empty {
                    space.set_cell(ni, &cell);
                    break;
                }
            }
        }
    }

    fn ignite_neighbours(&mut self, space: &mut Space, x: i32, y: i32) {
        let mut cells_lit = 0;
        for dx in -1..=1 {
            for dy in -1..=1 {
                if let Some(ni) = space.get_index_checked(x + dx, y + dy) {
                    let dest_cell = space.get_cell_at(ni);
                    let dest_props = CellType::get_properties(dest_cell.cell_type);
                    if dest_props.flammable {
                        dest_cell.init(CellType::Fire);
                        cells_lit += 1;
                        if cells_lit >= 1 {
                            break;
                        }
                    }
                }
            }
        }
    }


    fn check_density(source: &CellTypeProperties, dest: &CellTypeProperties) -> bool {
        source.density > dest.density
    }

    fn check_swap_from_list(&mut self, space: &mut Space, i: usize, list: Vec<(i32, i32)>, can_move: fn(&CellTypeProperties, &CellTypeProperties) -> bool) {
        for (x, y) in list.iter() {
            if let Some(ni) = space.get_index_checked(*x, *y) {
                let i_prop = CellType::get_properties(space.get_cell_type_at(i));
                let ni_prop = CellType::get_properties(space.get_cell_type_at(ni));
                if can_move(&i_prop, &ni_prop) {
                    space.swap_cells(i, ni);
                    break;
                }
            } 
        }
    }
}

macro_rules! swap {
    ( $x:expr, $y:expr ) => {
        let tmp = $x;
        $x = $y;
        $y = tmp;
    }
}

pub struct CellularSim;

impl Simulator for CellularSim {
    fn tick(&mut self, space: &mut Space) {
        self.advance(space);
    }
}

impl CellularSim {
    pub fn advance(&mut self, space: &mut Space) {
        space.increment_generation();
        let start = if space.get_generation() % 2 == 0 { 0 } else { 1 };

        for y in (start..(space.get_height() as i32 - start)).step_by(2) {
            for x in (start..(space.get_width() as i32 - start)).step_by(2) {
                // The 2 x 2 grid start at (x, y) needs to be evaluated

                let mut square = self.get_neighbourhood(space, x, y);

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

                self.set_neighbourhood(space, x, y, &square);
            }
        }
    }



    fn get_neighbourhood(&self, space: &mut Space, x: i32, y: i32) -> [[(Cell, &CellTypeProperties); 2]; 2] {
        let cell1a = *space.get_cell_at(space.get_index(x as u32, y as u32));
        let cell1b = *space.get_cell_at(space.get_index(x as u32 + 1, y as u32));
        let cell2a = *space.get_cell_at(space.get_index(x as u32, y as u32 + 1));
        let cell2b = *space.get_cell_at(space.get_index(x as u32 + 1, y as u32 + 1));
        [
            [ (cell1a, cell1a.get_properties()), (cell1b, cell1b.get_properties()) ],
            [ (cell2a, cell2a.get_properties()), (cell2b, cell2b.get_properties()) ]
        ]
    }

    fn set_neighbourhood(&self, space: &mut Space, x: i32, y: i32, square: &[[(Cell, &CellTypeProperties); 2]; 2]) {
        space.set_cell(space.get_index(x as u32, y as u32), &square[0][0].0);
        space.set_cell(space.get_index(x as u32 + 1, y as u32), &square[0][1].0);
        space.set_cell(space.get_index(x as u32, y as u32 + 1), &square[1][0].0);
        space.set_cell(space.get_index(x as u32 + 1, y as u32 + 1), &square[1][1].0);
    }

}

fn random_modifier() -> i32 {
    match rand() {
        x if x < 0.33 => 1,
        x if x < 0.66 => -1,
        _ => 0,
    }
}
