mod cityhash {
  use algorithms::hash::{cityhash::CityHash_32, cityhash::CityHash_64, HashFunc};

  #[test]
  fn cityhash_32_verify() {
    let hasher = CityHash_32::default();

    assert_eq!(0x61571065, hasher.get_hash(b"test"));
    assert_eq!(0x6FE1B5B5, hasher.get_hash(b"Hello, world!"));
    assert_eq!(
      0xA339C810,
      hasher.get_hash(b"The quick brown fox jumps over the lazy dog")
    );
  }

  // Tests edge cases
  #[test]
  fn cityhash_32_from_0_to_4() {
    let hasher = CityHash_32::default();

    assert_eq!(0xDC56D17A, hasher.get_hash(b""));
    assert_eq!(0xD63C2322, hasher.get_hash(b"0"));
    assert_eq!(0xE5B54743, hasher.get_hash(b"01"));
    assert_eq!(0xC243F2A6, hasher.get_hash(b"012"));
    assert_eq!(0xA7A6F80B, hasher.get_hash(b"0123"));
  }

  #[test]
  fn cityhash_32_from_5_to_12() {
    let hasher = CityHash_32::default();

    assert_eq!(0xEC60F6A0, hasher.get_hash(b"01234"));
    assert_eq!(0x99D93E22, hasher.get_hash(b"01234567"));
    assert_eq!(0x62D2EDE3, hasher.get_hash(b"0123456789AB"));
  }

  #[test]
  fn cityhash_32_from_13_to_24() {
    let hasher = CityHash_32::default();

    assert_eq!(0xDD51E526, hasher.get_hash(b"0123456789ABC"));
    assert_eq!(0x82697553, hasher.get_hash(b"0123456789ABCDEFGH"));
    assert_eq!(0xDB3F1CF1, hasher.get_hash(b"0123456789ABCDEFGHIJKLMN"));
  }

  #[test]
  fn cityhash_64_verify() {
    let hasher = CityHash_64::default();

    assert_eq!(0x7717383DAA85B5B2, hasher.get_hash(b"test"));
    assert_eq!(0x307C26B3E0789A47, hasher.get_hash(b"Hello, world!"));
    assert_eq!(
      0xC268724928FECA7D,
      hasher.get_hash(b"The quick brown fox jumps over the lazy dog")
    );
  }

  #[test]
  fn cityhash_64_from_0_to_16() {
    let hasher = CityHash_64::default();

    assert_eq!(0x9AE16A3B2F90404F, hasher.get_hash(b""));
    assert_eq!(0xD2ED96073B81823F, hasher.get_hash(b"0"));
    assert_eq!(0x73D7FA164971735C, hasher.get_hash(b"012"));
    assert_eq!(0x839ACC613DFB3687, hasher.get_hash(b"0123"));
    assert_eq!(0x1D096F87B7B28685, hasher.get_hash(b"0123456"));
    assert_eq!(0x6DFF3AF4CE43E8F6, hasher.get_hash(b"01234567"));
    assert_eq!(0x5C6A6DB7CFC1DF39, hasher.get_hash(b"0123456789AB"));
    assert_eq!(0x6DA7F6E5AD5A11D4, hasher.get_hash(b"0123456789ABCDEF"));
  }

  #[test]
  fn cityhash_64_from_17_to_32() {
    let hasher = CityHash_64::default();

    assert_eq!(0x519EF8279FA56DA8, hasher.get_hash(b"0123456789ABCDEFG"));
    assert_eq!(
      0x79E3DE290DD64958,
      hasher.get_hash(b"0123456789ABCDEFGHIJKLM")
    );
    assert_eq!(
      0x46B788F01C3BEF9D,
      hasher.get_hash(b"0123456789ABCDEFGHIJKLMNOPQRSTUV")
    );
  }

  #[test]
  fn cityhash_64_from_33_to_64() {
    let hasher = CityHash_64::default();

    assert_eq!(
      0x3122B526C080B298,
      hasher.get_hash(b"0123456789ABCDEFGHIJKLMNOPQRSTUVW")
    );
    assert_eq!(
      0x68AEEA84C323BFEE,
      hasher.get_hash(b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789")
    );
    assert_eq!(
      0x964ECAE01F972966,
      hasher.get_hash(b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789ABCDEFGHIJKLMNOPQR")
    );
  }

  #[test]
  fn cityhash_64_beyond_65() {
    let hasher = CityHash_64::default();

    assert_eq!(
      0xB1CDABB8345B282F,
      hasher.get_hash(b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ")
    );

    assert_eq!(0xD800360A5EE32429, hasher.get_hash(b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ"));
  }

  #[test]
  fn cityhash_64_with_seed() {
    let seed_u64: u64 = 65;
    let seed_u128: u128 = (256_u128 << 64) + 43_u128;
    let input = b"The quick brown fox jumps over the lazy dog";

    let hasher_0 = CityHash_64::new_with_seed(seed_u64.into());
    let hasher_1 = CityHash_64::new_with_seed(seed_u128.into());

    assert_eq!(0xE3951FDAB7579E23, hasher_0.get_hash(input));
    assert_eq!(0x9DB7790AA52E2889, hasher_1.get_hash(input));
  }
}
