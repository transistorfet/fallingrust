
use crate::rand;


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CellTypeProperties {
    pub name: &'static str,
    pub cell_type: CellType,
    pub density: f64,
    pub flammable: bool,
}


#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CellType {
    Empty,
    Rock,
    Sand,
    Water,
    Oil,
    Propane,
    Fire,
}


static CELL_TYPES: [CellType; 6] = [
    CellType::Rock,
    CellType::Sand,
    CellType::Water,
    CellType::Oil,
    CellType::Propane,
    CellType::Fire,
];

static CELL_PROPERTIES: [CellTypeProperties; 7] = [
    CellTypeProperties {
        name: "Erase",
        cell_type: CellType::Empty,
        density: 0.0,
        flammable: false,
    },
    CellTypeProperties {
        name: "Rock",
        cell_type: CellType::Rock,
        density: 3.0,
        flammable: false,
    },
    CellTypeProperties {
        name: "Sand",
        cell_type: CellType::Sand,
        density: 3.0,
        flammable: false,
    },
    CellTypeProperties {
        name: "Water",
        cell_type: CellType::Water,
        density: 1.0,
        flammable: false,
    },
    CellTypeProperties {
        name: "Oil",
        cell_type: CellType::Oil,
        density: 0.8,
        flammable: true,
    },
    CellTypeProperties {
        name: "Propane",
        cell_type: CellType::Propane,
        density: 0.1,
        flammable: true,
    },
    CellTypeProperties {
        name: "Fire",
        cell_type: CellType::Fire,
        density: 0.0,
        flammable: false,
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
    pub temp: i16,
    pub generation: u8,
}

impl Cell {
    pub fn empty() -> Cell {
        Cell {
            cell_type: CellType::Empty,
            temp: 20,
            generation: 0,
        }
    }

    #[allow(dead_code)]
    pub fn random() -> Cell {
        Cell {
            cell_type: CellType::random(),
            temp: 20,
            generation: 0,
        }
    }

    pub fn init(&mut self, cell_type: CellType) {
        self.cell_type = cell_type;

        match cell_type {
            CellType::Fire => self.temp = (rand() * 1000.0) as i16,
            _ => {},
        }
    }

    pub fn get_properties<'a>(&self) -> &'a CellTypeProperties {
        CellType::get_properties(self.cell_type)
    }
}

