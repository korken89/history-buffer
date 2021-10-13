use std::time;
use std::{fmt, iter::DoubleEndedIterator, mem, vec::Vec};

/// A "history buffer", similar to a write-only ring buffer of fixed length.
///
/// This buffer keeps a fixed number of elements. On write, the oldest element is overwritten.
/// Thus, the buffer is useful to keep a history of values with some desired depth.
pub struct HistoryBuffer<T> {
    max_size: usize,
    write_index: usize,
    buffer: Vec<T>,
    last_data_at: time::Instant,
}

impl<T> HistoryBuffer<T> {
    /// Create a new buffer with a specified max depth.
    pub fn new(max_size: usize) -> Self {
        HistoryBuffer {
            max_size,
            write_index: 0,
            buffer: Vec::with_capacity(max_size),
            last_data_at: time::Instant::now(),
        }
    }

    /// Checks if the buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.buffer.len() == 0
    }

    /// Checks if the buffer is full, i.e. it has reached capacity.
    pub fn is_full(&self) -> bool {
        self.buffer.len() == self.max_size
    }

    /// The maximum number of elements the buffer can hold.
    pub fn max_len(&self) -> usize {
        self.max_size
    }

    /// The number of elements currently in the buffer.
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    /// Clear all values in the buffer.
    pub fn clear(&mut self) {
        self.write_index = 0;
        self.buffer.clear();
    }

    /// Get the most recent value written to the buffer.
    pub fn most_recent(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            if self.write_index == 0 {
                self.buffer.last()
            } else {
                Some(&self.buffer[self.write_index - 1])
            }
        }
    }

    /// Write a new value into the buffer, if it overwrites an old value it is returned.
    pub fn write(&mut self, val: T) -> Option<T> {
        let r = if self.is_full() {
            Some(mem::replace(&mut self.buffer[self.write_index], val))
        } else {
            self.buffer.push(val);

            None
        };

        self.write_index = (self.write_index.wrapping_add(1)) % self.max_size;
        self.last_data_at = time::Instant::now();

        r
    }

    /// How long was it since the last measurement.
    pub fn duration_since_last_measurement(&self) -> Option<time::Duration> {
        if !self.is_empty() {
            let now = time::Instant::now();
            Some(now.duration_since(self.last_data_at))
        } else {
            None
        }
    }

    /// Get the entire buffer as unsorted.
    pub fn all_unsorted(&self) -> &[T] {
        &self.buffer
    }

    /// Get the entire in chronological order, starting with the oldest element.
    pub fn all(&self) -> impl DoubleEndedIterator<Item = &T> {
        let write_index = self.write_index;
        self.buffer[write_index..]
            .iter()
            .chain(self.buffer[..write_index].iter())
    }
}

impl<T> fmt::Debug for HistoryBuffer<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.debug_list().entries(self.all()).finish()
    }
}
