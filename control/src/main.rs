use serde::Serialize;
use std::fmt::Debug;
use std::sync::Arc;
use logging::{RustStateLoggerManager, StateLogger};

#[derive(Serialize, Debug, Clone)]
struct ControlState {
    pub time: f64,
    pub update_count: u32,
}

struct CascadedControl {
    logger: Arc<StateLogger<ControlState>>,
}

impl CascadedControl {
    pub fn new(logger_manager: &RustStateLoggerManager) -> Self {
        let logger = logger_manager.create_logger::<ControlState>("control/cascaded_control", 1000);

        CascadedControl { logger }
    }
}

fn main() {
    let logger_manager = RustStateLoggerManager;
    let control = CascadedControl::new(&logger_manager);
}
