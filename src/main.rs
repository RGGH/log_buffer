use std::collections::VecDeque;

struct LogBuffer {
    capacity: usize,
    buffer: VecDeque<String>,
}

impl LogBuffer {
    // Create a new log buffer with a given capacity
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            buffer: VecDeque::with_capacity(capacity),
        }
    }

    // Add a log message
    pub fn add(&mut self, message: String) {
        if self.buffer.len() == self.capacity {
            self.buffer.pop_front(); // Remove the oldest message
        }
        self.buffer.push_back(message);
    }

    // Retrieve all messages
    pub fn get_logs(&self) -> Vec<String> {
        self.buffer.iter().cloned().collect()
    }
}

fn main() {
    let mut log_buffer = LogBuffer::new(5);

    log_buffer.add("First log".to_string());
    log_buffer.add("Second log".to_string());
    log_buffer.add("Third log".to_string());
    log_buffer.add("Fourth log".to_string());
    log_buffer.add("Fifth log".to_string());
    log_buffer.add("Sixth log".to_string()); // This will overwrite the first log

    for log in log_buffer.get_logs() {
        println!("{}", log);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_log_buffer() {
        let log_buffer = LogBuffer::new(3);
        assert_eq!(log_buffer.capacity, 3);
        assert!(log_buffer.buffer.is_empty());
    }

    #[test]
    fn test_add_logs() {
        let mut log_buffer = LogBuffer::new(3);
        log_buffer.add("First log".to_string());
        log_buffer.add("Second log".to_string());
        assert_eq!(log_buffer.get_logs(), vec!["First log", "Second log"]);
    }

    #[test]
    fn test_overflow_behavior() {
        let mut log_buffer = LogBuffer::new(3);
        log_buffer.add("First log".to_string());
        log_buffer.add("Second log".to_string());
        log_buffer.add("Third log".to_string());
        log_buffer.add("Fourth log".to_string()); // This should remove "First log"

        assert_eq!(
            log_buffer.get_logs(),
            vec!["Second log", "Third log", "Fourth log"]
        );
    }

    #[test]
    fn test_retrieve_empty_logs() {
        let log_buffer = LogBuffer::new(3);
        assert!(log_buffer.get_logs().is_empty());
    }

    #[test]
    fn test_exact_capacity() {
        let mut log_buffer = LogBuffer::new(3);
        log_buffer.add("First log".to_string());
        log_buffer.add("Second log".to_string());
        log_buffer.add("Third log".to_string());

        assert_eq!(
            log_buffer.get_logs(),
            vec!["First log", "Second log", "Third log"]
        );
    }

    #[test]
    fn test_exceed_capacity() {
        let mut log_buffer = LogBuffer::new(3);
        log_buffer.add("First log".to_string());
        log_buffer.add("Second log".to_string());
        log_buffer.add("Third log".to_string());
        log_buffer.add("Fourth log".to_string());
        log_buffer.add("Fifth log".to_string());

        assert_eq!(
            log_buffer.get_logs(),
            vec!["Third log", "Fourth log", "Fifth log"]
        );
    }
}

