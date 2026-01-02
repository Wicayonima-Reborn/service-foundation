/// Discrete lifecycle states for a service.
///
/// States follow a strict linear progression and
/// cannot be skipped or reordered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum LifecycleState {
    /// Initial contruction phase.
    Initializing = 0,
    /// Startup in progress.
    Starting = 1,
    /// Service is ready to accept traffic.
    Ready = 2,
    /// Shutdown has been initiated.
    ShuttingDown = 3,
    /// Service has fully terminated.
    Terminated = 4,
}

/// Errors returned when lifecycle rules are violated.
#[derive(Debug)]
pub enum LifecycleError {
    /// Returned when an invalid state transition is attempted.
    InvalidTransition {
        from: LifecycleState,
        to: LifecycleState,
    },
}

impl LifecycleState {
    pub(crate) fn can_transition_to(self, next: LifecycleState) -> bool {
        use LifecycleState::*;

        matches!(
            (self, next),
            (Initializing, Starting)
                | (Starting, Ready)
                | (Ready, ShuttingDown)
                | (ShuttingDown, Terminated)
        )
    }
}

// Lifecycle transition event
#[derive(Debug, Clone, Copy)]
pub struct LifecycleEvent {
    pub from: LifecycleState,
    pub to: LifecycleState,
}