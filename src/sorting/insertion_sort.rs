pub fn sort<T>(input: &mut [T])
where
  T: Ord,
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

      match i.overflowing_sub(1) {
        (_, true) => break,
        (value, false) => {
          i = value;
        }
      }
    }
  }
}

mod test {
  #[test]
  fn sort_numbers() {
    let mut arr = [5, 2, 4, 6, 1, 3];
    crate::sorting::insertion_sort::sort(&mut arr);

    assert_eq!([1, 2, 3, 4, 5, 6], arr)
  }

  #[test]
  fn sort_strings() {
    let mut arr = ["zb", "zc", "dc", "ef", "_", "a"];
    crate::sorting::insertion_sort::sort(&mut arr);

    assert_eq!(["_", "a", "dc", "ef", "zb", "zc"], arr)
  }

  #[test]
  fn sort_structs() {
    #[derive(Eq, PartialEq, PartialOrd, Debug)]
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
    crate::sorting::insertion_sort::sort(&mut arr);

    assert_eq!([-1, 0, 12, 13], arr.map(|e| e.value))
  }

  #[test]
  fn sort_vec() {
    let mut arr = vec![5, 2, 4, 6, 1, 3];
    crate::sorting::insertion_sort::sort(&mut arr);

    assert_eq!(&[1, 2, 3, 4, 5, 6], arr.as_slice())
  }

  #[test]
  fn sort_empty() {
    let arr: &mut [i32] = &mut [];
    crate::sorting::insertion_sort::sort(arr);

    assert!(arr.is_empty())
  }

  #[test]
  fn sort_single() {
    let mut arr = [3];
    crate::sorting::insertion_sort::sort(&mut arr);

    assert_eq!([3], arr)
  }
}
