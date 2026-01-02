#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum LifecycleState {
    Initializing = 0,
    Starting = 1,
    Ready = 2,
    ShuttingDown = 3,
    Terminated = 4,
}

#[derive(Debug)]
pub enum LifecycleError {
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