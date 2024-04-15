pub fn sort<T>(input: &mut [T])
where
  T: Ord + Clone,
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
  T: Ord + Clone,
{
  let mut rhs_idx: usize = split_offset;
  let mut lhs_idx: usize = 0;

  // Not a perfect in-place merge but trying to reduce memory allocation with cloning the only
  // left-hand side as auxillary buffer and swapping right-hand side.
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
        *(buf.get_mut(idx).unwrap()) = val.clone();
        lhs_idx += 1;
      }
      (Some(left_val), Some(right_val)) => {
        if let std::cmp::Ordering::Less = right_val.cmp(left_val) {
          buf.swap(idx, rhs_idx);
          rhs_idx += 1;
        } else {
          *(buf.get_mut(idx).unwrap()) = left_val.clone();
          lhs_idx += 1;
        }
      }
    }
  }
}

mod test {
  #[test]
  fn sort_numbers() {
    let mut arr = [5, 2, 4, 6, 1, 3];
    crate::sorting::merge_sort::sort(&mut arr);

    assert_eq!([1, 2, 3, 4, 5, 6], arr)
  }

  #[test]
  fn sort_strings() {
    let mut arr = ["zb", "zc", "dc", "ef", "_", "a"];
    crate::sorting::merge_sort::sort(&mut arr);

    assert_eq!(["_", "a", "dc", "ef", "zb", "zc"], arr)
  }

  #[test]
  fn sort_structs() {
    #[derive(Eq, PartialEq, PartialOrd, Debug, Clone)]
    struct TestStruct {
      value: i32,
    }

    impl Ord for TestStruct {
      fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.value == other.value {
          std::cmp::Ordering::Equal
        } else if self.value > other.value {
          std::cmp::Ordering::Greater
        } else {
          std::cmp::Ordering::Less
        }
      }
    }

    let mut arr = [
      TestStruct { value: 12 },
      TestStruct { value: 13 },
      TestStruct { value: 0 },
      TestStruct { value: -1 },
    ];
    crate::sorting::merge_sort::sort(&mut arr);

    assert_eq!([-1, 0, 12, 13], arr.map(|e| e.value))
  }

  #[test]
  fn sort_vec() {
    let mut arr = vec![5, 2, 4, 6, 1, 3];
    crate::sorting::merge_sort::sort(&mut arr);

    assert_eq!(&[1, 2, 3, 4, 5, 6], arr.as_slice())
  }

  #[test]
  fn sort_empty() {
    let arr: &mut [i32] = &mut [];
    crate::sorting::merge_sort::sort(arr);

    assert!(arr.is_empty())
  }

  #[test]
  fn sort_single() {
    let mut arr = [3];
    crate::sorting::merge_sort::sort(&mut arr);

    assert_eq!([3], arr)
  }
}
