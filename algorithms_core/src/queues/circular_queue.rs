use std::alloc::{alloc, dealloc, Layout};
use std::marker::PhantomData;
use std::ptr;
use std::ptr::null_mut;

use super::errors::QueueError;
use super::Queue;

#[derive(Debug)]
pub struct CircularQueue<T> {
  inner: *mut T,
  len: usize,
  head: usize,
  capacity: usize,
  phantom: PhantomData<T>,
}

pub struct CircularQueueIntoIter<T> {
  queue: CircularQueue<T>,
  counter: usize,
}

pub struct CircularQueueIter<'a, T> {
  queue: &'a CircularQueue<T>,
  counter: usize,
}

pub struct CircularQueueMutIter<'a, T> {
  queue: &'a mut CircularQueue<T>,
  counter: usize,
}

macro_rules! get_circular_idx {
  ($offset:expr, $id:expr, $capacity:expr) => {
    ($offset + $id) % $capacity
  };
}

impl<T> CircularQueue<T> {
  pub fn get(&self, idx: usize) -> Option<&T> {
    if idx >= self.len {
      None
    } else {
      let item_ptr = unsafe {
        self
          .inner
          .add(get_circular_idx!(self.head, idx, self.capacity))
      };

      if item_ptr.is_null() {
        None
      } else {
        unsafe { Some(&*item_ptr) }
      }
    }
  }

  pub fn get_mut(&mut self, idx: usize) -> Option<&mut T> {
    if idx >= self.len {
      None
    } else {
      let item_ptr = unsafe {
        self
          .inner
          .add(get_circular_idx!(self.head, idx, self.capacity))
      };

      if item_ptr.is_null() {
        None
      } else {
        unsafe { Some(&mut *item_ptr) }
      }
    }
  }

  pub fn new(size: usize) -> Self {
    let mut queue = Self {
      inner: null_mut(),
      len: 0,
      head: 0,
      capacity: 0,
      phantom: PhantomData,
    };

    if size > 0 {
      queue
        .resize(size)
        .expect("Cannot allocated a proper buffer.");
    }

    queue
  }

  pub fn iter(&self) -> CircularQueueIter<T> {
    CircularQueueIter {
      queue: self,
      counter: 0,
    }
  }

  pub fn iter_mut(&mut self) -> CircularQueueMutIter<T> {
    CircularQueueMutIter {
      queue: self,
      counter: 0,
    }
  }

  pub fn capacity(&self) -> usize {
    self.capacity
  }

  pub fn trim(&mut self) -> Result<(), QueueError> {
    self.resize(self.len)
  }

  pub fn resize(&mut self, new_capacity: usize) -> Result<(), QueueError> {
    if new_capacity < self.len {
      return Err(QueueError::InvalidResize);
    }

    if new_capacity == self.capacity {
      return Ok(());
    }

    let new_layout = Layout::array::<T>(new_capacity).map_err(|_| QueueError::InvalidLayout)?;
    let old_layout = Layout::array::<T>(self.capacity).map_err(|_| QueueError::InvalidLayout)?;
    let new_buf: *mut T = unsafe { alloc(new_layout) as *mut T };

    if !self.inner.is_null() {
      if self.len() > 0 {
        // Tail > Head
        if self.head + self.len < self.capacity {
          unsafe {
            self
              .inner
              .add(self.head)
              .copy_to_nonoverlapping(new_buf, self.len);
          }
        }
        // Tail < Head
        else {
          let right_count = self.capacity - self.head;
          let left_count = self.len - right_count;

          unsafe {
            self
              .inner
              .add(self.head)
              .copy_to_nonoverlapping(new_buf, right_count);
          }

          if left_count > 0 {
            unsafe {
              self
                .inner
                .copy_to_nonoverlapping(new_buf.add(right_count), left_count);
            }
          }
        }
      }

      unsafe {
        dealloc(self.inner as *mut u8, old_layout);
      }
    }

    // Resize operation always defragments and puts head element to the 0.
    self.head = 0;
    self.inner = new_buf;
    self.capacity = new_capacity;

    Ok(())
  }
}

impl<T> Queue<T> for CircularQueue<T> {
  fn push(&mut self, value: T) -> Result<(), QueueError> {
    if self.len == self.capacity {
      return Err(QueueError::NotEnoughCapacity);
    }

    unsafe {
      let tail = self
        .inner
        .add(get_circular_idx!(self.head, self.len, self.capacity));

      tail.write(value);
    }

    self.len += 1;

    Ok(())
  }

  fn pop(&mut self) -> Option<T> {
    if self.len < 1 {
      return None;
    }

    let head = unsafe { self.inner.add(self.head) };

    if head.is_null() {
      None
    } else {
      let result = unsafe { head.read() };

      self.len -= 1;

      if self.len > 0 {
        self.head = get_circular_idx!(self.head, 1, self.capacity);
      }

      Some(result)
    }
  }

  fn peek(&self) -> Option<&T> {
    self.get(0)
  }

  /// If you call this function occasionally to remove an item from middle of the queue, maybe use a different data structure.
  fn remove(&mut self, idx: usize) -> Option<T> {
    if self.len < 1 || idx >= self.len {
      None
    } else if idx == 0 {
      self.pop()
    } else {
      let removed_ptr = unsafe {
        self
          .inner
          .add(get_circular_idx!(self.head, idx, self.capacity))
      };

      let removed_item = unsafe { removed_ptr.read() };

      // Shift head to idx
      if idx <= (self.len / 2) {
        for i in (1..=idx).rev() {
          unsafe {
            let src = self
              .inner
              .add(get_circular_idx!(self.head, i - 1, self.capacity));

            let dst = self
              .inner
              .add(get_circular_idx!(self.head, i, self.capacity));

            src.copy_to_nonoverlapping(dst, 1);
          }
        }

        self.head = (self.head + 1) % self.capacity;
      }
      // Shift tail to idx
      else {
        for i in idx..(self.len - 1) {
          unsafe {
            let src = self
              .inner
              .add(get_circular_idx!(self.head, i + 1, self.capacity));

            let dst = self
              .inner
              .add(get_circular_idx!(self.head, i, self.capacity));

            src.copy_to_nonoverlapping(dst, 1);
          }
        }
      }

      self.len -= 1;
      Some(removed_item)
    }
  }

  fn is_empty(&self) -> bool {
    self.len < 1
  }

  fn len(&self) -> usize {
    self.len
  }
}

impl<T> Iterator for CircularQueueIntoIter<T> {
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    if self.queue.len < 1 || self.counter >= self.queue.len {
      None
    } else {
      let offset = get_circular_idx!(self.queue.head, self.counter, self.queue.capacity) as isize;
      let item = unsafe { Some(self.queue.inner.offset(offset).read()) };
      self.counter += 1;
      item
    }
  }
}

impl<'a, T> Iterator for CircularQueueIter<'a, T> {
  type Item = &'a T;

  fn next(&mut self) -> Option<Self::Item> {
    if self.queue.len < 1 || self.counter >= self.queue.len {
      None
    } else {
      let offset = get_circular_idx!(self.queue.head, self.counter, self.queue.capacity) as isize;
      let item = unsafe { Some(&*self.queue.inner.offset(offset)) };
      self.counter += 1;
      item
    }
  }
}

impl<'a, T> Iterator for CircularQueueMutIter<'a, T> {
  type Item = &'a mut T;

  fn next(&mut self) -> Option<Self::Item> {
    if self.queue.len < 1 || self.counter >= self.queue.len {
      None
    } else {
      let offset = get_circular_idx!(self.queue.head, self.counter, self.queue.capacity) as isize;
      let item = unsafe { Some(&mut *self.queue.inner.offset(offset)) };
      self.counter += 1;
      item
    }
  }
}

impl<T> IntoIterator for CircularQueue<T> {
  type Item = T;

  type IntoIter = CircularQueueIntoIter<T>;

  fn into_iter(self) -> Self::IntoIter {
    Self::IntoIter {
      counter: 0,
      queue: self,
    }
  }
}

impl<T> Default for CircularQueue<T> {
  fn default() -> Self {
    Self::new(0)
  }
}

impl<T> FromIterator<T> for CircularQueue<T> {
  fn from_iter<A: IntoIterator<Item = T>>(iter: A) -> Self {
    let temp_buf: Vec<T> = Vec::from_iter(iter);
    let mut queue = Self::new(temp_buf.len());

    unsafe {
      queue
        .inner
        .copy_from_nonoverlapping(temp_buf.as_ptr(), temp_buf.len());
    }

    queue.len = temp_buf.len();
    queue
  }
}

impl<T> Drop for CircularQueue<T> {
  fn drop(&mut self) {
    let layout = Layout::array::<T>(self.capacity).expect("Ring queue dealloc layout failed.");

    for offset in (0..self.len).map(|e| get_circular_idx!(self.head, e, self.capacity) as isize) {
      unsafe {
        let item_ptr = self.inner.offset(offset);
        ptr::drop_in_place(item_ptr);
      }
    }

    unsafe { dealloc(self.inner as *mut u8, layout) };
  }
}
