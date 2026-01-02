use crate::lifecycle::{Lifecycle, LifecycleState};
use crate::shutdown::ShutdownCoordinator;
use crate::health::HealthState;
use std::sync::Arc;

/// Core startup orchestrator.
///
/// Coordinates service startup and shutdown lifecycle
/// without owning any runtime or framework concerns.
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

    /// Mark service as ready.
    pub fn mark_ready(&self) -> Result<(), crate::lifecycle::LifecycleError> {
        self.lifecycle.transition(LifecycleState::Ready)?;
        self.health.mark_ready();
        Ok(())
    }

    /// Execute shutdown flow.
    pub async fn shutdown_now(self) -> Result<(), crate::lifecycle::LifecycleError> {
        self.lifecycle.transition(LifecycleState::ShuttingDown)?;
        self.health.mark_not_ready();
        self.health.mark_dead();

        self.shutdown_coordinator.shutdown().await;

        self.lifecycle.transition(LifecycleState::Terminated)?;
        Ok(())
    }
}