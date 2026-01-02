mod state;

use std::sync::atomic::{AtomicU8, Ordering};

pub use state::{LifecycleError, LifecycleState};

pub struct Lifecycle {
    state: AtomicU8,
}

impl Lifecycle {
    pub fn new() -> Self {
        Self {
            state: AtomicU8::new(LifecycleState::Initializing as u8),
        }
    }

    pub fn state(&self) -> LifecycleState {
        match self.state.load(Ordering::SeqCst) {
            0 => LifecycleState::Initializing,
            1 => LifecycleState::Starting,
            2 => LifecycleState::Ready,
            3 => LifecycleState::ShuttingDown,
            _ => LifecycleState::Terminated,
        }
    }

    pub fn transition(&self, next: LifecycleState) -> Result<(), LifecycleError> {
        let current = self.state();

        if !current.can_transition_to(next) {
            return Err(LifecycleError::InvalidTransition {
                from: current,
                to: next,
            });
        }

        self.state.store(next as u8, Ordering::SeqCst);
        Ok(())
    }
}