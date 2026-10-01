pub mod event;
pub mod finding;
pub mod record;

pub use event::{EventRecord, EventType};
pub use finding::{Finding, FindingSeverity};
pub use record::{FlightRecording, FlightRecordingHeader};

#[derive(thiserror::Error, Debug)]
pub enum CoreError {
    #[error("Invalid flight recording magic bytes: expected EVR1, got {0}")]
    InvalidMagic(String),
    #[error("Unsupported format version: {0}")]
    UnsupportedVersion(u32),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}
