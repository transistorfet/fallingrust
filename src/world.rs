
use crate::space::Space;
use crate::input::InputTracker;
use crate::simulator::{ Simulator, SwappingSim, CellularSim };

pub struct World {
    pub run: bool,
    pub space: Space,
    pub input: InputTracker,
    simulator: Box<dyn Simulator>,
}

impl World {
    pub fn new(width: u32, height: u32) -> World {
        World {
            run: true,
            space: Space::new(width, height),
            input: InputTracker::new(),
            simulator: Box::new(SwappingSim { }),
        }
    }

    pub fn toggle_run(&mut self) {
        self.run = !self.run;
    }

    pub fn is_running(&self) -> bool {
        self.run
    }

    pub fn advance_simulation(&mut self) {
        if let Some((x, y)) = self.input.get_pos() {
            let offset = if self.space.get_generation() % 2 == 0 { 0 } else { 1 };
            self.space.add(x + offset, y, self.input.get_selected_type());
        }
        self.simulator.tick(&mut self.space);
    }
}

