
use crate::rand;


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CellTypeProperties {
    pub name: &'static str,
    pub cell_type: CellType,
    pub density: f64,
    pub temp_coefficient: f32,
    pub flammable: bool,
    pub dissolvable: bool,
}


#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CellType {
    Empty,
    Rock,
    Wood,
    Sand,
    Gunpowder,
    Water,
    Oil,
    Propane,
    Fire,
    Lava,
    Acid,
}


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

static CELL_PROPERTIES: [CellTypeProperties; 11] = [
    CellTypeProperties {
        name: "Empty",
        cell_type: CellType::Empty,
        density: 0.0,
        temp_coefficient: 0.1,
        flammable: false,
        dissolvable: false,
    },
    CellTypeProperties {
        name: "Rock",
        cell_type: CellType::Rock,
        density: 3.0,
        temp_coefficient: 0.1,
        flammable: false,
        dissolvable: true,
    },
    CellTypeProperties {
        name: "Wood",
        cell_type: CellType::Wood,
        density: 3.0,
        temp_coefficient: 0.8,
        flammable: true,
        dissolvable: true,
    },
    CellTypeProperties {
        name: "Sand",
        cell_type: CellType::Sand,
        density: 3.0,
        temp_coefficient: 0.1,
        flammable: false,
        dissolvable: true,
    },
    CellTypeProperties {
        name: "Gunpowder",
        cell_type: CellType::Gunpowder,
        density: 3.0,
        temp_coefficient: 20.0,
        flammable: true,
        dissolvable: false,
    },
    CellTypeProperties {
        name: "Water",
        cell_type: CellType::Water,
        density: 1.0,
        temp_coefficient: 0.4,
        flammable: false,
        dissolvable: false,
    },
    CellTypeProperties {
        name: "Oil",
        cell_type: CellType::Oil,
        density: 0.8,
        temp_coefficient: 10.0,
        flammable: true,
        dissolvable: false,
    },
    CellTypeProperties {
        name: "Propane",
        cell_type: CellType::Propane,
        density: 0.1,
        temp_coefficient: 200.0,
        flammable: true,
        dissolvable: false,
    },
    CellTypeProperties {
        name: "Fire",
        cell_type: CellType::Fire,
        density: 0.01,
        temp_coefficient: 1.0,
        flammable: false,
        dissolvable: false,
    },
    CellTypeProperties {
        name: "Lava",
        cell_type: CellType::Lava,
        density: 3.0,
        temp_coefficient: 100.0,
        flammable: false,
        dissolvable: true,
    },
    CellTypeProperties {
        name: "Acid",
        cell_type: CellType::Acid,
        density: 1.2,
        temp_coefficient: 0.1,
        flammable: false,
        dissolvable: false,
    },
];

impl CellType {
    pub fn random() -> CellType {
        match (rand() * 20.0) as u32 {
            0 => CellType::Rock,
            1 => CellType::Sand,
            2 => CellType::Water,
            _ => CellType::Empty,
        }
    }

    pub fn iter<'a>() -> std::slice::Iter<'a, CellType> {
        CELL_TYPES.iter()
    }

    pub fn get_properties<'a>(cell_type: CellType) -> &'a CellTypeProperties {
        &CELL_PROPERTIES[cell_type as usize]
    }
}



#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Cell {
    pub cell_type: CellType,
    pub generation: u8,
    pub temp: f32,
}

impl Cell {
    pub fn empty() -> Cell {
        Cell {
            cell_type: CellType::Empty,
            generation: 0,
            temp: 20.0,
        }
    }

    #[allow(dead_code)]
    pub fn random() -> Cell {
        Cell {
            cell_type: CellType::random(),
            temp: 20.0,
            generation: 0,
        }
    }

    pub fn init(&mut self, cell_type: CellType) {
        self.cell_type = cell_type;

        match cell_type {
            CellType::Lava => self.temp = 1000.0 + rand() as f32 * 1000.0,
            CellType::Fire => self.temp = rand() as f32 * 1000.0,
            _ => self.temp = 20.0,
        }
    }

    pub fn get_properties<'a>(&self) -> &'a CellTypeProperties {
        CellType::get_properties(self.cell_type)
    }
}

