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

