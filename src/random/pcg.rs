use std::marker::PhantomData;

use crate::random::pcg::output_fn::pcg_output_xsh_rr_16_8;
use crate::random::pcg::step_fn::pcg_mcg_16_step_r;

use self::output_fn::{
  pcg_output_rxs_m_xs_128_128, pcg_output_rxs_m_xs_16_16, pcg_output_rxs_m_xs_32_32,
  pcg_output_rxs_m_xs_64_64, pcg_output_rxs_m_xs_8_8, pcg_output_xsh_rr_128_64,
  pcg_output_xsh_rr_32_16, pcg_output_xsh_rr_64_32, pcg_output_xsh_rs_128_64,
  pcg_output_xsh_rs_16_8, pcg_output_xsh_rs_32_16, pcg_output_xsh_rs_64_32,
  pcg_output_xsl_rr_128_64, pcg_output_xsl_rr_64_32,
};
use self::step_fn::{
  pcg_mcg_128_step_r, pcg_mcg_32_step_r, pcg_mcg_64_step_r, pcg_oneseq_128_step_r,
  pcg_oneseq_16_step_r, pcg_oneseq_32_step_r, pcg_oneseq_64_step_r, pcg_oneseq_8_step_r,
};

use super::NumberGenerator;

mod output_fn;
mod step_fn;

pub struct McgXshRs<TIn, TOut> {
  seed: TIn,
  _p_out: PhantomData<TOut>,
}

pub struct McgXshRr<TIn, TOut> {
  seed: TIn,
  _p_out: PhantomData<TOut>,
}

pub struct OneSeqXshRs<TIn, TOut> {
  seed: TIn,
  _p_out: PhantomData<TOut>,
}

pub struct OneSeqXshRr<TIn, TOut> {
  seed: TIn,
  _p_out: PhantomData<TOut>,
}

pub struct OneSeqRxsMXs<TIn, TOut> {
  seed: TIn,
  _p_out: PhantomData<TOut>,
}

pub struct OneSeqXslRr<TIn, TOut> {
  seed: TIn,
  _p_out: PhantomData<TOut>,
}

pub struct McgXslRr<TIn, TOut> {
  seed: TIn,
  _p_out: PhantomData<TOut>,
}

pub struct PcgUsizeWrapper {
  #[cfg(target_pointer_width = "64")]
  inner: Pcg64RxsMXs64,

  #[cfg(target_pointer_width = "32")]
  inner: Pcg64RxsMXs6,
}

impl<TIn, TOut> McgXshRs<TIn, TOut> {
  pub fn new(seed: TIn) -> Self {
    Self {
      seed,
      _p_out: PhantomData::default(),
    }
  }

  pub fn reset_seed(&mut self, new_seed: TIn) -> &mut Self {
    self.seed = new_seed;
    self
  }

  pub fn get_seed(&self) -> &TIn {
    &self.seed
  }
}

impl<TIn, TOut> McgXshRr<TIn, TOut> {
  pub fn new(seed: TIn) -> Self {
    Self {
      seed,
      _p_out: PhantomData,
    }
  }

  pub fn reset_seed(&mut self, new_seed: TIn) -> &mut Self {
    self.seed = new_seed;
    self
  }

  pub fn get_seed(&self) -> &TIn {
    &self.seed
  }
}

impl<TIn, TOut> OneSeqXshRr<TIn, TOut> {
  pub fn new(seed: TIn) -> Self {
    Self {
      seed,
      _p_out: PhantomData,
    }
  }

  pub fn reset_seed(&mut self, new_seed: TIn) -> &mut Self {
    self.seed = new_seed;
    self
  }

  pub fn get_seed(&self) -> &TIn {
    &self.seed
  }
}

impl<TIn, TOut> OneSeqXshRs<TIn, TOut> {
  pub fn new(seed: TIn) -> Self {
    Self {
      seed,
      _p_out: PhantomData,
    }
  }

  pub fn reset_seed(&mut self, new_seed: TIn) -> &mut Self {
    self.seed = new_seed;
    self
  }

  pub fn get_seed(&self) -> &TIn {
    &self.seed
  }
}

impl<TIn, TOut> OneSeqRxsMXs<TIn, TOut> {
  pub fn new(seed: TIn) -> Self {
    Self {
      seed,
      _p_out: PhantomData,
    }
  }

  pub fn reset_seed(&mut self, new_seed: TIn) -> &mut Self {
    self.seed = new_seed;
    self
  }

  pub fn get_seed(&self) -> &TIn {
    &self.seed
  }
}

impl<TIn, TOut> OneSeqXslRr<TIn, TOut> {
  pub fn new(seed: TIn) -> Self {
    Self {
      seed,
      _p_out: PhantomData,
    }
  }

  pub fn reset_seed(&mut self, new_seed: TIn) -> &mut Self {
    self.seed = new_seed;
    self
  }

  pub fn get_seed(&self) -> &TIn {
    &self.seed
  }
}

impl<TIn, TOut> McgXslRr<TIn, TOut> {
  pub fn new(seed: TIn) -> Self {
    Self {
      seed,
      _p_out: PhantomData,
    }
  }

  pub fn reset_seed(&mut self, new_seed: TIn) -> &mut Self {
    self.seed = new_seed;
    self
  }

  pub fn get_seed(&self) -> &TIn {
    &self.seed
  }
}

impl PcgUsizeWrapper {
  pub fn new(seed: usize) -> Self {
    #[cfg(target_pointer_width = "64")]
    {
      Self {
        inner: Pcg64RxsMXs64::new(seed as u64),
      }
    }

    #[cfg(target_pointer_width = "32")]
    {
      Self {
        inner: Pcg64RxsMXs32::new(seed as u32),
      }
    }
  }

  pub fn reset_seed(&mut self, new_seed: usize) -> &mut Self {
    #[cfg(target_pointer_width = "64")]
    self.inner.reset_seed(new_seed as u64);

    #[cfg(target_pointer_width = "32")]
    self.inner.reset_seed(new_seed as u32);

    self
  }

  pub fn get_seed(&self) -> usize {
    self.inner.seed as usize
  }
}

// Default traits

impl<TIn, TOut> Default for OneSeqXshRs<TIn, TOut>
where
  TIn: Default,
{
  fn default() -> Self {
    Self::new(TIn::default())
  }
}

impl<TIn, TOut> Default for OneSeqXshRr<TIn, TOut>
where
  TIn: Default,
{
  fn default() -> Self {
    Self::new(TIn::default())
  }
}

impl<TIn, TOut> Default for McgXshRs<TIn, TOut>
where
  TIn: Default,
{
  fn default() -> Self {
    Self::new(TIn::default())
  }
}

impl<TIn, TOut> Default for McgXshRr<TIn, TOut>
where
  TIn: Default,
{
  fn default() -> Self {
    Self::new(TIn::default())
  }
}

impl<TIn, TOut> Default for OneSeqRxsMXs<TIn, TOut>
where
  TIn: Default,
{
  fn default() -> Self {
    Self::new(TIn::default())
  }
}

impl<TIn, TOut> Default for OneSeqXslRr<TIn, TOut>
where
  TIn: Default,
{
  fn default() -> Self {
    Self::new(TIn::default())
  }
}

impl<TIn, TOut> Default for McgXslRr<TIn, TOut>
where
  TIn: Default,
{
  fn default() -> Self {
    Self::new(TIn::default())
  }
}

// MCG XHS RR Impls

impl NumberGenerator<u8> for McgXshRr<u16, u8> {
  fn get_next(&mut self) -> u8 {
    self.seed = pcg_mcg_16_step_r(self.seed);
    pcg_output_xsh_rr_16_8(self.seed)
  }
}

impl Iterator for McgXshRr<u16, u8> {
  type Item = u8;

  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

impl NumberGenerator<u16> for McgXshRr<u32, u16> {
  fn get_next(&mut self) -> u16 {
    self.seed = pcg_mcg_32_step_r(self.seed);
    pcg_output_xsh_rr_32_16(self.seed)
  }
}

impl Iterator for McgXshRr<u32, u16> {
  type Item = u16;

  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

impl NumberGenerator<u32> for McgXshRr<u64, u32> {
  fn get_next(&mut self) -> u32 {
    self.seed = pcg_mcg_64_step_r(self.seed);
    pcg_output_xsh_rr_64_32(self.seed)
  }
}

impl Iterator for McgXshRr<u64, u32> {
  type Item = u32;

  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

impl NumberGenerator<u64> for McgXshRr<u128, u64> {
  fn get_next(&mut self) -> u64 {
    self.seed = pcg_mcg_128_step_r(self.seed);
    pcg_output_xsh_rr_128_64(self.seed)
  }
}

impl Iterator for McgXshRr<u128, u64> {
  type Item = u64;

  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

// MCG XHS RS Impls

impl NumberGenerator<u8> for McgXshRs<u16, u8> {
  fn get_next(&mut self) -> u8 {
    self.seed = pcg_mcg_16_step_r(self.seed);
    pcg_output_xsh_rs_16_8(self.seed)
  }
}

impl Iterator for McgXshRs<u16, u8> {
  type Item = u8;

  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

impl NumberGenerator<u16> for McgXshRs<u32, u16> {
  fn get_next(&mut self) -> u16 {
    self.seed = pcg_mcg_32_step_r(self.seed);
    pcg_output_xsh_rs_32_16(self.seed)
  }
}

impl Iterator for McgXshRs<u32, u16> {
  type Item = u16;

  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

impl NumberGenerator<u32> for McgXshRs<u64, u32> {
  fn get_next(&mut self) -> u32 {
    self.seed = pcg_mcg_64_step_r(self.seed);
    pcg_output_xsh_rs_64_32(self.seed)
  }
}

impl Iterator for McgXshRs<u64, u32> {
  type Item = u32;

  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

impl NumberGenerator<u64> for McgXshRs<u128, u64> {
  fn get_next(&mut self) -> u64 {
    self.seed = pcg_mcg_128_step_r(self.seed);
    pcg_output_xsh_rs_128_64(self.seed)
  }
}

impl Iterator for McgXshRs<u128, u64> {
  type Item = u64;

  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

// ONESEQ XHS RR Impls

impl NumberGenerator<u8> for OneSeqXshRr<u16, u8> {
  fn get_next(&mut self) -> u8 {
    self.seed = pcg_oneseq_16_step_r(self.seed);
    pcg_output_xsh_rr_16_8(self.seed)
  }
}

impl Iterator for OneSeqXshRr<u16, u8> {
  type Item = u8;

  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

impl NumberGenerator<u16> for OneSeqXshRr<u32, u16> {
  fn get_next(&mut self) -> u16 {
    self.seed = pcg_oneseq_32_step_r(self.seed);
    pcg_output_xsh_rr_32_16(self.seed)
  }
}

impl Iterator for OneSeqXshRr<u32, u16> {
  type Item = u16;

  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

impl NumberGenerator<u32> for OneSeqXshRr<u64, u32> {
  fn get_next(&mut self) -> u32 {
    self.seed = pcg_oneseq_64_step_r(self.seed);
    pcg_output_xsh_rr_64_32(self.seed)
  }
}

impl Iterator for OneSeqXshRr<u64, u32> {
  type Item = u32;

  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

impl NumberGenerator<u64> for OneSeqXshRr<u128, u64> {
  fn get_next(&mut self) -> u64 {
    self.seed = pcg_oneseq_128_step_r(self.seed);
    pcg_output_xsh_rr_128_64(self.seed)
  }
}

impl Iterator for OneSeqXshRr<u128, u64> {
  type Item = u64;

  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

// ONESEQ XHS RS Impls

impl NumberGenerator<u8> for OneSeqXshRs<u16, u8> {
  fn get_next(&mut self) -> u8 {
    self.seed = pcg_oneseq_16_step_r(self.seed);
    pcg_output_xsh_rs_16_8(self.seed)
  }
}

impl Iterator for OneSeqXshRs<u16, u8> {
  type Item = u8;

  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

impl NumberGenerator<u16> for OneSeqXshRs<u32, u16> {
  fn get_next(&mut self) -> u16 {
    self.seed = pcg_oneseq_32_step_r(self.seed);
    pcg_output_xsh_rs_32_16(self.seed)
  }
}

impl Iterator for OneSeqXshRs<u32, u16> {
  type Item = u16;

  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

impl NumberGenerator<u32> for OneSeqXshRs<u64, u32> {
  fn get_next(&mut self) -> u32 {
    self.seed = pcg_oneseq_64_step_r(self.seed);
    pcg_output_xsh_rs_64_32(self.seed)
  }
}

impl Iterator for OneSeqXshRs<u64, u32> {
  type Item = u32;

  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

impl NumberGenerator<u64> for OneSeqXshRs<u128, u64> {
  fn get_next(&mut self) -> u64 {
    self.seed = pcg_oneseq_128_step_r(self.seed);
    pcg_output_xsh_rs_128_64(self.seed)
  }
}

impl Iterator for OneSeqXshRs<u128, u64> {
  type Item = u64;

  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

// ONESEQ RXS M XS Impls

impl NumberGenerator<u8> for OneSeqRxsMXs<u8, u8> {
  fn get_next(&mut self) -> u8 {
    self.seed = pcg_oneseq_8_step_r(self.seed);
    pcg_output_rxs_m_xs_8_8(self.seed)
  }
}

impl Iterator for OneSeqRxsMXs<u8, u8> {
  type Item = u8;

  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

impl NumberGenerator<u16> for OneSeqRxsMXs<u16, u16> {
  fn get_next(&mut self) -> u16 {
    self.seed = pcg_oneseq_16_step_r(self.seed);
    pcg_output_rxs_m_xs_16_16(self.seed)
  }
}

impl Iterator for OneSeqRxsMXs<u16, u16> {
  type Item = u16;

  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

impl NumberGenerator<u32> for OneSeqRxsMXs<u32, u32> {
  fn get_next(&mut self) -> u32 {
    self.seed = pcg_oneseq_32_step_r(self.seed);
    pcg_output_rxs_m_xs_32_32(self.seed)
  }
}

impl Iterator for OneSeqRxsMXs<u32, u32> {
  type Item = u32;

  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

impl NumberGenerator<u64> for OneSeqRxsMXs<u64, u64> {
  fn get_next(&mut self) -> u64 {
    self.seed = pcg_oneseq_64_step_r(self.seed);
    pcg_output_rxs_m_xs_64_64(self.seed)
  }
}

impl Iterator for OneSeqRxsMXs<u64, u64> {
  type Item = u64;

  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

impl NumberGenerator<u128> for OneSeqRxsMXs<u128, u128> {
  fn get_next(&mut self) -> u128 {
    self.seed = pcg_oneseq_128_step_r(self.seed);
    pcg_output_rxs_m_xs_128_128(self.seed)
  }
}

impl Iterator for OneSeqRxsMXs<u128, u128> {
  type Item = u128;

  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

// ONESEQ XSL RR Impls

impl NumberGenerator<u32> for OneSeqXslRr<u64, u32> {
  fn get_next(&mut self) -> u32 {
    self.seed = pcg_oneseq_64_step_r(self.seed);
    pcg_output_xsl_rr_64_32(self.seed)
  }
}

impl Iterator for OneSeqXslRr<u64, u32> {
  type Item = u32;

  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

impl NumberGenerator<u64> for OneSeqXslRr<u128, u64> {
  fn get_next(&mut self) -> u64 {
    self.seed = pcg_oneseq_128_step_r(self.seed);
    pcg_output_xsl_rr_128_64(self.seed)
  }
}

impl Iterator for OneSeqXslRr<u128, u64> {
  type Item = u64;

  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

// MCG XSL RR Impls

impl NumberGenerator<u32> for McgXslRr<u64, u32> {
  fn get_next(&mut self) -> u32 {
    self.seed = pcg_mcg_64_step_r(self.seed);
    pcg_output_xsl_rr_64_32(self.seed)
  }
}

impl Iterator for McgXslRr<u64, u32> {
  type Item = u32;

  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

impl NumberGenerator<u64> for McgXslRr<u128, u64> {
  fn get_next(&mut self) -> u64 {
    self.seed = pcg_mcg_128_step_r(self.seed);
    pcg_output_xsl_rr_128_64(self.seed)
  }
}

impl Iterator for McgXslRr<u128, u64> {
  type Item = u64;

  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

// Usize

impl NumberGenerator<usize> for PcgUsizeWrapper {
  fn get_next(&mut self) -> usize {
    self.inner.get_next() as usize
  }
}

impl Iterator for PcgUsizeWrapper {
  type Item = usize;

  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next() as usize)
  }
}

// Type Aliases

pub type Pcg8 = OneSeqXshRr<u16, u8>;
pub type Pcg16 = OneSeqXshRr<u32, u16>;
pub type Pcg32 = OneSeqXshRr<u64, u32>;
pub type Pcg64 = OneSeqXshRr<u128, u64>;
pub type PcgUsize = PcgUsizeWrapper;

pub type Pcg16XshRr8 = OneSeqXshRr<u16, u8>;
pub type Pcg32XshRr16 = OneSeqXshRr<u32, u16>;
pub type Pcg64XshRr32 = OneSeqXshRr<u64, u32>;
pub type Pcg128XshRr64 = OneSeqXshRr<u128, u64>;

pub type Pcg16XshRs8 = OneSeqXshRs<u16, u8>;
pub type Pcg32XshRs16 = OneSeqXshRs<u32, u16>;
pub type Pcg64XshRs32 = OneSeqXshRs<u64, u32>;
pub type Pcg128XshRs64 = OneSeqXshRs<u128, u64>;

pub type PcgMcg16XshRr8 = McgXshRr<u16, u8>;
pub type PcgMcg32XshRr16 = McgXshRr<u32, u16>;
pub type PcgMcg64XshRr32 = McgXshRr<u64, u32>;
pub type PcgMcg128XshRr64 = McgXshRr<u128, u64>;

pub type PcgMcg16XshRs8 = McgXshRs<u16, u8>;
pub type PcgMcg32XshRs16 = McgXshRs<u32, u16>;
pub type PcgMcg64XshRs32 = McgXshRs<u64, u32>;
pub type PcgMcg128XshRs64 = McgXshRs<u128, u64>;

pub type Pcg8RxsMXs8 = OneSeqRxsMXs<u8, u8>;
pub type Pcg16RxsMXs16 = OneSeqRxsMXs<u16, u16>;
pub type Pcg32RxsMXs32 = OneSeqRxsMXs<u32, u32>;
pub type Pcg64RxsMXs64 = OneSeqRxsMXs<u64, u64>;
pub type Pcg128RxsMXs128 = OneSeqRxsMXs<u128, u128>;

pub type Pcg64XslRr32 = OneSeqXslRr<u64, u32>;
pub type Pcg128XslRr64 = OneSeqXslRr<u128, u64>;

pub type PcgMcg64XslRr32 = McgXslRr<u64, u32>;
pub type PcgMcg128XslRr64 = McgXslRr<u128, u64>;
