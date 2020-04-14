
use crate::cells::CellType;


pub struct InputTracker {
    down: bool,
    x: i32,
    y: i32,
    selected_type: CellType,
}

impl InputTracker {
    pub fn new() -> InputTracker {
        InputTracker {
            down: false,
            x: 0,
            y: 0,
            selected_type: CellType::Sand,
        }
    }

    pub fn update_down(&mut self, down: bool) {
        self.down = down;
    }

    pub fn update_pos(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    pub fn update_selected_type(&mut self, cell_type: CellType) {
        self.selected_type = cell_type;
    }

    pub fn is_down(&self) -> bool {
        self.down
    }

    pub fn get_pos(&self) -> Option<(i32, i32)> {
        match self.down {
            true => Some((self.x, self.y)),
            false => None
        }
    }

    pub fn get_selected_type(&self) -> CellType {
        self.selected_type
    }
}

