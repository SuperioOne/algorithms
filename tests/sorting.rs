#[derive(Eq, PartialEq, PartialOrd, Debug, Clone, Copy)]
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

macro_rules! test_array_i32 {
  () => {
    ([0, 1, 2, 3, 4, 5, 6], [5, 2, 4, 6, 1, 3, 0])
  };
}

macro_rules! test_array_string {
  () => {
    (
      ["_", "a", "dc", "ef", "zb", "zc"],
      ["zb", "zc", "dc", "ef", "_", "a"],
    )
  };
}

macro_rules! test_array_struct {
  () => {
    (
      [
        TestStruct { value: -1 },
        TestStruct { value: 0 },
        TestStruct { value: 12 },
        TestStruct { value: 13 },
      ],
      [
        TestStruct { value: 12 },
        TestStruct { value: 13 },
        TestStruct { value: 0 },
        TestStruct { value: -1 },
      ],
    )
  };
}

mod merge_sort_test {
  use crate::TestStruct;

  #[test]
  fn sort_numbers() {
    let (sorted, mut input) = test_array_i32!();
    algorithms::sorting::merge_sort::sort(&mut input);
    assert_eq!(sorted, input)
  }

  #[test]
  fn sort_strings() {
    let (sorted, mut input) = test_array_string!();
    algorithms::sorting::merge_sort::sort(&mut input);
    assert_eq!(sorted, input)
  }

  #[test]
  fn sort_structs() {
    let (sorted, mut input) = test_array_struct!();
    algorithms::sorting::merge_sort::sort(&mut input);
    assert_eq!(sorted, input)
  }

  #[test]
  fn sort_empty() {
    let arr: &mut [i32] = &mut [];
    algorithms::sorting::merge_sort::sort(arr);
    assert!(arr.is_empty())
  }

  #[test]
  fn sort_single() {
    let mut arr = [3];
    algorithms::sorting::merge_sort::sort(&mut arr);
    assert_eq!([3], arr)
  }
}

mod insertion_sort_test {
  use crate::TestStruct;

  #[test]
  fn sort_numbers() {
    let (sorted, mut input) = test_array_i32!();
    algorithms::sorting::insertion_sort::sort(&mut input);
    assert_eq!(sorted, input)
  }

  #[test]
  fn sort_strings() {
    let (sorted, mut input) = test_array_string!();
    algorithms::sorting::insertion_sort::sort(&mut input);
    assert_eq!(sorted, input)
  }

  #[test]
  fn sort_structs() {
    let (sorted, mut input) = test_array_struct!();
    algorithms::sorting::insertion_sort::sort(&mut input);
    assert_eq!(sorted, input)
  }

  #[test]
  fn sort_empty() {
    let arr: &mut [i32] = &mut [];
    algorithms::sorting::insertion_sort::sort(arr);
    assert!(arr.is_empty())
  }

  #[test]
  fn sort_single() {
    let mut arr = [3];
    algorithms::sorting::insertion_sort::sort(&mut arr);
    assert_eq!([3], arr)
  }
}

mod bubble_sort_test {
  use crate::TestStruct;

  #[test]
  fn sort_numbers() {
    let (sorted, mut input) = test_array_i32!();
    algorithms::sorting::bubble_sort::sort(&mut input);
    assert_eq!(sorted, input)
  }

  #[test]
  fn sort_strings() {
    let (sorted, mut input) = test_array_string!();
    algorithms::sorting::bubble_sort::sort(&mut input);
    assert_eq!(sorted, input)
  }

  #[test]
  fn sort_structs() {
    let (sorted, mut input) = test_array_struct!();
    algorithms::sorting::bubble_sort::sort(&mut input);
    assert_eq!(sorted, input)
  }

  #[test]
  fn sort_empty() {
    let arr: &mut [i32] = &mut [];
    algorithms::sorting::bubble_sort::sort(arr);
    assert!(arr.is_empty())
  }

  #[test]
  fn sort_single() {
    let mut arr = [3];
    algorithms::sorting::bubble_sort::sort(&mut arr);
    assert_eq!([3], arr)
  }
}

mod heap_sort_test {
  use crate::TestStruct;

  #[test]
  fn sort_numbers() {
    let (sorted, mut input) = test_array_i32!();
    algorithms::sorting::heap_sort::sort(&mut input);
    assert_eq!(sorted, input)
  }

  #[test]
  fn sort_strings() {
    let (sorted, mut input) = test_array_string!();
    algorithms::sorting::heap_sort::sort(&mut input);
    assert_eq!(sorted, input)
  }

  #[test]
  fn sort_structs() {
    let (sorted, mut input) = test_array_struct!();
    algorithms::sorting::heap_sort::sort(&mut input);
    assert_eq!(sorted, input)
  }

  #[test]
  fn sort_empty() {
    let arr: &mut [i32] = &mut [];
    algorithms::sorting::heap_sort::sort(arr);
    assert!(arr.is_empty())
  }

  #[test]
  fn sort_single() {
    let mut arr = [3];
    algorithms::sorting::heap_sort::sort(&mut arr);
    assert_eq!([3], arr)
  }
}
