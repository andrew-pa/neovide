use std::{
    process::ExitCode,
    sync::{
        atomic::{AtomicBool, AtomicU8, Ordering},
        Arc,
    },
};

use log::info;

#[derive(Clone)]
pub struct RunningTracker {
    exit_code: Arc<AtomicU8>,
    quitting: Arc<AtomicBool>,
}

impl RunningTracker {
    pub fn new() -> Self {
        Self {
            exit_code: Arc::new(AtomicU8::new(0)),
            quitting: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn quit_with_code(&self, code: u8, reason: &str) {
        self.exit_code.store(code, Ordering::Release);
        self.request_quit();
        info!("Quit with code {}: {}", code, reason);
    }

    pub fn request_quit(&self) {
        self.quitting.store(true, Ordering::Release);
    }

    pub fn quit_requested(&self) -> bool {
        self.quitting.load(Ordering::Acquire)
    }

    pub fn exit_code(&self) -> ExitCode {
        ExitCode::from(self.exit_code.load(Ordering::Acquire))
    }
}
