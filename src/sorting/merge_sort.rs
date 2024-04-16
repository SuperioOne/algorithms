pub fn sort<T>(input: &mut [T])
where
  T: PartialOrd + Copy,
{
  match input.len() {
    0..=1 => (),
    2 => {
      merge(input, 1);
    }
    _ => {
      let q = input.len() / 2;

      let lhs = &mut input[0..q];
      sort(lhs);

      let rhs = &mut input[q..];
      sort(rhs);

      merge(input, q);
    }
  }
}

fn merge<T>(buf: &mut [T], split_offset: usize)
where
  T: PartialOrd + Copy,
{
  let mut rhs_idx: usize = split_offset;
  let mut lhs_idx: usize = 0;

  // Not a perfect in-place merge but trying to reduce memory allocation by cloning the only
  // left-hand side to an auxillary buffer and swapping the right-hand side.
  let lhs = Vec::from(&buf[0..split_offset]);

  for idx in 0..buf.len() {
    let l = lhs.get(lhs_idx);
    let r = buf.get(rhs_idx);

    match (l, r) {
      (None, None) => return,
      (None, Some(_)) => {
        buf.swap(idx, rhs_idx);
        rhs_idx += 1;
      }
      (Some(val), None) => {
        *(buf.get_mut(idx).unwrap()) = *val;
        lhs_idx += 1;
      }
      (Some(left_val), Some(right_val)) => {
        if right_val.le(left_val) {
          buf.swap(idx, rhs_idx);
          rhs_idx += 1;
        } else {
          *(buf.get_mut(idx).unwrap()) = *left_val;
          lhs_idx += 1;
        }
      }
    }
  }
}
