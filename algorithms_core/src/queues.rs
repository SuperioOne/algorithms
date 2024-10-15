pub mod errors;

mod circular_queue;
mod max_priority_queue;
mod min_priority_queue;

pub use circular_queue::CircularQueue;
pub use max_priority_queue::MaxPriorityQueue;
pub use min_priority_queue::MinPriorityQueue;

use self::errors::QueueError;

pub trait Queue<T>: Default {
  fn push(&mut self, value: T) -> Result<(), QueueError>;
  fn pop(&mut self) -> Option<T>;
  fn peek(&self) -> Option<&T>;
  fn remove(&mut self, idx: usize) -> Option<T>;
  fn is_empty(&self) -> bool;
  fn len(&self) -> usize;
}
