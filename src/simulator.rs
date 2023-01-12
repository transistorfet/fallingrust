
use std::convert::TryInto;

#[allow(unused_imports)]
use crate::{ log, rand, debug_print };
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
            CellType::Wood |
            CellType::Rock => {

            },

            // Powdered Solids
            CellType::Sand |
            CellType::Gunpowder => {
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

            CellType::Acid => {
                let mut expend = false;
                self.foreach_neighbour(space, x, y, |cell, props| {
                    if props.dissolvable && rand() < 0.005 {
                        cell.init(CellType::Empty);
                        expend = true;
                    }
                });

                if expend {
                    let cell = space.get_cell_at(i);
                    cell.init(CellType::Empty);
                } else {
                    self.move_liquid(space, x, y);
                }
            },

            CellType::Lava => {
                cell.temp -= rand() as f32 * 5.0;
                if cell.temp < 10.0 {
                    cell.cell_type = CellType::Rock;
                }

                self.ignite_neighbours(cell.temp, space, x, y);
                self.move_liquid(space, x, y);
            },

            CellType::Fire => {
                cell.temp -= rand() as f32 * 5.0;
                if cell.temp < 10.0 {
                    cell.cell_type = CellType::Empty;
                    cell.temp = 0.0;
                }

                if cell.temp > 300.0 && rand() < 0.4 {
                    cell.temp = cell.temp * 3.0 / 5.0;
                    let cell = *cell;
                    self.spawn_new(space, x, y, cell);
                }

                let cell = space.get_cell_at(i);
                self.ignite_neighbours(cell.temp, space, x, y);

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

    fn ignite_neighbours(&mut self, temp: f32, space: &mut Space, x: i32, y: i32) {
        /*
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

        self.foreach_neighbour(space, x, y, |cell, props| {
            //if props.flammable && cell.temp > 100.0 {
            if props.flammable && rand() < 0.50 {
                cell.init(CellType::Fire);
            }
        });
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

    fn foreach_neighbour<F>(&mut self, space: &mut Space, x: i32, y: i32, mut f: F) where F: FnMut(&mut Cell, &CellTypeProperties) {
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == x && dy == y {
                    continue;
                }

                if let Some(ni) = space.get_index_checked(x + dx, y + dy) {
                    let dest_cell = space.get_cell_at(ni);
                    let dest_props = CellType::get_properties(dest_cell.cell_type);
                    f(dest_cell, dest_props);
                }
            }
        }
    }
}


enum MatchCell {
    Exact(CellType),
    //EmptyOr(CellType),
    Any,
}

enum ModifyCell {
    Same,
    Type(CellType),
}

impl MatchCell {
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

type NeighbourhoodCells = [Cell; 4];
type NeighbourhoodPattern = [MatchCell; 4];
type NeighbourhoodModifier = [ModifyCell; 4];

struct CellRule {
    probability: f64,
    if_nb: NeighbourhoodPattern,
    then_nb: NeighbourhoodModifier,
}

impl CellRule {
    const fn new(probability: f64, if_nb: NeighbourhoodPattern, then_nb: NeighbourhoodModifier) -> CellRule {
        CellRule {
            probability,
            if_nb,
            then_nb,
        }
    }

    fn match_if(&self, nb: NeighbourhoodCells) -> bool {
        self.if_nb.iter()
            .zip(nb.iter())
            .all(|(pattern, cell)| pattern.match_cell(*cell) && rand() < self.probability)
    }

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



    fn get_neighbourhood(&self, space: &mut Space, x: i32, y: i32) -> [Cell; 4] {
        let cell1a = *space.get_cell_at(space.get_index(x as u32, y as u32));
        let cell1b = *space.get_cell_at(space.get_index(x as u32 + 1, y as u32));
        let cell2a = *space.get_cell_at(space.get_index(x as u32, y as u32 + 1));
        let cell2b = *space.get_cell_at(space.get_index(x as u32 + 1, y as u32 + 1));
        [ cell1a, cell1b, cell2a, cell2b ]
    }

    fn set_neighbourhood(&self, space: &mut Space, x: i32, y: i32, square: &[Cell; 4]) {
        space.set_cell(space.get_index(x as u32, y as u32), &square[0]);
        space.set_cell(space.get_index(x as u32 + 1, y as u32), &square[1]);
        space.set_cell(space.get_index(x as u32, y as u32 + 1), &square[2]);
        space.set_cell(space.get_index(x as u32 + 1, y as u32 + 1), &square[3]);
    }

}

fn random_modifier() -> i32 {
    match rand() {
        x if x < 0.33 => 1,
        x if x < 0.66 => -1,
        _ => 0,
    }
}

