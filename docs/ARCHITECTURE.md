# Everust Architecture

## Overview

Everust follows a pipeline architecture designed for minimal in-process overhead during normal execution and deterministic offline investigation during incidents:

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
              │ Flight Recorder │ (Rolling Memory Ring Buffer)
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
                 │ Workbench │ (Interactive Svelte UI)
                 └───────────┘
```

## System Components

### 1. `everust-core`
The bedrock crate containing the canonical event schema, flight recording containers, and structured findings. It depends only on standard traits and serde serialization, keeping it independent of runtimes, storage, and presentation layers.

### 2. `everust-agent`
In-process instrumentation library embedded directly into Rust services.
- **Tokio Instrumentation:** Measures task polling latency and tracks task wake-to-poll delay.
- **Lock Contention Tracking:** Intercepts mutex acquisition to measure wait times and hold durations.
- **Blocking Operation Sentinel:** Captures uncooperative synchronous operations blocking worker threads.

### 3. `everust-recorder`
Maintains a fixed-capacity, thread-safe circular ring buffer in application memory.
- Writes are lock-free or partitioned to ensure sub-microsecond overhead.
- Oldest events are continuously overwritten until a snapshot is requested (via signal, programmatic API, or stall trigger).
- Serializes the recording into the canonical `.evr` format.

### 4. `everust-analyzer`
Deterministic diagnostic engine that operates over recorded event streams:
- Identifies tasks starved waiting for executor worker threads.
- Flags long task polls violating the async cooperative contract (>5ms default).
- Pinpoints mutex contention hot-spots and greedy lock acquisitions.
- Emits structured `Finding` objects with exact timestamps, task IDs, and duration metrics.

### 5. `everust-cli`
The developer workstation CLI providing quick investigation commands:
- `everust record`: Executes or instruments a target workload.
- `everust inspect <file.evr>`: Parses flight recordings and outputs analyzer findings in terminal format.
- `everust demo <workload>`: Generates reference recordings for known failure modes.

### 6. Everust Workbench (`web/`)
A responsive, high-performance SvelteKit web application for visual post-mortem investigation:
- Interactive timeline showing task states and thread activity.
- Contention graph displaying lock hold times and waiting tasks.
- Finding details with remediation guidance.
