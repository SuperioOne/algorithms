use std::slice::Iter;

use super::PriorityQueue;
use crate::reverse_min_heapify;

#[derive(Debug)]
pub struct MinPriorityQueue<T> {
  inner_buf: Vec<T>,
}

impl<T> MinPriorityQueue<T>
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

impl<T> FromIterator<T> for MinPriorityQueue<T>
where
  T: PartialOrd,
{
  fn from_iter<A: IntoIterator<Item = T>>(iter: A) -> Self {
    let mut vec: Vec<T> = Vec::from_iter(iter);
    reverse_min_heapify(&mut vec);
    Self { inner_buf: vec }
  }
}

impl<T> Default for MinPriorityQueue<T>
where
  T: PartialOrd,
{
  fn default() -> Self {
    Self::new()
  }
}

impl<T> IntoIterator for MinPriorityQueue<T>
where
  T: PartialOrd,
{
  type Item = T;
  type IntoIter = std::vec::IntoIter<T>;

  fn into_iter(self) -> Self::IntoIter {
    self.inner_buf.into_iter()
  }
}

impl<T> Clone for MinPriorityQueue<T>
where
  T: PartialOrd + Clone,
{
  fn clone(&self) -> Self {
    Self {
      inner_buf: self.inner_buf.clone(),
    }
  }
}

impl<T> PriorityQueue<T> for MinPriorityQueue<T>
where
  T: PartialOrd,
{
  fn push(&mut self, value: T) {
    let rebalance = self.inner_buf.last().is_some_and(|v| v.lt(&value));
    self.inner_buf.push(value);

    if rebalance {
      reverse_min_heapify(&mut self.inner_buf);
    }
  }

  fn replace(&mut self, old_idx: usize, new: T) -> bool {
    match self.inner_buf.get_mut(old_idx) {
      Some(val) => {
        *val = new;
        reverse_min_heapify(&mut self.inner_buf);
        true
      }

      None => false,
    }
  }

  fn pop(&mut self) -> Option<T> {
    match self.inner_buf.pop() {
      value @ Some(_) => {
        if self.inner_buf.len() > 1 {
          let l_child: &T = self.inner_buf.get(self.inner_buf.len() - 1).unwrap();
          let r_child: &T = self.inner_buf.get(self.inner_buf.len() - 2).unwrap();

          if l_child.gt(r_child) {
            reverse_min_heapify(&mut self.inner_buf);
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

  fn delete<F>(&mut self, predicate: F) -> bool
  where
    F: Fn(&T) -> bool,
  {
    if self.inner_buf.is_empty() {
      return false;
    }

    for (idx, val) in self.inner_buf.iter().enumerate() {
      if predicate(val) {
        self.inner_buf.remove(idx);
        reverse_min_heapify(&mut self.inner_buf);

        return true;
      }
    }

    false
  }

  fn find<F>(&self, predicate: F) -> Option<(usize, &T)>
  where
    F: Fn(&T) -> bool,
  {
    for (idx, value) in self.inner_buf.iter().enumerate() {
      if predicate(value) {
        return Some((idx, value));
      }
    }

    None
  }

  fn is_empty(&self) -> bool {
    self.inner_buf.is_empty()
  }

  fn len(&self) -> usize {
    self.inner_buf.len()
  }
}
