use serde::{Deserialize, Serialize};

/// Discrete runtime event variants captured by the Everust Flight Recorder.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum EventType {
    /// A Tokio task has been spawned on the executor.
    TaskSpawn {
        task_id: u64,
        name: Option<String>,
    },
    /// A task has been notified by a waker and placed in the executor run queue.
    TaskWake {
        task_id: u64,
    },
    /// A task was polled by a worker thread for `elapsed_ns`.
    TaskPoll {
        task_id: u64,
        elapsed_ns: u64,
    },
    /// A task has finished execution and dropped.
    TaskComplete {
        task_id: u64,
        total_duration_ns: u64,
    },
    /// A thread or task began waiting to acquire a synchronization primitive.
    LockWait {
        lock_id: u64,
        task_id: u64,
    },
    /// A thread or task successfully acquired the lock after waiting `wait_ns`.
    LockAcquire {
        lock_id: u64,
        task_id: u64,
        wait_ns: u64,
    },
    /// A lock guard was released.
    LockRelease {
        lock_id: u64,
        task_id: u64,
        held_ns: u64,
    },
    /// A synchronous, non-yielding blocking operation executed on a worker thread.
    BlockingOperation {
        task_id: u64,
        name: String,
        duration_ns: u64,
    },
}

/// An immutable, timestamped event envelope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventRecord {
    /// Monotonically increasing sequence identifier.
    pub id: u64,
    /// Absolute timestamp in nanoseconds since Unix epoch.
    pub timestamp_ns: u64,
    /// Identifier of the operating system thread that emitted the event.
    pub thread_id: u64,
    /// The typed event payload.
    pub event: EventType,
}

impl EventRecord {
    pub fn new(id: u64, timestamp_ns: u64, thread_id: u64, event: EventType) -> Self {
        Self {
            id,
            timestamp_ns,
            thread_id,
            event,
        }
    }
}
