use serde::{Deserialize, Serialize};

/// Severity classification for incident findings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum FindingSeverity {
    Info,
    Warning,
    Critical,
}

/// Structured deterministic diagnostic emitted by the Everust Analyzer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Finding {
    /// Unique diagnostic code, e.g. "EVERUST-L01"
    pub code: String,
    /// Concise human-readable title
    pub title: String,
    /// Detailed factual explanation of what occurred and why
    pub description: String,
    /// Severity level
    pub severity: FindingSeverity,
    /// Affected task identifier, if applicable
    pub task_id: Option<u64>,
    /// Contested lock identifier, if applicable
    pub lock_id: Option<u64>,
    /// Incident duration in nanoseconds
    pub duration_ns: Option<u64>,
    /// Timestamp where the condition peaked or was observed
    pub timestamp_ns: u64,
}

impl Finding {
    pub fn new(
        code: impl Into<String>,
        title: impl Into<String>,
        description: impl Into<String>,
        severity: FindingSeverity,
        timestamp_ns: u64,
    ) -> Self {
        Self {
            code: code.into(),
            title: title.into(),
            description: description.into(),
            severity,
            task_id: None,
            lock_id: None,
            duration_ns: None,
            timestamp_ns,
        }
    }

    pub fn with_task(mut self, task_id: u64) -> Self {
        self.task_id = Some(task_id);
        self
    }

    pub fn with_lock(mut self, lock_id: u64) -> Self {
        self.lock_id = Some(lock_id);
        self
    }

    pub fn with_duration(mut self, duration_ns: u64) -> Self {
        self.duration_ns = Some(duration_ns);
        self
    }
}
