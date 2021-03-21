
pub struct Timer {
    start: u32,
}

impl Timer {
    pub fn start() -> Timer {
        Timer {
            start: Timer::now(),
        }
    }

    pub fn get(&self) -> u32 {
        Timer::now() - self.start
    }

    pub fn interval(&mut self) -> u32 {
        let time = Timer::now();
        let elapsed = time - self.start;
        self.start = time;
        elapsed
    }

    pub fn now() -> u32 {
        let performance = web_sys::window().unwrap().performance().unwrap();
        performance.now() as u32
    }
}

