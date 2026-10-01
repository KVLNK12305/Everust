use everust_core::event::EventType;
use everust_core::record::FlightRecording;
use everust_recorder::{RollingBuffer, write_recording, RecorderError};
use std::ops::{Deref, DerefMut};
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;

/// Active in-process flight recorder agent.
#[derive(Clone)]
pub struct EverustAgent {
    buffer: Arc<RollingBuffer>,
    service_name: String,
}

static AGENT_INSTANCE: parking_lot::RwLock<Option<EverustAgent>> = parking_lot::RwLock::new(None);
static NEXT_LOCK_ID: AtomicU64 = AtomicU64::new(1);

impl EverustAgent {
    /// Initializes the global in-process agent with a given buffer capacity.
    pub fn init(service_name: impl Into<String>, capacity: usize) -> Self {
        let agent = Self {
            buffer: Arc::new(RollingBuffer::new(capacity)),
            service_name: service_name.into(),
        };
        let mut guard = AGENT_INSTANCE.write();
        *guard = Some(agent.clone());
        agent
    }

    /// Retrieves a clone of the global agent instance, if initialized.
    pub fn current() -> Option<Self> {
        AGENT_INSTANCE.read().clone()
    }

    /// Emits a raw event into the active rolling buffer.
    pub fn emit(&self, event: EventType) -> u64 {
        let thread_id = std::thread::current().id();
        // Pack ThreadId into a u64 hash representation
        let tid_u64 = format!("{:?}", thread_id)
            .strip_prefix("ThreadId(")
            .and_then(|s| s.strip_suffix(')'))
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(0);

        self.buffer.push(tid_u64, event)
    }

    /// Snapshots current in-memory events and writes an `.evr` flight recording to disk.
    pub fn snapshot<P: AsRef<Path>>(&self, path: P) -> Result<FlightRecording, RecorderError> {
        let events = self.buffer.snapshot();
        let recording = FlightRecording::new(self.service_name.clone(), events);
        write_recording(path, &recording)?;
        Ok(recording)
    }

    /// Instruments a synchronous or async block, recording poll duration.
    pub fn instrument_poll<R>(&self, task_id: u64, f: impl FnOnce() -> R) -> R {
        let start = Instant::now();
        let result = f();
        let elapsed_ns = start.elapsed().as_nanos() as u64;

        self.emit(EventType::TaskPoll {
            task_id,
            elapsed_ns,
        });

        result
    }

    /// Records a blocking operation executed on an async thread.
    pub fn record_blocking_op(&self, task_id: u64, name: impl Into<String>, duration_ns: u64) {
        self.emit(EventType::BlockingOperation {
            task_id,
            name: name.into(),
            duration_ns,
        });
    }

    pub fn buffer(&self) -> &Arc<RollingBuffer> {
        &self.buffer
    }
}

/// A wrapper around `parking_lot::Mutex<T>` that transparently measures lock contention,
/// wait duration, and hold duration, emitting telemetry to Everust.
pub struct ContendedMutex<T> {
    inner: parking_lot::Mutex<T>,
    lock_id: u64,
}

pub struct ContendedMutexGuard<'a, T> {
    guard: parking_lot::MutexGuard<'a, T>,
    lock_id: u64,
    task_id: u64,
    acquire_time: Instant,
}

impl<T> ContendedMutex<T> {
    pub fn new(val: T) -> Self {
        Self {
            inner: parking_lot::Mutex::new(val),
            lock_id: NEXT_LOCK_ID.fetch_add(1, Ordering::Relaxed),
        }
    }

    pub fn lock(&self, task_id: u64) -> ContendedMutexGuard<'_, T> {
        if let Some(agent) = EverustAgent::current() {
            agent.emit(EventType::LockWait {
                lock_id: self.lock_id,
                task_id,
            });
        }

        let start = Instant::now();
        let guard = self.inner.lock();
        let wait_ns = start.elapsed().as_nanos() as u64;

        if let Some(agent) = EverustAgent::current() {
            agent.emit(EventType::LockAcquire {
                lock_id: self.lock_id,
                task_id,
                wait_ns,
            });
        }

        ContendedMutexGuard {
            guard,
            lock_id: self.lock_id,
            task_id,
            acquire_time: Instant::now(),
        }
    }

    pub fn lock_id(&self) -> u64 {
        self.lock_id
    }
}

impl<'a, T> Deref for ContendedMutexGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.guard
    }
}

impl<'a, T> DerefMut for ContendedMutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.guard
    }
}

impl<'a, T> Drop for ContendedMutexGuard<'a, T> {
    fn drop(&mut self) {
        let held_ns = self.acquire_time.elapsed().as_nanos() as u64;
        if let Some(agent) = EverustAgent::current() {
            agent.emit(EventType::LockRelease {
                lock_id: self.lock_id,
                task_id: self.task_id,
                held_ns,
            });
        }
    }
}
