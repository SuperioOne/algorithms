use std::ops::Add;

#[derive(PartialEq, Copy, Clone, Debug)]
struct Vec2D(f64, f64);

impl Vec2D {
  pub fn length(&self) -> f64 {
    (self.0.powi(2) + self.1.powi(2)).sqrt()
  }
}

impl Default for Vec2D {
  fn default() -> Self {
    Self(0.0, 0.0)
  }
}

impl Add for Vec2D {
  type Output = Vec2D;

  fn add(self, rhs: Self) -> Self::Output {
    Vec2D(self.0 + rhs.0, self.1 + rhs.1)
  }
}

impl PartialOrd for Vec2D {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.length().total_cmp(&other.length()))
  }
}

#[test]
fn even_n_basic() {
  let arr = [-1, 3, 7, 0];
  let result = algorithms::max_sub_array::get_max_subarray(&arr);

  assert_eq!([3, 7], result)
}

#[test]
fn odd_n_basic() {
  let arr = [-1, 3, 7, -2, 12];
  let result = algorithms::max_sub_array::get_max_subarray(&arr);

  assert_eq!([3, 7, -2, 12], result)
}

#[test]
fn all_negative() {
  let arr = [-1, -3, -7, -5, -12];
  let result = algorithms::max_sub_array::get_max_subarray(&arr);

  assert_eq!([-1], result)
}

#[test]
fn empty_array() {
  let arr: &[i32] = &[];
  let result = algorithms::max_sub_array::get_max_subarray(arr);

  assert_eq!(arr, result)
}

#[test]
fn complex_types() {
  let arr: Vec<Vec2D> = vec![
    Vec2D(1.0, 2.0),
    Vec2D(-1.0, -2.0),
    Vec2D(3.0, 1.0),
    Vec2D(-2.0, 4.0),
    Vec2D(1.0, 1.0),
  ];
  let result = algorithms::max_sub_array::get_max_subarray(&arr);

  assert_eq!(&arr[2..], result)
}
