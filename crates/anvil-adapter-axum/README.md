# anvil-adapter-axum

Axum adapter for the `anvil-core` infrastructure foundation.

This crate provides a **thin integration layer** between `anvil-core` and the Axum web framework. It exposes core health and readiness state over HTTP without introducing business logic.

---

## What This Crate Is

* Axum-specific adapter
* Maps `anvil-core` health state to HTTP endpoints
* Contains **no business logic**
* Does not own runtime or application state

---

## Installation

```toml
[dependencies]
anvil-core = "0.1"
anvil-adapter-axum = "0.1"
axum = "0.7"
```

---

## Minimal Usage

```rust
use std::sync::Arc;
use axum::Router;
use anvil_core::startup::Startup;

let startup = Startup::new();

let app = anvil_adapter_axum::health_routes(
    startup.health(),
);
```

This adds the following endpoints:

* `GET /health/live`
* `GET /health/ready`
* `GET /health/ready/reasons`

---

## Lifecycle Integration

Health responses automatically reflect the lifecycle and health state managed by `anvil-core`.

No additional synchronization or framework hooks are required.

---

## Design Constraints

* No middleware
* No request handling logic
* No framework-specific state stored in core

If an adapter grows complex, the core design should be revisited.

---

## When to Use

Use this crate when:

* Your service uses Axum
* You want to expose health endpoints backed by `anvil-core`
* You want to keep infrastructure and application logic separate

---

## Full Example

A complete end-to-end example, including framework integration and shutdown flow,
is available in the repository root README:

👉 https://github.com/Wicayonima-Reborn/anvil#readme