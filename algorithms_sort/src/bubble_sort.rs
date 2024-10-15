pub fn sort<T>(input: &mut [T])
where
  T: PartialOrd,
{
  let mut no_swap = false;

  while !no_swap {
    no_swap = true;

    for j in 1..input.len() {
      if input.get(j).unwrap().le(input.get(j - 1).unwrap()) {
        input.swap(j - 1, j);
        no_swap = false;
      }
    }
  }
}
