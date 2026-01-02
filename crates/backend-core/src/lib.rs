/// Execute a minimal shutdown flow using the provided lifecycle and coordinator.
///
/// This function performs no signal handling and does not
/// assume any runtime ownership.
pub mod config;
pub mod lifecycle;
pub mod shutdown;
pub mod startup;
pub mod health;
pub mod observability;

use lifecycle::{Lifecycle, LifecycleError, LifecycleState};
use shutdown::ShutdownCoordinator;

pub async fn shutdown_flow(
    lifecycle: &Lifecycle,
    coordinator: ShutdownCoordinator,
) -> Result<(), LifecycleError> {
    lifecycle.transition(LifecycleState::ShuttingDown)?;

    coordinator.shutdown().await;

    lifecycle.transition(LifecycleState::Terminated)?;
    Ok(())
}