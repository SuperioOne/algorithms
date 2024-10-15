pub enum QueueError {
  InvalidResize,
  InvalidLayout,
  NotEnoughCapacity,
}

impl std::fmt::Debug for QueueError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      QueueError::InvalidResize => {
        f.write_str("Given resize value is less than current capacity. It will cause a data loss.")
      }
      QueueError::InvalidLayout => f.write_str("Created layout is invalid."),
      QueueError::NotEnoughCapacity => {
        f.write_str("Queue capacity is filled. Consider resizing it.")
      }
    }
  }
}
