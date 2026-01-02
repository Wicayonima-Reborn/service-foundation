pub mod config;
pub mod lifecycle;
pub mod shutdown;
pub mod startup;

use lifecycle::{Lifecycle, LifecycleState};
use shutdown::ShutdownCoordinator;

pub async fn shutdown_flow(
    lifecycle: &Lifecycle,
    coordinator: ShutdownCoordinator,
) {
    lifecycle.transition(LifecycleState::ShuttingDown);
    coordinator.shutdown().await;
    lifecycle.transition(LifecycleState::Terminated);
}