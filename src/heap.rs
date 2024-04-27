macro_rules! left_node {
  ($idx:expr) => {{
    let i: usize = $idx;
    ((i + 1_usize) * 2_usize) - 1_usize
  }};
}

macro_rules! right_node {
  ($idx:expr) => {{
    let i: usize = $idx;
    (i + 1_usize) * 2_usize
  }};
}

macro_rules! parent {
  ($idx:expr) => {{
    let i: usize = $idx;
    ((i + 1_usize) / 2_usize).checked_sub(1).unwrap_or(0)
  }};
}

pub fn max_heapify<T>(input: &mut [T])
where
  T: PartialOrd,
{
  let length = input.len();

  if length < 2 {
    return;
  }

  for idx in (0..length).step_by(2).rev() {
    let p_idx = parent!(idx);
    let l_idx = left_node!(p_idx);
    let r_idx = right_node!(p_idx);

    let largest = {
      if l_idx < length && input.get(p_idx).unwrap().lt(input.get(l_idx).unwrap()) {
        l_idx
      } else if r_idx < length && input.get(p_idx).unwrap().lt(input.get(r_idx).unwrap()) {
        r_idx
      } else {
        p_idx
      }
    };

    if largest != p_idx {
      input.swap(p_idx, largest);
      max_heapify(&mut input[largest..]);
    }
  }
}

pub fn min_heapify<T>(input: &mut [T])
where
  T: PartialOrd,
{
  let length = input.len();

  if length < 2 {
    return;
  }

  for idx in (0..length).step_by(2).rev() {
    let p_idx = parent!(idx);
    let l_idx = left_node!(p_idx);
    let r_idx = right_node!(p_idx);

    let smallest = {
      if l_idx < length && input.get(p_idx).unwrap().gt(input.get(l_idx).unwrap()) {
        l_idx
      } else if r_idx < length && input.get(p_idx).unwrap().gt(input.get(r_idx).unwrap()) {
        r_idx
      } else {
        p_idx
      }
    };

    if smallest != p_idx {
      input.swap(p_idx, smallest);
      min_heapify(&mut input[smallest..]);
    }
  }
}

// Builds heap structure from n to 0, instead of 0 to n.j
// It may look similar to min/max_heapify but it's guarantees
// the max/min value is the last element.

pub fn max_heapify_rev<T>(input: &mut [T])
where
  T: PartialOrd,
{
  let length = input.len();

  if length < 2 {
    return;
  }

  let max_idx = length - 1;

  for idx in (0..length).step_by(2).rev() {
    let p_idx = parent!(idx);
    let l_idx = max_idx.checked_sub(left_node!(p_idx));
    let r_idx = max_idx.checked_sub(right_node!(p_idx));
    let p_idx = max_idx - p_idx;

    let largest = {
      if l_idx.is_some_and(|v| v < length && input.get(p_idx).unwrap().lt(input.get(v).unwrap())) {
        l_idx.unwrap()
      } else if r_idx
        .is_some_and(|v| v < length && input.get(p_idx).unwrap().lt(input.get(v).unwrap()))
      {
        r_idx.unwrap()
      } else {
        p_idx
      }
    };

    if largest != p_idx {
      input.swap(p_idx, largest);
      max_heapify_rev(&mut input[..=largest]);
    }
  }
}

pub fn min_heapify_rev<T>(input: &mut [T])
where
  T: PartialOrd,
{
  let length = input.len();

  if length < 2 {
    return;
  }

  let max_idx = length - 1;

  for idx in (0..length).step_by(2).rev() {
    let p_idx = parent!(idx);
    let l_idx = max_idx.checked_sub(left_node!(p_idx));
    let r_idx = max_idx.checked_sub(right_node!(p_idx));
    let p_idx = max_idx - p_idx;

    let smallest = {
      if l_idx.is_some_and(|v| v < length && input.get(p_idx).unwrap().gt(input.get(v).unwrap())) {
        l_idx.unwrap()
      } else if r_idx
        .is_some_and(|v| v < length && input.get(p_idx).unwrap().gt(input.get(v).unwrap()))
      {
        r_idx.unwrap()
      } else {
        p_idx
      }
    };

    if smallest != p_idx {
      input.swap(p_idx, smallest);
      min_heapify_rev(&mut input[..=smallest]);
    }
  }
}
