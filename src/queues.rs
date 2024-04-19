mod max_priority_queue;
mod min_priority_queue;

pub use max_priority_queue::MaxPriorityQueue;
pub use min_priority_queue::MinPriorityQueue;

pub trait PriorityQueue<T>: Default
where
  T: PartialOrd,
{
  fn push(&mut self, value: T);
  fn replace(&mut self, old_idx: usize, new: T) -> bool;
  fn pop(&mut self) -> Option<T>;
  fn peek(&self) -> Option<&T>;
  fn delete<F>(&mut self, predicate: F) -> bool
  where
    F: Fn(&T) -> bool;
  fn is_empty(&self) -> bool;
  fn len(&self) -> usize;
  fn find<F>(&self, predicate: F) -> Option<(usize, &T)>
  where
    F: Fn(&T) -> bool;
}
