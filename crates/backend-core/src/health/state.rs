use std::collections::HashSet;
use std::sync::{RwLock};
use std::sync::atomic::{AtomicBool, Ordering};

/// A structured reason explaining why the service is not ready.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DegradationReason {
    pub code: &'static str,
    pub message: &'static str,
}

impl DegradationReason {
    pub const fn new(code: &'static str, message: &'static str) -> Self {
        Self { code, message }
    }
}

/// Represents service health state.
///
/// - Liveness: is the process alive?
/// - Readiness: derived from degradation reasons
pub struct HealthState {
    live: AtomicBool,
    readiness_reasons: RwLock<HashSet<DegradationReason>>,
}

impl HealthState {
    /// Create a new health state.
    ///
    /// Services start as:
    /// - live = true
    /// - ready = false (implicit: has initial degradation)
    pub fn new() -> Self {
        let mut reasons = HashSet::new();
        reasons.insert(DegradationReason::new(
            "starting",
            "service is starting",
        ));

        Self {
            live: AtomicBool::new(true),
            readiness_reasons: RwLock::new(reasons),
        }
    }

    /// Mark service as alive.
    pub fn mark_alive(&self) {
        self.live.store(true, Ordering::SeqCst);
    }

    /// Mark service as not live.
    ///
    /// Typically used during shutdown.
    pub fn mark_dead(&self) {
        self.live.store(false, Ordering::SeqCst);
    }

    /// Is the service alive?
    pub fn is_live(&self) -> bool {
        self.live.load(Ordering::SeqCst)
    }

    /// Is the service ready?
    ///
    /// Readiness is derived from the absence of degradation reasons.
    pub fn is_ready(&self) -> bool {
        self.readiness_reasons.read().unwrap().is_empty()
    }

    /// Add a readiness degradation reason.
    pub fn add_degradation(&self, reason: DegradationReason) {
        self.readiness_reasons.write().unwrap().insert(reason);
    }

    /// Remove a readiness degradation reason.
    pub fn remove_degradation(&self, reason: &DegradationReason) {
        self.readiness_reasons.write().unwrap().remove(reason);
    }

    /// Clear all readiness degradation reasons.
    pub fn clear_degradations(&self) {
        self.readiness_reasons.write().unwrap().clear();
    }

    /// Get a snapshot of current readiness degradation reasons.
    pub fn degradation_reasons(&self) -> Vec<DegradationReason> {
        self.readiness_reasons
            .read()
            .unwrap()
            .iter()
            .cloned()
            .collect()
    }
}