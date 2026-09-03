use signal_hook::consts::SIGINT;
use signal_hook::flag;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

pub struct SignalHandler {
    pub interrupted: Arc<AtomicBool>,
}

impl Default for SignalHandler {
    fn default() -> Self {
        let interrupted = Arc::new(AtomicBool::new(false));

        flag::register(SIGINT, Arc::clone(&interrupted))
            .expect("failed to register SIGINT handler");

        Self { interrupted }
    }
}

impl SignalHandler {
    pub fn was_interrupted(&self) -> bool {
        self.interrupted.swap(false, Ordering::Relaxed)
    }
}
