
#[derive(Debug)]
pub struct Logger {
    name: &'static str,
    debug_mode: bool
}

impl Clone for Logger {
    fn clone(&self) -> Self {
        Self { name: self.name.clone(), debug_mode: self.debug_mode.clone() }
    }
}

impl Logger {
    pub fn new (name: &'static str, debug_mode: bool) -> Logger {
        Logger {
            name,
            debug_mode
        }
    }

    pub fn warn (&self, message: &str) -> () {
        println!("[WARN] [{}] {}", self.name, message);
    }

    pub fn error (&self, message: &str) -> () {
        panic!("[ERROR] [{}] {}", self.name, message);
    }

    pub fn log (&self, message: &str) -> () {
        println!("[LOG] [{}] {}", self.name, message);
    }

    pub fn debug (&self, message: &str) -> () {
        if !self.debug_mode {
            return;
        }

        println!("[DEBUG] [{}] {}", self.name, message);
    }
}