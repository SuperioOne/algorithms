pub fn sort<T>(input: &mut [T])
where
  T: PartialOrd,
{
  if input.len() < 2 {
    return;
  }

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

  if s_idx == 0 {
    sort(input);
  } else if s_idx > 0 && s_idx < length {
    sort(&mut input[..=s_idx]);
    sort(&mut input[s_idx + 1..]);
  } else {
    sort(&mut input[..s_idx]);
  }
}

pub enum PivotSelection {
  CustomIndex(usize),
  Gaussian(i64),
  LogNormal(i64),
}

pub enum PivotError {
  OutOfRange,
}

// pub fn sort_randomized<T>(input: &mut [T], option: PivotSelection) -> Result<(), PivotError>
// where
//   T: PartialOrd,
// {
//   if input.len() < 2 {
//     return Ok(());
//   }
//
//   let pivot_idx = match option {
//     PivotSelection::CustomIndex(idx) => {
//       if idx > input.len() {
//         return Err(PivotError::OutOfRange);
//       } else {
//         idx
//       }
//     }
//     PivotSelection::Gaussian(seed) => {}
//     PivotSelection::LogNormal(seed) => todo!(),
//   };
//
//   if pivot_idx != input.len() - 1 {
//     input.swap(pivot_idx, input.len() - 1);
//   }
//
//   sort(input);
//
//   Ok(())
// }
