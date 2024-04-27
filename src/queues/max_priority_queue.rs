use std::slice::Iter;

use crate::heap::max_heapify_rev;

use super::errors::QueueError;
use super::Queue;

#[derive(Debug)]
pub struct MaxPriorityQueue<T> {
  inner_buf: Vec<T>,
}

impl<T> MaxPriorityQueue<T>
where
  T: PartialOrd,
{
  pub fn new() -> Self {
    Self {
      inner_buf: Vec::new(),
    }
  }

  pub fn wit_capacity(capacity: usize) -> Self {
    Self {
      inner_buf: Vec::with_capacity(capacity),
    }
  }

  pub fn iter(&self) -> Iter<T> {
    self.inner_buf.iter()
  }
}

impl<T> FromIterator<T> for MaxPriorityQueue<T>
where
  T: PartialOrd,
{
  fn from_iter<A: IntoIterator<Item = T>>(iter: A) -> Self {
    let mut vec: Vec<T> = Vec::from_iter(iter);
    max_heapify_rev(&mut vec);
    Self { inner_buf: vec }
  }
}

impl<T> Default for MaxPriorityQueue<T>
where
  T: PartialOrd,
{
  fn default() -> Self {
    Self::new()
  }
}

impl<T> IntoIterator for MaxPriorityQueue<T>
where
  T: PartialOrd,
{
  type Item = T;
  type IntoIter = std::vec::IntoIter<T>;

  fn into_iter(self) -> Self::IntoIter {
    self.inner_buf.into_iter()
  }
}

impl<T> Clone for MaxPriorityQueue<T>
where
  T: PartialOrd + Clone,
{
  fn clone(&self) -> Self {
    Self {
      inner_buf: self.inner_buf.clone(),
    }
  }
}

impl<T> Queue<T> for MaxPriorityQueue<T>
where
  T: PartialOrd,
{
  fn push(&mut self, value: T) -> Result<(), QueueError> {
    let rebalance = self.inner_buf.last().is_some_and(|v| v.gt(&value));

    self.inner_buf.push(value);

    if rebalance {
      max_heapify_rev(&mut self.inner_buf);
    }

    Ok(())
  }

  fn pop(&mut self) -> Option<T> {
    match self.inner_buf.pop() {
      value @ Some(_) => {
        if self.inner_buf.len() > 1 {
          let l_child: &T = self.inner_buf.get(self.inner_buf.len() - 1).unwrap();
          let r_child: &T = self.inner_buf.get(self.inner_buf.len() - 2).unwrap();

          if l_child.lt(r_child) {
            max_heapify_rev(&mut self.inner_buf);
          }
        }

        value
      }
      None => None,
    }
  }

  fn peek(&self) -> Option<&T> {
    self.inner_buf.last()
  }

  fn remove(&mut self, idx: usize) -> Option<T> {
    if self.inner_buf.is_empty() || idx >= self.inner_buf.len() {
      None
    } else {
      let item = self.inner_buf.remove(idx);

      max_heapify_rev(&mut self.inner_buf);

      Some(item)
    }
  }

  fn is_empty(&self) -> bool {
    self.inner_buf.is_empty()
  }

  fn len(&self) -> usize {
    self.inner_buf.len()
  }
}
