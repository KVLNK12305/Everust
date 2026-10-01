# Everust 🦀

> **A visual systems flight recorder and post-mortem investigation workbench for Rust applications.**

[![CI](https://img.shields.io/badge/build-passing-brightgreen.svg)]()
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Status: v0.1-alpha](https://img.shields.io/badge/status-v0.1--alpha-orange.svg)]()

---

## 1. What Everust Is

Everust is a continuous in-process flight recorder and visual investigation workbench purpose-built for Rust services.

Like a flight recorder ("black box") on an aircraft, Everust maintains a low-overhead, in-memory rolling circular buffer of runtime execution events. When an async executor stall, lock contention event, or critical error occurs, Everust snapshots the buffer into an `.evr` flight recording file. Engineers can inspect this recording using the terminal CLI or load it into the **Everust Workbench** to deterministically replay execution swimlanes, scrub the timeline, and read automated incident diagnoses.

---

## 2. The Problem

Modern async Rust (e.g., Tokio) offers incredible throughput and memory safety. However, diagnosing concurrency bugs in production remains difficult:

- **Worker Thread Stalls:** An accidental synchronous file read or blocking computation starves cooperative async tasks.
- **Lock Contention:** Sub-millisecond mutex contention builds up under concurrency spikes, while APM metrics aggregators average out and hide the tail latency.
- **Starvation:** Tasks get woken and queued, but wait tens of milliseconds before receiving worker thread CPU time.

Standard distributed tracing and logs answer *"which endpoint returned 500?"*  
Everust answers:
> **"What was my Rust service runtime actually doing, and why did it behave this way?"**

---

## 3. Architecture

Everust separates continuous low-overhead in-process recording from deterministic offline post-mortem analysis:

```text
                    EVERUST ARCHITECTURE

              ┌─────────────────┐
              │  Event Sources  │
              └────────┬────────┘
                       │
          ┌────────────┼────────────┐
          │            │            │
        Tokio      Contended Locks  Sync Stalls
          │            │            │
          └────────────┼────────────┘
                       ▼
               ┌───────────────┐
               │ Everust Agent │
               └───────┬───────┘
                       │
              ┌────────▼────────┐
              │ Flight Recorder │ (In-Memory Circular Buffer)
              └────────┬────────┘
                       │ Snapshot on trigger / stall
                       ▼
              ┌─────────────────┐
              │  .evr Artifact  │ (Flight Recording File)
              └────────┬────────┘
                       │
                 ┌─────▼─────┐
                 │  Analyzer │ (Deterministic Rules)
                 └─────┬─────┘
                       │
                 ┌─────▼─────┐
                 │ Workbench │ (SvelteKit Visual UI)
                 └───────────┘
```

### Core Crates

- **`crates/everust-core`**: Canonical event data models (`TaskSpawn`, `TaskWake`, `TaskPoll`, `LockWait`, `LockAcquire`, `BlockingOperation`), `.evr` headers, and structured `Finding` types. Independent of runtimes or UI.
- **`crates/everust-agent`**: In-process instrumentation agent. Embeds into application threads with sub-microsecond overhead.
- **`crates/everust-recorder`**: Fixed-capacity rolling ring buffer and `.evr` reader/writer.
- **`crates/everust-analyzer`**: Deterministic rule engine identifying long task polls, mutex contention, and task starvation.
- **`crates/everust-cli`**: Terminal inspection tool (`everust inspect <file.evr>`).
- **`workloads/lock-contention`**: Reproducible multi-threaded lock contention workload generating live flight recordings.
- **`web/`**: Investigation Workbench web application with interactive execution swimlanes, scrub cursor, and incident remediation guidance.
- **`foundations/`**: Pure systems engineering foundations (ownership models, borrowing rules matrix, zero-copy slices).

---

## 4. Quick Example

### Instrumenting Your Service

```rust
use everust_agent::{EverustAgent, ContendedMutex};
use everust_core::event::EventType;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize rolling in-memory flight recorder (20,000 event capacity)
    let agent = EverustAgent::init("order-service", 20_000);

    // 2. Wrap shared synchronization primitives
    let state = Arc::new(ContendedMutex::new(0));

    // 3. Normal execution (events are written to ring buffer with minimal overhead)
    {
        let mut guard = state.lock(101); // measures wait duration & hold time
        *guard += 1;
    }

    // 4. Snapshot flight recording on anomaly or shutdown
    agent.snapshot("incident.evr")?;
    Ok(())
}
```

### Inspecting an Incident via CLI

```bash
$ everust inspect incident.evr

🔍 Inspecting Everust Flight Recording: incident.evr

--- HEADER METADATA ---
  Magic:        EVR1
  Service:      order-service
  PID:          44806
  Total Events: 252

--- DETERMINISTIC ANALYZER FINDINGS ---
  1. 🔴 [CRITICAL] EVERUST-L01: High Lock Contention on Lock #1
     Task #101 stalled for 6.04ms waiting to acquire Lock #1. Mutex held by concurrent worker.
     Duration: 6.04 ms
     Target Task ID: #101

  2. 🔴 [CRITICAL] EVERUST-B01: Synchronous Blocking Call: sync_file_read_on_worker
     Task #103 invoked blocking operation 'sync_file_read_on_worker' lasting 8.50ms on worker thread.
```

---

## 5. Current Status (v0.1-alpha)

- ✅ Core canonical event model & `.evr` flight recording format specification
- ✅ Low-overhead in-memory rolling circular buffer (`everust-recorder`)
- ✅ In-process contention and poll instrumentation (`everust-agent`)
- ✅ Deterministic incident analyzer (`everust-analyzer`)
- ✅ Command-line inspection tool (`everust-cli`)
- ✅ Reproducible lock contention workload (`workloads/lock-contention`)
- ✅ Interactive visual Investigation Workbench (`web/src/routes/workbench`)
- ✅ Preserved systems foundations (`foundations/`)

---

## 6. Development Setup

### Prerequisites
- Rust 1.80+ (2021 / 2024 editions)
- Node.js 20+ & pnpm / npm (for Workbench UI)

### Running the Rust Workspace
```bash
# Verify all workspace crates
cargo check --workspace

# Run test suite
cargo test --workspace

# Run the lock-contention workload and generate an .evr recording
cargo run -p lock-contention

# Inspect the recording via Everust CLI
cargo run -p everust-cli -- inspect lock_contention.evr
```

### Running the Investigation Workbench
```bash
cd web
pnpm install
pnpm dev
```
Open `http://localhost:5173/workbench` in your browser.

---

## 7. Roadmap

- **v0.1 (Current):** In-process Tokio task poll tracking, mutex contention analysis, `.evr` serialization, and visual timeline workbench.
- **v0.2:** eBPF/kprobe scheduler integration for Linux off-CPU analysis and kernel wake latency correlation.
- **v0.3:** Allocation spike & memory pool tracker (`jemalloc` / system allocator latency correlation).
- **v0.4:** Headless background recording daemon with automated crash/stall triggers (`SIGUSR1` / latency thresholds).

---

## 📄 License

This project is licensed under the [MIT License](LICENSE).
