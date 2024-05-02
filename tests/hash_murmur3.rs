mod murmur3_hash {
  use algorithms::hash::murmur3::hash_fn::murmur3_32;

  #[test]
  fn murmur3_32_verification() {
    assert_eq!(0x00000000, murmur3_32(0, b""));
    assert_eq!(0x514E28B7, murmur3_32(0x00000001, b""));
    assert_eq!(0x81F16F39, murmur3_32(0xFFFFFFFF, b""));

    assert_eq!(0xBA6BD213, murmur3_32(0, b"test"));
    assert_eq!(0x704B81DC, murmur3_32(0x9747B28C, b"test"));

    assert_eq!(0xC0363E43, murmur3_32(0, b"Hello, world!"));
    assert_eq!(0x24884CBA, murmur3_32(0x9747B28C, b"Hello, world!"));

    assert_eq!(
      0x2E4FF723,
      murmur3_32(0, b"The quick brown fox jumps over the lazy dog")
    );
    assert_eq!(
      0x2FA826CD,
      murmur3_32(0x9747B28C, b"The quick brown fox jumps over the lazy dog")
    );
  }
}
