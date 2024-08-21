use serde::Serialize;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Serialize, Debug, Clone)]
struct ControlState {
    pub time: f64,
    pub update_count: u32,
}

struct CascadedControl {
    logger: Arc<StateLogger<ControlState>>,
}

struct StateLogger<T> {
    name: String,
    buffer_size: usize,
    _marker: std::marker::PhantomData<T>,
}

impl<T> StateLogger<T> {
    fn new(name: String, buffer_size: usize) -> Self {
        StateLogger {
            name,
            buffer_size,
            _marker: std::marker::PhantomData,
        }
    }
}

struct RustStateLoggerManager;

impl RustStateLoggerManager {
    pub fn create_logger<T: Serialize + Debug + Send + Sync + Clone + 'static>(
        &self,
        name: &str,
        buffer_size: usize,
    ) -> Arc<StateLogger<T>> {
        Arc::new(StateLogger::new(name.to_string(), buffer_size))
    }
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
