use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

pub struct Backend {
    pub address: String,
    pub alive: AtomicBool,
    pub current_connections: AtomicUsize,
    pub fail_count: AtomicUsize,
}

impl Backend {
    
    pub fn new(address: String) -> Self {
        Self {
            address,
            alive: AtomicBool::new(true),
            current_connections: AtomicUsize::new(0),
            fail_count: AtomicUsize::new(0),
        }
    }
    
    pub fn is_alive(&self) -> bool {
        self.alive.load(Ordering::SeqCst)
    }
    
    pub fn set_alive(&self, value: bool) {
        self.alive.store(value, Ordering::SeqCst)
    }
    
    pub fn get_alive(&self) -> bool {
        self.alive.load(Ordering::SeqCst)
    }
    
    pub fn inc_conn(&self) {
        self.current_connections.fetch_add(1, Ordering::SeqCst);
    }
    
    pub fn dec_conn(&self) {
        self.current_connections.fetch_sub(1, Ordering::SeqCst);
    }
    
    pub fn get_conn(&self) -> usize {
        self.current_connections.load(Ordering::SeqCst)
    }
    
    pub fn inc_fail(&self) {
        self.fail_count.fetch_add(1, Ordering::SeqCst);
    }
    
    pub fn reset_fail(&self) {
        self.fail_count.store(0, Ordering::SeqCst);
    }
    
    pub fn get_fail(&self) -> usize {
        self.fail_count.load(Ordering::SeqCst)
    }
}