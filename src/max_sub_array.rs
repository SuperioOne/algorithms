use std::ops::Add;

pub fn get_max_subarray<'a, T>(input: &'a [T]) -> &'a [T]
where
  T: Add<Output = T> + PartialOrd + Copy + Default,
{
  if input.len() < 2 {
    return input;
  }

  let bounds = find_bounds(input, 0, input.len() - 1);

  &input[bounds.start..=bounds.end]
}

struct SubArrayBounds<T: Add<Output = T> + PartialOrd + Copy + Default> {
  start: usize,
  end: usize,
  sum: T,
}

fn find_bounds<T>(input: &[T], start: usize, end: usize) -> SubArrayBounds<T>
where
  T: Add<Output = T> + PartialOrd + Copy + Default,
{
  if start == end {
    return SubArrayBounds {
      start,
      end,
      sum: *input.get(start).unwrap(),
    };
  }

  let mid = (start + end) / 2;
  let left_bounds = find_bounds(input, start, mid);
  let right_bounds = find_bounds(input, mid + 1, end);
  let cross_bounds = find_cross_bounds(input, start, mid, end);

  if left_bounds.sum >= right_bounds.sum && left_bounds.sum >= cross_bounds.sum {
    left_bounds
  } else if right_bounds.sum >= left_bounds.sum && right_bounds.sum >= cross_bounds.sum {
    right_bounds
  } else {
    cross_bounds
  }
}

fn find_cross_bounds<T>(input: &[T], low: usize, mid: usize, high: usize) -> SubArrayBounds<T>
where
  T: Add<Output = T> + PartialOrd + Copy + Default,
{
  let mut left_sum: Option<T> = None;
  let mut right_sum: Option<T> = None;
  let mut sum: T = T::default();
  let mut left_max: usize = 0;
  let mut right_max: usize = 0;

  for i in (low..=mid).rev() {
    sum = sum + (*input.get(i).unwrap());
    if left_sum.is_none() || left_sum.is_some_and(|v| sum > v) {
      left_sum = Some(sum);
      left_max = i;
    }
  }

  sum = T::default();
  for i in (mid + 1)..=high {
    sum = sum + (*input.get(i).unwrap());
    if right_sum.is_none() || right_sum.is_some_and(|v| sum > v) {
      right_sum = Some(sum);
      right_max = i;
    }
  }

  SubArrayBounds {
    start: left_max,
    end: right_max,
    sum: left_sum.unwrap_or_default() + right_sum.unwrap_or_default(),
  }
}
