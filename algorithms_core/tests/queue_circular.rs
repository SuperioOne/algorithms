use algorithms_core::queues::Queue;

#[test]
fn allocate_from_iter() {
  let inputs = [-1, 0, 65, 4, 32, 3, 90, -87];
  let queue = algorithms_core::queues::CircularQueue::from_iter(inputs.into_iter());

  assert_eq!(inputs.len(), queue.len());
  assert_eq!(Some(&-1), queue.peek());
}

#[test]
fn resize() {
  let inputs = [-1, 0, 65, 4, 32, 3, 90, -87];
  let mut queue = algorithms_core::queues::CircularQueue::from_iter(inputs.into_iter());

  if let Err(_) = queue.resize(10) {
    assert!(false);
  }

  assert_eq!(10, queue.capacity());
  assert_eq!(inputs.len(), queue.len());
  assert_eq!(Some(&-1), queue.peek());
}

#[test]
fn invalid_resize() {
  let inputs = [-1, 0, 65, 4, 32, 3, 90, -87];
  let mut queue = algorithms_core::queues::CircularQueue::from_iter(inputs.into_iter());

  if let Err(_e) = queue.resize(3) {
    assert!(true);
  }

  assert_eq!(inputs.len(), queue.capacity());
  assert_eq!(inputs.len(), queue.len());
  assert_eq!(Some(&-1), queue.peek());
}

#[test]
fn same_size_resize() {
  let inputs = [-1, 0, 65, 4, 32, 3, 90, -87];
  let mut queue = algorithms_core::queues::CircularQueue::from_iter(inputs.into_iter());

  assert!(queue.resize(queue.capacity()).is_ok());
  assert_eq!(inputs.len(), queue.capacity());
  assert_eq!(inputs.len(), queue.len());
  assert_eq!(Some(&-1), queue.peek());
}

#[test]
fn trim() {
  let inputs = [-1, 0, 65, 4, 32, 3, 90, -87];
  let mut queue = algorithms_core::queues::CircularQueue::from_iter(inputs.into_iter());

  for _i in 0..3 {
    if let None = queue.pop() {
      assert!(false);
    }
  }

  assert!(queue.trim().is_ok());
  assert_eq!(inputs.len() - 3, queue.capacity());
  assert_eq!(inputs.len() - 3, queue.len());
  assert_eq!(Some(&4), queue.peek());
}

#[test]
fn resize_tail_lt_head() {
  let inputs = [-5, -12, -65, -87];
  let mut queue = algorithms_core::queues::CircularQueue::from_iter(inputs.into_iter());

  // Shifts head to mid section before resize
  let mut ctr = 1;
  for _j in 0..2 {
    for _i in 0..3 {
      if let None = queue.pop() {
        assert!(false);
      }
      if let Err(_e) = queue.push(ctr) {
        assert!(false);
      }

      ctr += 1;
    }
  }

  assert!(queue.resize(6).is_ok());
  assert_eq!(6, queue.capacity());
  assert_eq!(inputs.len(), queue.len());
  assert_eq!(Some(&3), queue.peek());
}

#[test]
fn remove_from_mid_left() {
  let inputs = [-1, 0, 65, 4, 32, 3, 90, -87, 26, 43, 23, 32, 6, 7];
  let mut queue = algorithms_core::queues::CircularQueue::<i32>::from_iter(inputs.into_iter());

  assert_eq!(Some(65), queue.remove(2));
  assert_eq!(None, queue.remove(999));
  assert_eq!(inputs.len() - 1, queue.len());
  assert_eq!(Some(&-1), queue.peek());
}

#[test]
fn remove_from_mid_right() {
  let inputs = [-1, 0, 65, 4, 32, 3, 90, -87, 26, 43, 23, 32, 6, 7];
  let mut queue = algorithms_core::queues::CircularQueue::<i32>::from_iter(inputs.into_iter());

  assert_eq!(Some(43), queue.remove(9));
  assert_eq!(None, queue.remove(999));
  assert_eq!(inputs.len() - 1, queue.len());
  assert_eq!(Some(&-1), queue.peek());
}

#[test]
fn remove_first() {
  let inputs = [-1, 0, 65, 4, 32, 3, 90, -87, 26, 43, 23, 32, 6, 7];
  let mut queue = algorithms_core::queues::CircularQueue::<i32>::from_iter(inputs.into_iter());

  assert_eq!(Some(-1), queue.remove(0));
  assert_eq!(None, queue.remove(999));
  assert_eq!(inputs.len() - 1, queue.len());
  assert_eq!(Some(&0), queue.peek());
}

#[test]
fn remove_last() {
  let inputs = [-1, 0, 65, 4, 32, 3, 90, -87, 26, 43, 23, 32, 6, 7];
  let mut queue = algorithms_core::queues::CircularQueue::<i32>::from_iter(inputs.into_iter());

  assert_eq!(Some(7), queue.remove(inputs.len() - 1));
  assert_eq!(None, queue.remove(999));
  assert_eq!(inputs.len() - 1, queue.len());
  assert_eq!(Some(&-1), queue.peek());
}

#[test]
fn get() {
  let inputs = [-1, 0, 65, 4, 32, 3, 90, -87, 26, 43, 23, 32, 6, 7];
  let queue = algorithms_core::queues::CircularQueue::<i32>::from_iter(inputs.into_iter());

  assert_eq!(Some(&7), queue.get(inputs.len() - 1));
  assert_eq!(Some(&-1), queue.get(0));
  assert_eq!(Some(&0), queue.get(1));
  assert_eq!(None, queue.get(999));
}

#[test]
fn get_mut() {
  let inputs = [-1, 0, 65, 4, 32, 3, 90, -87, 26, 43, 23, 32, 6, 7];
  let mut queue = algorithms_core::queues::CircularQueue::<i32>::from_iter(inputs.into_iter());

  if let Some(last) = queue.get_mut(inputs.len() - 1) {
    *last += 1;
  } else {
    assert!(false);
  }

  if let Some(first) = queue.get_mut(0) {
    *first = 42;
  } else {
    assert!(false);
  }

  assert_eq!(None, queue.get_mut(999));
  assert_eq!(Some(&8), queue.get(inputs.len() - 1));
  assert_eq!(Some(&42), queue.get(0));
  assert_eq!(Some(&42), queue.peek());
}

#[test]
pub fn iter() {
  let inputs = [-1, 0, 65, 4, 32, 3, 90, -87, 26, 43, 23, 32, 6, 7];
  let mut queue = algorithms_core::queues::CircularQueue::<i32>::from_iter(inputs.into_iter());

  _ = queue.pop();
  _ = queue.pop();
  _ = queue.pop();

  assert_eq!(inputs.iter().skip(3).sum::<i32>(), queue.iter().sum());
}

#[test]
fn two_cyclic_push() {
  let inputs = [-1, 0, 65, 4, 32, 3, 90, -87, 26, 43, 23, 32, 6, 7];
  let mut queue = algorithms_core::queues::CircularQueue::<i32>::new(inputs.len() / 2);
  let offset = inputs.len() / 2;

  for i in inputs.iter().take(inputs.len() / 2) {
    if let Err(_e) = queue.push(*i) {
      assert!(false);
    }
  }

  assert_eq!(Some(&-1), queue.peek());

  for i in 0..queue.len() {
    if let Some(val) = queue.pop() {
      assert_eq!(*inputs.get(i).unwrap(), val);
    } else {
      assert!(false);
    }
  }

  assert_eq!(0, queue.len());
  assert_eq!(inputs.len() / 2, queue.capacity());

  for i in inputs.iter().skip(inputs.len() / 2) {
    if let Err(_e) = queue.push(*i) {
      assert!(false);
    }
  }

  for i in 0..queue.len() {
    if let Some(val) = queue.pop() {
      assert_eq!(*inputs.get(i + offset).unwrap(), val);
    } else {
      assert!(false);
    }
  }

  assert!(queue.is_empty());
  assert_eq!(0, queue.len());
}
