use std::sync::Arc;
use crate::backend::backend::Backend;

pub struct LeastConn {
    backends: Vec<Arc<Backend>>,
}


impl LeastConn {

    pub fn new(backends: Vec<Arc<Backend>>) -> Self {
        Self {
            backends,
        }
    }
    pub fn next_least_conn(&self) -> Option<Arc<Backend>> {

        self.backends
            .iter()
            .filter(|b| b.is_alive())
            .min_by_key(|b| b.get_conn())
            .map(|b| Arc::clone(b))

    }
}
