use crate::random::NumberGenerator;

pub fn sort<T>(input: &mut [T])
where
  T: PartialOrd,
{
  if input.len() < 2 {
    return;
  }

  let s_idx = partition(input);

  if s_idx == 0 {
    sort(input);
  } else if s_idx > 0 && s_idx < (input.len() - 1) {
    sort(&mut input[..=s_idx]);
    sort(&mut input[s_idx + 1..]);
  } else {
    sort(&mut input[..s_idx]);
  }
}

#[inline]
fn partition<T>(input: &mut [T]) -> usize
where
  T: PartialOrd,
{
  let mut s_idx: usize = 0;
  let length = input.len() - 1;

  unsafe {
    let pivot: *const T = input.last().unwrap();

    for idx in 0..length {
      if input.get(idx).unwrap().le(&*pivot) {
        if idx != s_idx {
          input.swap(idx, s_idx);
        }

        s_idx += 1;
      }
    }
  }

  input.swap(s_idx, input.len() - 1);

  s_idx
}

pub fn sort_randomized<T>(input: &mut [T], generator: &mut impl NumberGenerator<usize>)
where
  T: PartialOrd,
{
  if input.len() < 2 {
    return;
  }

  let pivot = generator.get_next() % input.len();

  if pivot != (input.len() - 1) {
    input.swap(pivot, input.len() - 1);
  }

  let s_idx = partition(input);

  if s_idx == 0 {
    sort_randomized(input, generator);
  } else if s_idx > 0 && s_idx < (input.len() - 1) {
    sort_randomized(&mut input[..=s_idx], generator);
    sort_randomized(&mut input[s_idx + 1..], generator);
  } else {
    sort_randomized(&mut input[..s_idx], generator);
  }
}
