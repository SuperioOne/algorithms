use algorithms_random::pcg::PcgUsize;
mod common;
use common::*;

#[test]
fn sort_numbers() {
  let (sorted, mut input) = test_array_i32!();
  algorithms_sort::quick_sort::sort(&mut input);
  assert_eq!(sorted, input)
}

#[test]
fn sort_numbers_randomized() {
  let (sorted, mut input) = test_array_i32!();
  let mut generator: PcgUsize = PcgUsize::new(13);
  algorithms_sort::quick_sort::sort_randomized(&mut input, &mut generator);
  assert_eq!(sorted, input)
}

#[test]
fn sort_strings() {
  let (sorted, mut input) = test_array_string!();
  algorithms_sort::quick_sort::sort(&mut input);
  assert_eq!(sorted, input)
}

#[test]
fn sort_structs() {
  let (sorted, mut input) = test_array_struct!();
  algorithms_sort::quick_sort::sort(&mut input);
  assert_eq!(sorted, input)
}

#[test]
fn sort_empty() {
  let arr: &mut [i32] = &mut [];
  algorithms_sort::quick_sort::sort(arr);
  assert!(arr.is_empty())
}

#[test]
fn sort_single() {
  let mut arr = [3];
  algorithms_sort::quick_sort::sort(&mut arr);
  assert_eq!([3], arr)
}
