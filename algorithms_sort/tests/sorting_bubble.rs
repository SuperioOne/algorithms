mod common;
use common::*;

#[test]
fn sort_numbers() {
  let (sorted, mut input) = test_array_i32!();
  algorithms_sort::bubble_sort::sort(&mut input);
  assert_eq!(sorted, input)
}

#[test]
fn sort_strings() {
  let (sorted, mut input) = test_array_string!();
  algorithms_sort::bubble_sort::sort(&mut input);
  assert_eq!(sorted, input)
}

#[test]
fn sort_structs() {
  let (sorted, mut input) = test_array_struct!();
  algorithms_sort::bubble_sort::sort(&mut input);
  assert_eq!(sorted, input)
}

#[test]
fn sort_empty() {
  let arr: &mut [i32] = &mut [];
  algorithms_sort::bubble_sort::sort(arr);
  assert!(arr.is_empty())
}

#[test]
fn sort_single() {
  let mut arr = [3];
  algorithms_sort::bubble_sort::sort(&mut arr);
  assert_eq!([3], arr)
}
