pub mod lcg;
pub mod pcg;

pub trait NumberGenerator<T>: Iterator {
  fn get_next(&mut self) -> T;
}
