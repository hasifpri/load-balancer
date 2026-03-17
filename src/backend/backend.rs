use std::sync::atomic::{AtomicBool, Ordering};

pub struct Backend {
    pub address: String,
    pub alive: AtomicBool,
}

impl Backend {
    
    pub fn new(address: String) -> Self {
        Self {
            address,
            alive: AtomicBool::new(true),
        }
    }
    
    pub fn is_alive(&self) -> bool {
        self.alive.load(Ordering::SeqCst)
    }
    
    pub fn set_alive(&self, value: bool) {
        self.alive.store(value, Ordering::SeqCst)
    }
}