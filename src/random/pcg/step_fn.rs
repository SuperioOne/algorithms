const PCG_M_8: u8 = 141;
const PCG_M_16: u16 = 12829;
const PCG_M_32: u32 = 747796405;
const PCG_M_64: u64 = 6364136223846793005;
const PCG_M_128: u128 = (2549297995355413924_u128 << 64) + 4865540595714422341_u128;
const PCG_C_8: u8 = 77;
const PCG_C_16: u16 = 47989;
const PCG_C_32: u32 = 2891336453;
const PCG_C_64: u64 = 1442695040888963407;
const PCG_C_128: u128 = (6364136223846793005_u128 << 64) + 1442695040888963407_u128;

#[inline]
pub fn pcg_oneseq_8_step_r(state: u8) -> u8 {
  state.wrapping_mul(PCG_M_8).wrapping_add(PCG_C_8)
}

#[inline]
pub fn pcg_oneseq_16_step_r(state: u16) -> u16 {
  state.wrapping_mul(PCG_M_16).wrapping_add(PCG_C_16)
}

#[inline]
pub fn pcg_mcg_16_step_r(state: u16) -> u16 {
  state.wrapping_mul(PCG_M_16)
}

#[inline]
pub fn pcg_oneseq_32_step_r(state: u32) -> u32 {
  state.wrapping_mul(PCG_M_32).wrapping_add(PCG_C_32)
}

#[inline]
pub fn pcg_mcg_32_step_r(state: u32) -> u32 {
  state.wrapping_mul(PCG_M_32)
}

#[inline]
pub fn pcg_oneseq_64_step_r(state: u64) -> u64 {
  state.wrapping_mul(PCG_M_64).wrapping_add(PCG_C_64)
}

#[inline]
pub fn pcg_mcg_64_step_r(state: u64) -> u64 {
  state.wrapping_mul(PCG_M_64)
}

#[inline]
pub fn pcg_oneseq_128_step_r(state: u128) -> u128 {
  state.wrapping_mul(PCG_M_128).wrapping_add(PCG_C_128)
}

#[inline]
pub fn pcg_mcg_128_step_r(state: u128) -> u128 {
  state.wrapping_mul(PCG_M_128)
}
