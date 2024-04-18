use crate::min_heapify;

pub fn sort<T>(input: &mut [T])
where
  T: PartialOrd,
{
  if input.len() < 2 {
    return;
  }

  for idx in 0..(input.len() - 1) {
    min_heapify(&mut input[idx..]);
  }
}
