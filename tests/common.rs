use log::LevelFilter;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub fn setup() {
    lazy_static::lazy_static! { static ref SETUP_DONE: Arc<AtomicBool> = Arc::new(AtomicBool::new(false)); }
    if !SETUP_DONE.swap(true, Ordering::SeqCst) {
        env_logger::builder()
            .filter_level(LevelFilter::Trace)
            .init();
    }
}