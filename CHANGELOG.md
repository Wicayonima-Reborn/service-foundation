# Changelog

All notable changes to this project will be documented in this file.

The format is based on **Keep a Changelog**, and this project follows **Semantic Versioning** (with `0.x` allowances).

---

## [0.1.0] - 2026-01-03

### Added

* Initial public release of **Anvil** infrastructure foundation.
* `anvil-core`

  * Deterministic lifecycle state machine with strict transition validation.
  * Startup orchestrator coordinating lifecycle, health, and shutdown.
  * Framework-agnostic health state with structured readiness degradation reasons.
  * Graceful shutdown coordinator with async hook execution.
  * Minimal observability helpers for tracing/log initialization.
* `anvil-adapter-axum`

  * Thin Axum adapter exposing health and readiness over HTTP.
  * Zero business logic and no runtime ownership.

### Design Guarantees

* Framework-agnostic core with no HTTP or transport dependencies.
* Explicit behavior only (no hidden side effects or magic).
* Opt-in runtime features and conservative public API surface.

### Notes

* This is the first public release intended for early adopters.
* APIs may evolve within the `0.x` series as real-world usage informs design.

---

[0.1.0]: https://crates.io/crates/anvil-core/0.1.0
