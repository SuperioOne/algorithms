pub fn sort<T>(input: &mut [T])
where
  T: PartialOrd,
{
  if input.len() < 2 {
    return;
  }

  for j in 1..input.len() {
    let mut i: usize = j - 1;

    // Interestingly, rust compiler (1.77) generates better assembly code with safe `.get().unwrap()`, if
    // code already has a bound check logic (see line 9).
    while input.get(i).unwrap().gt(input.get(i + 1).unwrap()) {
      input.swap(i, i + 1);

      match i.checked_sub(1) {
        Some(value) => {
          i = value;
        }
        None => {
          break;
        }
      }
    }
  }
}
