# Everust Flight Recording Format Specification (.evr)

## Status: v0.1-draft

The `.evr` (Everust Flight Recording) format is the file standard used by Everust to store, transfer, and analyze recorded in-process runtime events.

For **v0.1**, the format prioritizes **correctness, inspectability, and robust versioning** over premature binary compression.

---

## 1. File Structure

An `.evr` container is structured into three clear segments:

```text
┌──────────────────────────────────────────────┐
│ Header (Magic, Version, Service Metadata)    │  (Fixed / Length-prefixed)
├──────────────────────────────────────────────┤
│ String Interning Table (String Dictionary)   │  (Deduplication segment)
├──────────────────────────────────────────────┤
│ Event Stream (Chronological Event Envelopes) │  (Stream of recorded records)
└──────────────────────────────────────────────┘
```

For v0.1, two serialization variants are supported:
- **`evr-json`**: Human-readable and debug-friendly JSON representation used during development and browser interchange.
- **`evr-framed`**: Compact length-delimited binary format for production recording dumps.

---

## 2. Header Specification

```rust
#[repr(C)]
pub struct EvrHeader {
    pub magic: [u8; 4],             // ASCII "EVR1" (0x45, 0x56, 0x52, 0x31)
    pub format_version: u32,        // Format schema version (currently 1)
    pub service_name: String,       // Target process / service identifier
    pub pid: u32,                   // OS Process ID
    pub start_timestamp_ns: u64,    // Epoch timestamp (nanoseconds) when recording started
    pub end_timestamp_ns: u64,      // Epoch timestamp when recording ended / snapshotted
    pub total_events: u64,          // Number of events contained in stream
    pub dropped_events: u64,        // Events dropped if rolling buffer overflowed without snapshot
}
```

### Magic Bytes
- Bytes 0-3 must equal `b"EVR1"`. Parsers encountering mismatched magic must reject the file with `InvalidMagicError`.

### Versioning & Compatibility
- `format_version`: Incremented upon breaking schema changes. Parsers must check `format_version <= SUPPORTED_VERSION`.

---

## 3. Identifiers & Timestamps

- **Timestamps:** 64-bit unsigned integers representing nanoseconds since the Unix epoch (`u64`). Nanosecond resolution avoids rounding errors during sub-millisecond task poll measurements.
- **Task Identifiers (`task_id`):** 64-bit unsigned integers (`u64`) assigned monotonically during `TaskSpawn`.
- **Thread Identifiers (`thread_id`):** 64-bit unsigned integer representing the OS thread or runtime worker index.
- **Lock Identifiers (`lock_id`):** 64-bit hash or unique allocation address of the contested synchronization primitive.

---

## 4. String Handling & Interning Strategy

Dynamic string allocations (such as task names, lock labels, and subsystem tags) are interned in a string dictionary table at the beginning of the file:
- Each distinct string is mapped to a `u32` string ID.
- Event records reference strings via `u32` string IDs instead of repeating variable-length strings inline.
- This reduces memory overhead in high-throughput loops by over 80%.

---

## 5. Event Stream Schema (v0.1)

Each event envelope consists of:
1. `id`: Monotonic event index (`u64`).
2. `timestamp_ns`: Absolute event timestamp (`u64`).
3. `thread_id`: Recording thread identifier (`u64`).
4. `payload`: Tagged event union:

| Event Variant | Attributes | Description |
|---|---|---|
| `TaskSpawn` | `task_id: u64`, `name_id: Option<u32>` | A Tokio task is spawned. |
| `TaskWake` | `task_id: u64` | A task is woken and queued for scheduling. |
| `TaskPoll` | `task_id: u64`, `elapsed_ns: u64` | Worker thread polled task for `elapsed_ns`. |
| `TaskComplete` | `task_id: u64`, `duration_ns: u64` | Task concluded execution. |
| `LockWait` | `lock_id: u64`, `task_id: u64` | Thread/task initiated wait on mutex. |
| `LockAcquire` | `lock_id: u64`, `task_id: u64`, `wait_ns: u64` | Lock acquired after `wait_ns` delay. |
| `BlockingOperation` | `task_id: u64`, `name_id: u32`, `duration_ns: u64` | Blocking synchronous call executed on worker. |

---

## 6. Indexing & Seeking Strategy

For large recordings in v0.2+:
- Checkpoint indexes will be appended every 10,000 events with stream offsets to allow $O(1)$ random seeking to arbitrary timestamps.
- For v0.1, files are small enough (< 50MB) to be streamed sequentially into memory.

---

## 7. Compression Strategy

- **v0.1:** Uncompressed or raw `.evr` with direct JSON export for browser workbench inspection.
- **v0.2 Roadmap:** Optional streaming Zstandard (`zstd`) compression block framing for disk efficiency.
