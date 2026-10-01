# Everust Vision

## The Problem
Async Rust offers high throughput, memory safety, and fine-grained resource control. However, when things go wrong in production, diagnosing runtime problems remains painful:
- A Tokio worker thread gets blocked by a synchronous call or an uninstrumented lock.
- Tasks experience subtle starvation under mutex contention that metrics aggregators hide inside 10-second averages.
- Tail latencies spike, but distributed traces only show that a span took 800ms—not *why* the executor stalled or *which* lock was contested.

Standard logging, APM counters, and distributed tracing are coarse-grained. They answer *"where did the request fail?"* but cannot answer:
> **"What was my Rust service runtime actually doing, and why did it behave this way?"**

## The Product
**Everust** is a visual systems flight recorder and investigation workbench purpose-built for Rust applications.

Like an airplane's black box, Everust maintains a continuous, low-overhead rolling flight recording of in-process runtime events:
1. **Task lifecycle transitions:** spawn, wake, poll duration, completion.
2. **Lock contention:** wait times, hold durations, and contending threads/tasks.
3. **Runtime stalls:** blocking operations executed on async worker threads.

When an anomaly or stall occurs, Everust snapshots the rolling buffer to an `.evr` flight recording file. Engineers can load this file into the **Everust Workbench** to deterministically replay the timeline, inspect stalled tasks, and read automated diagnoses.

## Core Philosophy
1. **Deterministic, Not Guesswork:** Analysis is performed by deterministic rule engines inspecting real timestamped runtime events—no probabilistic approximations or LLMs.
2. **Low-Overhead Rolling Buffer:** Record only what matters in a fixed-size ring buffer. Zero heap churn during active recording.
3. **Investigate Concurrency & Runtime Behavior First:** Focus on async executor stalls, lock contention, and task starvation before attempting generic observability.
4. **Inspectable Open Format (`.evr`):** Open, reproducible flight recordings that can be archived, attached to incident tickets, or analyzed offline.
