use everust_core::event::{EventRecord, EventType};
use parking_lot::RwLock;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

/// A thread-safe, memory-backed circular ring buffer for runtime events.
pub struct RollingBuffer {
    capacity: usize,
    buffer: RwLock<Vec<Option<EventRecord>>>,
    head: AtomicU64,
    event_seq: AtomicU64,
}

impl RollingBuffer {
    /// Creates a new rolling buffer with a specified fixed capacity.
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "Capacity must be greater than zero");
        let mut initial = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            initial.push(None);
        }

        Self {
            capacity,
            buffer: RwLock::new(initial),
            head: AtomicU64::new(0),
            event_seq: AtomicU64::new(1),
        }
    }

    /// Pushes a new event into the buffer, overwriting the oldest event if full.
    pub fn push(&self, thread_id: u64, event: EventType) -> u64 {
        let seq = self.event_seq.fetch_add(1, Ordering::Relaxed);
        let timestamp_ns = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0);

        let record = EventRecord::new(seq, timestamp_ns, thread_id, event);

        let idx = (self.head.fetch_add(1, Ordering::Relaxed) as usize) % self.capacity;
        let mut guard = self.buffer.write();
        guard[idx] = Some(record);

        seq
    }

    /// Snapshots all active records in chronological sequence.
    pub fn snapshot(&self) -> Vec<EventRecord> {
        let current_head = self.head.load(Ordering::Relaxed) as usize;
        let guard = self.buffer.read();

        let mut records = Vec::with_capacity(self.capacity);
        if current_head < self.capacity {
            // Buffer has not yet wrapped
            for item in guard.iter().take(current_head) {
                if let Some(r) = item {
                    records.push(r.clone());
                }
            }
        } else {
            // Buffer has wrapped: read from (head % capacity) to end, then 0 to (head % capacity)
            let start = current_head % self.capacity;
            for i in start..self.capacity {
                if let Some(r) = &guard[i] {
                    records.push(r.clone());
                }
            }
            for i in 0..start {
                if let Some(r) = &guard[i] {
                    records.push(r.clone());
                }
            }
        }

        records
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn total_recorded(&self) -> u64 {
        self.event_seq.load(Ordering::Relaxed).saturating_sub(1)
    }
}
