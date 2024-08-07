mod find_byte;
pub mod max_sub_array;
mod split;

pub use find_byte::fast_find;

use self::split::FastSplitIterator;

pub trait FastBufferUtils {
  fn fast_find(&self, search_byte: u8) -> Option<usize>;
  fn fast_split_by_byte<'a>(&'a self, split_byte: u8) -> FastSplitIterator<'a>;
}

impl<T> FastBufferUtils for T
where
  T: AsRef<[u8]> + ?Sized,
{
  #[inline]
  fn fast_find(&self, search_byte: u8) -> Option<usize> {
    let bytes = self.as_ref();
    find_byte::fast_find(bytes, search_byte)
  }

  fn fast_split_by_byte<'a>(&'a self, split_byte: u8) -> FastSplitIterator<'a> {
    FastSplitIterator::new(self.as_ref(), split_byte)
  }
}
