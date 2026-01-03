# Anvil

## Overview

Anvil is a framework-agnostic backend foundation for Rust services. The project provides shared infrastructure layers that are commonly required by production backend services, without introducing a new web framework, runtime, or transport abstraction.

The goal of this project is to improve consistency, reliability, and developer experience across Rust backend services by extracting non-business concerns into a stable, composable foundation.

Anvil is designed to integrate with existing Rust web frameworks such as Axum or Actix through thin adapters, while keeping the core strictly independent of any framework.

---

## Design Principles

### Framework-Agnostic Core

The core crate does not depend on any web framework, HTTP abstraction, or transport layer. Frameworks interact with the foundation through explicit integration points.

### Explicit and Predictable Behavior

All behavior is explicit. There are no hidden side effects, global state mutations, or implicit runtime assumptions.

### Small and Stable Core

The core favors minimalism and stability over feature richness. Each module addresses a clearly defined infrastructure concern.

### Opt-In Modules

Functionality is modular and opt-in. Consumers may adopt only the components they need.

---

## Scope

Anvil focuses on infrastructure-level concerns that are common across backend services:

* Deterministic startup lifecycle
* Fail-fast configuration loading
* Graceful shutdown coordination
* Health and readiness state management
* Observability initialization helpers

The project does not implement application logic, request routing, or protocol handling.

---

## Project Structure

```
crates/
  anvil-core/
    config/
    lifecycle/
    shutdown/
    startup/
    health/
    observability/

  anvil-adapter-axum/
```

* **anvil-core**: Framework-agnostic infrastructure primitives
* **anvil-adapter-axum**: Thin Axum integration layer

Adapters are intentionally minimal. If an adapter grows complex, the core design should be reconsidered.

---

## Core Modules

### Configuration

Provides fail-fast configuration loading with explicit handling of required and optional environment variables.

### Lifecycle

Defines a deterministic service lifecycle with explicit state transitions.

### Startup Orchestrator

Coordinates service startup, lifecycle state, health signaling, and shutdown preparation.

### Shutdown Coordination

Provides a framework-agnostic mechanism to register and execute graceful shutdown hooks. Optional Tokio-based signal handling is provided behind a feature flag.

### Health State

Manages liveness and readiness state independently of any transport protocol.

### Observability

Provides minimal helpers for initializing structured logging and tracing with sensible defaults.

---

## Adapters

Adapters translate core infrastructure state into framework-specific constructs. They do not contain business logic.

The Axum adapter demonstrates how health state can be exposed over HTTP without coupling the core to Axum.

---

## Usage Model

A typical service integrates Anvil during startup:

1. Initialize configuration
2. Initialize observability
3. Create the startup orchestrator
4. Register shutdown hooks
5. Start the framework server
6. Transition to ready state
7. Execute coordinated shutdown on termination

The foundation does not start servers or manage runtimes.

---

## Quick Start (Minimal Example)

Below is a minimal example showing how to integrate **anvil-core** with an Axum service using the Axum adapter.

### Cargo.toml

```toml
[dependencies]
anvil-core = "0.1"
anvil-adapter-axum = "0.1"
axum = "0.7"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
tracing-subscriber = "0.3"
```

### main.rs

```rust
use std::net::SocketAddr;
use std::time::Duration;

use axum::Router;
use anvil_core::startup::Startup;
use anvil_core::lifecycle::LifecycleState;
use anvil_core::health::DegradationReason;

#[tokio::main]
async fn main() {
    // Initialize observability (explicit, opt-in)
    tracing_subscriber::fmt::init();

    // Create startup orchestrator
    let startup = Startup::new();

    // Example: mark service as not ready until a dependency is available
    startup.health().add_degradation(
        DegradationReason::new("db_unavailable", "database not connected"),
    );

    // Build Axum router from adapter
    let app = anvil_adapter_axum::health_routes(startup.health());

    // Simulate startup phases
    tokio::spawn({
        let startup = startup;
        async move {
            startup
                .lifecycle()
                .transition(LifecycleState::Starting)
                .expect("valid lifecycle transition");

            tokio::time::sleep(Duration::from_secs(3)).await;

            startup
                .mark_ready()
                .expect("service should be able to enter Ready state");
        }
    });

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app,
    )
    .await
    .unwrap();
}
```

This example demonstrates:

* Explicit lifecycle transitions
* Health readiness management with degradation reasons
* Framework integration via a thin adapter
* No framework or runtime ownership inside the core

---

## Non-Goals

Anvil does not:

* Define HTTP routes or middleware
* Replace existing Rust web frameworks
* Provide a runtime or async executor
* Enforce architectural patterns on application code

---

## Intended Audience

* Rust backend developers
* Teams maintaining multiple Rust services
* Platform and infrastructure engineers

---

## Project Status

This project is under active development. The API aims to remain stable and conservative, prioritizing long-term maintainability over rapid feature expansion.
