# Contributing to Everust

Thank you for your interest in contributing to Everust!

## Codebase Principles
1. **Core Runtime Safety:** We build tools that run inside latency-sensitive Rust production services. Memory safety, low overhead, and deterministic execution are top priorities.
2. **Explicit Separation of Concerns:**
   - `crates/everust-core`: Pure schemas, data models, and records. No runtime or UI dependencies.
   - `crates/everust-agent`: In-process instrumentation. Must never panic in client application threads.
   - `crates/everust-recorder`: Ring buffer and `.evr` file serialization. Low allocation overhead.
   - `crates/everust-analyzer`: Deterministic rule evaluation over event streams.
   - `crates/everust-cli`: User-facing terminal utilities.
   - `web/`: Visual investigation workbench.
3. **Reproducible Workloads:** All runtime diagnostic rules in `everust-analyzer` must be verified against reproducible test workloads under `workloads/`.

## Local Development Setup

### Rust Workspace
Ensure you have a modern Rust toolchain installed (edition 2021/2024 compatible):
```bash
cargo check --workspace
cargo test --workspace
```

### Everust Workbench (`web/`)
```bash
cd web
pnpm install # or npm install
pnpm dev
pnpm check
```

## Pull Request Guidelines
- Always verify that all workspace crates compile and tests pass.
- Run `cargo clippy --workspace` and `cargo fmt --check` before submitting.
- Provide descriptive pull request summaries detailing the problem addressed and verification steps taken.
