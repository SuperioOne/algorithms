use algorithms::queues::Queue;

#[test]
fn pop() {
  let inputs = [-1, 0, 65, 4, 32, 3, 90, -87];
  let mut process_order: Vec<i32> = vec![];
  let mut task_queue = algorithms::queues::MaxPriorityQueue::from_iter(inputs.into_iter());

  while let Some(task) = task_queue.pop() {
    process_order.push(task);
  }

  assert_eq!(&[90, 65, 32, 4, 3, 0, -1, -87], process_order.as_slice())
}

#[test]
fn insert() {
  let inputs = [-1, 0, 65, 4, 32, 3, 90, -87];
  let mut task_queue = algorithms::queues::MaxPriorityQueue::new();

  for i in inputs {
    _ = task_queue.push(i);
  }

  assert_eq!(inputs.len(), task_queue.len());
  assert_eq!(Some(&90), task_queue.peek());
}

#[test]
fn remove() {
  let inputs = [-1, 0, 65, 4, 32, 3, 90, -87];
  let mut task_queue = algorithms::queues::MaxPriorityQueue::from_iter(inputs.into_iter());
  let mut i: Option<usize> = None;

  for (idx, v) in task_queue.iter().enumerate() {
    if *v == 0 {
      i = Some(idx);
      break;
    }
  }

  if let Some(pos) = i {
    assert_eq!(Some(0), task_queue.remove(pos));
    assert_eq!(None, task_queue.remove(999));
    assert_eq!(inputs.len() - 1, task_queue.len());
    assert_eq!(Some(&90), task_queue.peek());
  } else {
    assert!(false);
  }
}
