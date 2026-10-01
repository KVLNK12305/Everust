use everust_core::record::FlightRecording;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

#[derive(thiserror::Error, Debug)]
pub enum RecorderError {
    #[error("I/O error during flight recording operation: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization / format error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Invalid magic in .evr file: expected EVR1, found {0}")]
    InvalidMagic(String),
}

/// Serializes and writes a `FlightRecording` to a `.evr` file.
pub fn write_recording<P: AsRef<Path>>(
    path: P,
    recording: &FlightRecording,
) -> Result<(), RecorderError> {
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, recording)?;
    Ok(())
}

/// Reads and parses an `.evr` flight recording file.
pub fn read_recording<P: AsRef<Path>>(path: P) -> Result<FlightRecording, RecorderError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let recording: FlightRecording = serde_json::from_reader(reader)?;
    if recording.header.magic != "EVR1" {
        return Err(RecorderError::InvalidMagic(recording.header.magic));
    }
    Ok(recording)
}
