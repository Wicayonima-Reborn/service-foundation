# anvil-core

Framework-agnostic infrastructure foundation for Rust backend services.

`anvil-core` provides **production-oriented primitives** such as lifecycle management, health state, graceful shutdown coordination, and minimal observability helpers — without owning any web framework, runtime, or transport.

---

## What This Crate Is

* Deterministic lifecycle state machine
* Explicit startup orchestration
* Framework-agnostic health & readiness state
* Graceful shutdown coordination
* Minimal observability initialization helpers

This crate contains **infrastructure only**. No HTTP, no routing, no runtime ownership.

---

## Installation

```toml
[dependencies]
anvil-core = "0.1"
```

---

## Minimal Usage

```rust
use anvil_core::startup::Startup;
use anvil_core::lifecycle::LifecycleState;

let startup = Startup::new();

startup
    .lifecycle()
    .transition(LifecycleState::Starting)?;

// Application initialization here

startup.mark_ready()?;
```

---

## Health State

```rust
let health = startup.health();

if health.is_ready() {
    // service can receive traffic
}
```

Health state is transport-agnostic and can be exposed via adapters.

---

## Shutdown Coordination

```rust
startup.shutdown().register(async {
    // cleanup logic
});
```

Shutdown hooks are executed deterministically during service termination.

---

## Design Guarantees

* No implicit runtime assumptions
* No hidden side effects
* Explicit error handling
* Thread-safe (`Send + Sync`) core types

---

## When to Use

Use `anvil-core` when you want:

* Consistent service lifecycle across projects
* Explicit startup/shutdown behavior
* Infrastructure primitives independent of frameworks

---

## Full Example

A complete end-to-end example, including framework integration and shutdown flow,
is available in the repository root README:

👉 https://github.com/Wicayonima-Reborn/anvil#readme