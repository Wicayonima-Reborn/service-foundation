mod state;

use std::sync::atomic::{AtomicU8, Ordering};

pub use state::{LifecycleError, LifecycleState};

/// Thread-safe lifecycle state tracker with strict transition validation.
///
/// This type does not own any runtime, threading model,
/// or transport concerns.
pub struct Lifecycle {
    state: AtomicU8,
}

impl Lifecycle {
    /// Create a new lifecycle initialized to `Initializing`.
    pub fn new() -> Self {
        Self {
            state: AtomicU8::new(LifecycleState::Initializing as u8),
        }
    }

    /// Get the current lifecycle state.
    pub fn state(&self) -> LifecycleState {
        match self.state.load(Ordering::SeqCst) {
            0 => LifecycleState::Initializing,
            1 => LifecycleState::Starting,
            2 => LifecycleState::Ready,
            3 => LifecycleState::ShuttingDown,
            _ => LifecycleState::Terminated,
        }
    }

    /// Transition to the next lifecycle state.
    ///
    /// Returns an error if the transition is invalid.
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