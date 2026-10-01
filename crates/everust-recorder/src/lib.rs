pub mod ring_buffer;
pub mod writer;

pub use ring_buffer::RollingBuffer;
pub use writer::{read_recording, write_recording, RecorderError};

#[cfg(test)]
mod tests {
    use super::*;
    use everust_core::event::EventType;

    #[test]
    fn test_rolling_buffer_wrapping() {
        let buffer = RollingBuffer::new(3);
        buffer.push(1, EventType::TaskWake { task_id: 101 });
        buffer.push(1, EventType::TaskWake { task_id: 102 });
        buffer.push(1, EventType::TaskWake { task_id: 103 });
        buffer.push(1, EventType::TaskWake { task_id: 104 }); // Wraps

        let snapshot = buffer.snapshot();
        assert_eq!(snapshot.len(), 3);
        assert_eq!(
            snapshot[0].event,
            EventType::TaskWake { task_id: 102 }
        );
        assert_eq!(
            snapshot[2].event,
            EventType::TaskWake { task_id: 104 }
        );
    }
}
