mod max_priority_queue {
  use algorithms::queues::PriorityQueue;

  #[test]
  fn pop_test() {
    let inputs = [-1, 0, 65, 4, 32, 3, 90, -87];
    let mut process_order: Vec<i32> = vec![];
    let mut task_queue = algorithms::queues::MaxPriorityQueue::from_iter(inputs.into_iter());

    while let Some(task) = task_queue.pop() {
      process_order.push(task);
    }

    assert_eq!(&[90, 65, 32, 4, 3, 0, -1, -87], process_order.as_slice())
  }

  #[test]
  fn insert_test() {
    let inputs = [-1, 0, 65, 4, 32, 3, 90, -87];
    let mut task_queue = algorithms::queues::MaxPriorityQueue::new();

    for i in inputs {
      task_queue.push(i);
    }

    assert_eq!(inputs.len(), task_queue.len());
    assert_eq!(Some(&90), task_queue.peek());
  }

  #[test]
  fn find() {
    let inputs = [-1, 0, 65, 4, 32, 3, 90, -87];
    let task_queue = algorithms::queues::MaxPriorityQueue::from_iter(inputs.into_iter());

    match task_queue.find(|v| *v == 0) {
      Some((_, val)) => {
        assert_eq!(0, *val)
      }
      None => assert!(false),
    }
  }

  #[test]
  fn find_and_replace_top() {
    let inputs = [-1, 0, 65, 4, 32, 3, 90, -87];
    let mut task_queue = algorithms::queues::MaxPriorityQueue::from_iter(inputs.into_iter());

    match task_queue.find(|v| *v == 90) {
      Some((id, _)) => {
        task_queue.replace(id, 91);
        assert_eq!(Some(&91), task_queue.peek());
        assert_eq!(inputs.len(), task_queue.len())
      }
      None => assert!(false),
    }
  }

  #[test]
  fn delete() {
    let inputs = [-1, 0, 65, 4, 32, 3, 90, -87];
    let mut task_queue = algorithms::queues::MaxPriorityQueue::from_iter(inputs.into_iter());

    assert_eq!(false, task_queue.delete(|v| *v == 1));
    assert_eq!(true, task_queue.delete(|v| *v == 0));
    assert_eq!(inputs.len() - 1, task_queue.len());
  }
}

mod min_priority_queue {
  use algorithms::queues::PriorityQueue;

  #[test]
  fn pop_test() {
    let inputs = [-1, 0, 65, 4, 32, 3, 90, -87];
    let mut process_order: Vec<i32> = vec![];
    let mut task_queue = algorithms::queues::MinPriorityQueue::from_iter(inputs.into_iter());

    while let Some(task) = task_queue.pop() {
      process_order.push(task);
    }

    assert_eq!(&[-87, -1, 0, 3, 4, 32, 65, 90], process_order.as_slice())
  }

  #[test]
  fn insert_test() {
    let inputs = [-1, 0, 65, 4, 32, 3, 90, -87];
    let mut task_queue = algorithms::queues::MinPriorityQueue::new();

    for i in inputs {
      task_queue.push(i);
    }

    assert_eq!(inputs.len(), task_queue.len());
    assert_eq!(Some(&-87), task_queue.peek());
  }

  #[test]
  fn find() {
    let inputs = [-1, 0, 65, 4, 32, 3, 90, -87];
    let task_queue = algorithms::queues::MinPriorityQueue::from_iter(inputs.into_iter());

    match task_queue.find(|v| *v == 0) {
      Some((_, val)) => {
        assert_eq!(0, *val)
      }
      None => assert!(false),
    }
  }

  #[test]
  fn find_and_replace_top() {
    let inputs = [-1, 0, 65, 4, 32, 3, 90, -87];
    let mut task_queue = algorithms::queues::MinPriorityQueue::from_iter(inputs.into_iter());

    match task_queue.find(|v| *v == -87) {
      Some((id, _)) => {
        task_queue.replace(id, -88);
        assert_eq!(Some(&-88), task_queue.peek());
        assert_eq!(inputs.len(), task_queue.len())
      }
      None => assert!(false),
    }
  }

  #[test]
  fn delete() {
    let inputs = [-1, 0, 65, 4, 32, 3, 90, -87];
    let mut task_queue = algorithms::queues::MinPriorityQueue::from_iter(inputs.into_iter());

    assert_eq!(false, task_queue.delete(|v| *v == 1));
    assert_eq!(true, task_queue.delete(|v| *v == 0));
    assert_eq!(inputs.len() - 1, task_queue.len());
  }
}
