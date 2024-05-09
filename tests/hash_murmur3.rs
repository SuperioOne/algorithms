mod murmur3_hash {
  use algorithms::hash::murmur3::{MurmurHash3_128, MurmurHash3_32};
  use algorithms::hash::HashFunc;

  #[test]
  fn murmur3_32_verify() {
    let hasher = MurmurHash3_32::default();

    assert_eq!(0x00000000, hasher.get_hash(b""));
    assert_eq!(0xBA6BD213, hasher.get_hash(b"test"));
    assert_eq!(0xC0363E43, hasher.get_hash(b"Hello, world!"));
    assert_eq!(
      0x2E4FF723,
      hasher.get_hash(b"The quick brown fox jumps over the lazy dog")
    );
  }

  #[test]
  fn murmur3_32_with_seed_verify() {
    let seed: u32 = 0x9747B28C;
    let hasher = MurmurHash3_32::new_with_seed(seed);

    assert_eq!(0xEBB6C228, hasher.get_hash(b""));
    assert_eq!(0x704B81DC, hasher.get_hash(b"test"));
    assert_eq!(0x24884CBA, hasher.get_hash(b"Hello, world!"));
    assert_eq!(
      0x2FA826CD,
      hasher.get_hash(b"The quick brown fox jumps over the lazy dog")
    );
  }

  #[test]
  fn murmur3_128_verify() {
    let hasher = MurmurHash3_128::default();

    assert_eq!(0x00000000, hasher.get_hash(b""));
    assert_eq!(
      0xAC7D28CC74BDE19D_9A128231F9BD4D82,
      hasher.get_hash(b"test")
    );
    assert_eq!(
      0xF1512DD1D2D665DF_2C326650A8F3C564,
      hasher.get_hash(b"Hello, world!")
    );
    assert_eq!(
      0xE34BBC7BBC071B6C_7A433CA9C49A9347,
      hasher.get_hash(b"The quick brown fox jumps over the lazy dog")
    );
  }

  #[test]
  fn murmur3_128_with_seed_verify() {
    let seed: u64 = 0x9747B28C;
    let hasher = MurmurHash3_128::new_with_seed(seed);

    assert_eq!(0x392B208A1DAABBB3_93B0608FE302957A, hasher.get_hash(b""));
    assert_eq!(
      0xA066A6B76C553018_64A6E65666D07937,
      hasher.get_hash(b"test")
    );
    assert_eq!(
      0xEDC485D662A8392E_F85E7E7631D576BA,
      hasher.get_hash(b"Hello, world!")
    );
    assert_eq!(
      0x738A7F3BD2633121_F94573727EC016E5,
      hasher.get_hash(b"The quick brown fox jumps over the lazy dog")
    );
  }
}
