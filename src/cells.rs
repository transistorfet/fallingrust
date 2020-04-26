
use crate::rand;


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CellTypeProperties {
    pub cell_type: CellType,
    pub density: f64,
}


#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CellType {
    Empty,
    Rock,
    Sand,
    Water,
    Oil,
    Propane,
}


static CELL_TYPES: [CellType; 5] = [
    CellType::Rock,
    CellType::Sand,
    CellType::Water,
    CellType::Oil,
    CellType::Propane,
];

static CELL_PROPERTIES: [CellTypeProperties; 6] = [
    CellTypeProperties {
        cell_type: CellType::Empty,
        density: 0.0,
    },
    CellTypeProperties {
        cell_type: CellType::Rock,
        density: 3.0,
    },
    CellTypeProperties {
        cell_type: CellType::Sand,
        density: 3.0,
    },
    CellTypeProperties {
        cell_type: CellType::Water,
        density: 1.0,
    },
    CellTypeProperties {
        cell_type: CellType::Oil,
        density: 0.8,
    },
    CellTypeProperties {
        cell_type: CellType::Propane,
        density: 0.1,
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
    pub temp: u16,
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

    pub fn random() -> Cell {
        Cell {
            cell_type: CellType::random(),
            temp: 20,
            generation: 0,
        }
    }
}

