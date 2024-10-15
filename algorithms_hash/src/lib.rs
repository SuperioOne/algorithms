#![no_std]

pub mod cityhash;
pub mod crc;
pub mod murmur3;
pub mod pearson;

pub trait HashFunc {
  type Output;

  fn get_hash(&self, bytes: &[u8]) -> Self::Output;
}
