// TODO: Current version will fail to compile without sse4.2 feature enabled
// Add software and ARM fallbacks https://developer.arm.com/documentation/dui0801/g/A64-General-Instructions/CRC32CB--CRC32CH--CRC32CW--CRC32CX

macro_rules! read {
  ($ptr:expr, $offset:expr) => {
    unsafe { $ptr.byte_add($offset).read_unaligned() }
  };

  ($ptr:expr) => {
    unsafe { $ptr.read_unaligned() }
  };
}

#[inline(always)]
pub fn crc32c_u64_step(crc: u32, value: u64) -> u32 {
  {
    unsafe { std::arch::x86_64::_mm_crc32_u64(crc as u64, value) as u32 }
  }
}

#[inline(always)]
pub fn crc32c_u32_step(crc: u32, value: u32) -> u32 {
  {
    unsafe { std::arch::x86_64::_mm_crc32_u32(crc, value) }
  }
}

#[inline(always)]
pub fn crc32c_u16_step(crc: u32, value: u16) -> u32 {
  {
    unsafe { std::arch::x86_64::_mm_crc32_u16(crc, value) }
  }
}

#[inline(always)]
pub fn crc32c_u8_step(crc: u32, value: u8) -> u32 {
  {
    unsafe { std::arch::x86_64::_mm_crc32_u8(crc, value) }
  }
}

pub fn crc32c(input: &[u8]) -> u32 {
  let mut crc: u32 = 0xFFFFFFFF;
  let mut offset: usize = 0;

  let len: usize = input.len() / 8;
  let input_ptr: *const u64 = input.as_ptr().cast();

  for _i in 0..len {
    crc = crc32c_u64_step(crc, read!(input_ptr, offset));
    offset += 8;
  }

  let mut tail_len = input.len() & 7;
  let mut tail_ptr: *const u8 = unsafe { input.as_ptr().byte_add(offset) };

  if tail_len >= 4 {
    let tail_u32: *const u32 = tail_ptr.cast();
    crc = crc32c_u32_step(crc, read!(tail_u32));

    tail_len -= 4;
    tail_ptr = unsafe { tail_ptr.byte_add(4) };
  }

  if tail_len >= 2 {
    let tail_u16: *const u16 = tail_ptr.cast();
    crc = crc32c_u16_step(crc, read!(tail_u16));

    tail_len -= 2;
    tail_ptr = unsafe { tail_ptr.byte_add(2) };
  }

  if tail_len > 0 {
    for idx in 0..tail_len {
      crc = crc32c_u8_step(crc, read!(tail_ptr, idx));
    }
  }

  crc ^ 0xFFFFFFFF
}
