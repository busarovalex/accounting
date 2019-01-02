use std::sync::Mutex;

use registry::Registry;

#[derive(Debug)]
pub struct AppState {
    pub registry: Mutex<Registry>,
}

impl AppState {
    pub fn new(registry: Registry) -> AppState {
        AppState {
            registry: Mutex::new(registry),
        }
    }
}
