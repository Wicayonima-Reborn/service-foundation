mod state;

use std::sync::{
    atomic::{AtomicU8, Ordering},
    Arc, RwLock,
};

pub use state::{LifecycleError, LifecycleEvent, LifecycleState};

/// Observer invoked after a successful lifecycle transition.
pub type LifecycleObserver =
    Arc<dyn Fn(LifecycleEvent) + Send + Sync + 'static>;

/// Thread-safe lifecycle state tracker with strict transition validation.
pub struct Lifecycle {
    state: AtomicU8,
    observers: RwLock<Vec<LifecycleObserver>>,
}

impl Lifecycle {
    /// Create a new lifecycle initialized to `Initializing`.
    pub fn new() -> Self {
        Self {
            state: AtomicU8::new(LifecycleState::Initializing as u8),
            observers: RwLock::new(Vec::new()),
        }
    }

    /// Register a lifecycle observer.
    ///
    /// Observers are invoked after a successful state transition.
    pub fn register_observer(&self, observer: LifecycleObserver) {
        self.observers.write().unwrap().push(observer);
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

        let event = LifecycleEvent {
            from: current,
            to: next,
        };

        for observer in self.observers.read().unwrap().iter() {
            observer(event);
        }

        Ok(())
    }
}