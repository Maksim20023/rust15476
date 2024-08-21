use serde::ser::Serialize;
use std::fmt::Debug;
use std::sync::Arc;

pub struct StateLogger<T> {
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

pub struct RustStateLoggerManager;

impl RustStateLoggerManager {
    pub fn create_logger<T: Serialize + Debug + Send + Sync + Clone + 'static>(
        &self,
        name: &str,
        buffer_size: usize,
    ) -> Arc<StateLogger<T>> {
        Arc::new(StateLogger::new(name.to_string(), buffer_size))
    }
}