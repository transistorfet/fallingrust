
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
}


static CELL_TYPES: [CellType; 4] = [
    CellType::Empty,
    CellType::Rock,
    CellType::Sand,
    CellType::Water
];

static CELL_PROPERTIES: [CellTypeProperties; 4] = [
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
    pub generation: u32,
}

impl Cell {
    pub fn random() -> Cell {
        Cell {
            cell_type: CellType::random(),
            generation: 0,
        }
    }
}

