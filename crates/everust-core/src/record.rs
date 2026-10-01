use crate::event::EventRecord;
use serde::{Deserialize, Serialize};

/// Canonical file header for `.evr` flight recordings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FlightRecordingHeader {
    /// Format identifier: "EVR1"
    pub magic: String,
    /// Schema format version (starts at 1)
    pub version: u32,
    /// Target service or process name
    pub service_name: String,
    /// Operating system process ID
    pub pid: u32,
    /// Recording start timestamp (Unix epoch nanoseconds)
    pub start_timestamp_ns: u64,
    /// Recording end timestamp (Unix epoch nanoseconds)
    pub end_timestamp_ns: u64,
    /// Total count of events stored in this snapshot
    pub total_events: u64,
}

impl Default for FlightRecordingHeader {
    fn default() -> Self {
        Self {
            magic: "EVR1".to_string(),
            version: 1,
            service_name: "rust-service".to_string(),
            pid: std::process::id(),
            start_timestamp_ns: 0,
            end_timestamp_ns: 0,
            total_events: 0,
        }
    }
}

/// A complete flight recording container ready for serialization / inspection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlightRecording {
    pub header: FlightRecordingHeader,
    pub events: Vec<EventRecord>,
}

impl FlightRecording {
    pub fn new(service_name: String, events: Vec<EventRecord>) -> Self {
        let (start, end) = if let (Some(first), Some(last)) = (events.first(), events.last()) {
            (first.timestamp_ns, last.timestamp_ns)
        } else {
            (0, 0)
        };

        let header = FlightRecordingHeader {
            magic: "EVR1".to_string(),
            version: 1,
            service_name,
            pid: std::process::id(),
            start_timestamp_ns: start,
            end_timestamp_ns: end,
            total_events: events.len() as u64,
        };

        Self { header, events }
    }
}
