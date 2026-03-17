use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use crate::backend::backend::Backend;

pub struct RoundRobin {
    backends: Vec<Arc<Backend>>,
    counter: AtomicUsize,
}

impl RoundRobin {
    pub fn new(backends: Vec<Arc<Backend>>) -> Self {
        Self {
            backends,
            counter: AtomicUsize::new(0),
        }
    }

    pub fn next(&self) -> Option<Arc<Backend>> {
        
        // Check Total Backend
        if self.backends.is_empty() {
            return None
        }
        
        let total = self.backends.len();

        for _ in 0..total {

            let index = self.counter.fetch_add(1, Ordering::SeqCst);
            let backend = &self.backends[index % total];

            if backend.is_alive() {
                return Some(Arc::clone(backend));
            }
        }

        None
    }
}