use crate::lifecycle::{Lifecycle, LifecycleState};
use crate::shutdown::ShutdownCoordinator;
use crate::health::HealthState;
use std::sync::Arc;

/// Core startup orchestrator.
///
/// Coordinates lifecycle, health, and shutdown concerns
/// without owning any runtime or framework.
pub struct Startup {
    lifecycle: Lifecycle,
    shutdown_coordinator: ShutdownCoordinator,
    health: Arc<HealthState>,
}

impl Startup {
    pub fn new() -> Self {
        Self {
            lifecycle: Lifecycle::new(),
            shutdown_coordinator: ShutdownCoordinator::new(),
            health: Arc::new(HealthState::new()),
        }
    }

    pub fn lifecycle(&self) -> &Lifecycle {
        &self.lifecycle
    }

    pub fn health(&self) -> Arc<HealthState> {
        Arc::clone(&self.health)
    }

    pub fn shutdown(&self) -> &ShutdownCoordinator {
        &self.shutdown_coordinator
    }

    /// Mark the service as ready.
    ///
    /// Fails if the lifecycle is not in a valid state.
    pub fn mark_ready(&self) -> Result<(), crate::lifecycle::LifecycleError> {
        self.lifecycle.transition(LifecycleState::Ready)?;
        self.health.clear_degradations();
        Ok(())
    }

    /// Execute a full shutdown sequence.
    pub async fn shutdown_now(self) -> Result<(), crate::lifecycle::LifecycleError> {
        self.lifecycle.transition(LifecycleState::ShuttingDown)?;
        self.health.mark_dead();

        self.shutdown_coordinator.shutdown().await;

        self.lifecycle.transition(LifecycleState::Terminated)?;
        Ok(())
    }
}