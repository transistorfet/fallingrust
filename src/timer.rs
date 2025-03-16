// This file provides timing functionality for the simulation.
// It measures elapsed time and helps maintain a consistent frame rate.

// The Timer struct keeps track of a starting time and provides methods to
// measure how much time has passed since that starting point.
pub struct Timer {
    start: u32,  // Start time in milliseconds (since the page loaded)
}

impl Timer {
    // Creates a new Timer and sets its start time to the current time
    pub fn start() -> Timer {
        Timer {
            start: Timer::now(), // Set the starting time to now
        }
    }

    // Returns how many milliseconds have passed since the timer was started
    pub fn get(&self) -> u32 {
        Timer::now() - self.start
    }

    // Measures the time since the last call to interval() and resets the timer
    // This is useful for measuring frame times in animation loops
    pub fn interval(&mut self) -> u32 {
        let time = Timer::now();       // Get the current time
        let elapsed = time - self.start; // Calculate elapsed time
        self.start = time;              // Reset the start time to now
        elapsed                         // Return the elapsed time
    }

    // Gets the current time in milliseconds
    // Uses the browser's high-precision performance.now() method
    pub fn now() -> u32 {
        // Access the browser's window object
        let performance = web_sys::window().unwrap().performance().unwrap();
        // Get the current time in milliseconds and convert it to a u32
        performance.now() as u32
    }
}

