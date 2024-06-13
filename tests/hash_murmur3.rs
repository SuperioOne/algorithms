mod murmur3_hash {
  use std::ptr::copy_nonoverlapping;

  use algorithms::hash::murmur3::hash_fn::{murmurhash3_128, murmurhash3_32};
  use algorithms::hash::murmur3::{Murmur3Hash128, Murmur3Hash32};
  use algorithms::hash::HashFunc;

  const TEST_CASE_0: &'static [u8] = b"";
  const TEST_CASE_1: &'static [u8] = b"test";
  const TEST_CASE_2: &'static [u8] = b"Hello, world!";
  const TEST_CASE_3: &'static [u8] = b"The quick brown fox jumps over the lazy dog";
  // Copypasta for testing very long text. Reference: https://htmx.org/essays/rest-copypasta/
  const TEST_CASE_4: &'static [u8] = "I’d just like to interject for a moment. What you’re referring to as REST, is in fact, JSON/RPC, or as I’ve recently taken to calling it, REST-less. JSON is not a hypermedia unto itself, but rather a plain data format made useful by out of band information as defined by swagger documentation or similar.Many computer users work with a canonical version of REST every day, without realizing it. Through a peculiar turn of events, the version of REST which is widely used today is often called “The Web”, and many of its users are not aware that it is basically the REST-ful architecture, defined by Roy Fielding. There really is a REST, and these people are using it, but it is just a part of The Web they use. REST is the network architecture: hypermedia encodes the state of resources for hypermedia clients. JSON is an essential part of Single Page Applications, but useless by itself; it can only function in the context of a complete API specification. JSON is normally used in combination with SPA libraries: the whole system is basically RPC with JSON added, or JSON/RPC. All these so-called “REST-ful” APIs are really JSON/RPC.".as_bytes();
  const TEST_CASE_5: &'static [u8] = b"0123456";
  const TEST_CASE_6: &'static [u8] = b"01234567";
  const TEST_CASE_7: &'static [u8] = b"012345678";
  const TEST_CASE_8: &'static [u8] = b"0123456789AB";
  const TEST_CASE_9: &'static [u8] = b"0123456789ABCDE";

  #[test]
  fn murmur3_32_verify() {
    let hasher = Murmur3Hash32::default();

    assert_eq!(0x00000000, hasher.get_hash(TEST_CASE_0));
    assert_eq!(0xBA6BD213, hasher.get_hash(TEST_CASE_1));
    assert_eq!(0xC0363E43, hasher.get_hash(TEST_CASE_2));
    assert_eq!(0x2E4FF723, hasher.get_hash(TEST_CASE_3));
    assert_eq!(0x8C16F361, hasher.get_hash(TEST_CASE_4));
    assert_eq!(0xAEF31A8, hasher.get_hash(TEST_CASE_5));
    assert_eq!(0x56831753, hasher.get_hash(TEST_CASE_6));
    assert_eq!(0x5081DA7D, hasher.get_hash(TEST_CASE_7));
    assert_eq!(0xEA51B5DC, hasher.get_hash(TEST_CASE_8));
    assert_eq!(0xD5C4EAA1, hasher.get_hash(TEST_CASE_9));
  }

  #[test]
  fn murmur3_32_with_seed_verify() {
    let seed: u32 = 0x9747B28C;
    let hasher = Murmur3Hash32::new_with_seed(seed);

    assert_eq!(0xEBB6C228, hasher.get_hash(TEST_CASE_0));
    assert_eq!(0x704B81DC, hasher.get_hash(TEST_CASE_1));
    assert_eq!(0x24884CBA, hasher.get_hash(TEST_CASE_2));
    assert_eq!(0x2FA826CD, hasher.get_hash(TEST_CASE_3));
    assert_eq!(0x9AE5D7FA, hasher.get_hash(TEST_CASE_4));
    assert_eq!(0xAB2513B7, hasher.get_hash(TEST_CASE_5));
    assert_eq!(0x7BC64FBE, hasher.get_hash(TEST_CASE_6));
    assert_eq!(0xE0CFE8FB, hasher.get_hash(TEST_CASE_7));
    assert_eq!(0x29982342, hasher.get_hash(TEST_CASE_8));
    assert_eq!(0xE505999C, hasher.get_hash(TEST_CASE_9));
  }

  #[test]
  fn murmur3_128_verify() {
    let hasher = Murmur3Hash128::default();

    assert_eq!(0x00000000, hasher.get_hash(TEST_CASE_0));
    assert_eq!(
      0x9A128231F9BD4D82_AC7D28CC74BDE19D,
      hasher.get_hash(TEST_CASE_1)
    );
    assert_eq!(
      0x2C326650A8F3C564_F1512DD1D2D665DF,
      hasher.get_hash(TEST_CASE_2)
    );
    assert_eq!(
      0x7A433CA9C49A9347_E34BBC7BBC071B6C,
      hasher.get_hash(TEST_CASE_3)
    );
    assert_eq!(
      0x821D0936B48B4438_1F4D7E788D8307ED,
      hasher.get_hash(TEST_CASE_4)
    );
    assert_eq!(
      0xB4EBEF492FDEF34E_13EB9FB82606F7A6,
      hasher.get_hash(TEST_CASE_5)
    );
    assert_eq!(
      0xC3369387D8964920_8236039B7387354D,
      hasher.get_hash(TEST_CASE_6)
    );
    assert_eq!(
      0x72A17AF899D597F1_4C1E87519FE738BA,
      hasher.get_hash(TEST_CASE_7)
    );
    assert_eq!(
      0x9A6096214190A0BD_46967871F3B4400A,
      hasher.get_hash(TEST_CASE_8)
    );
    assert_eq!(
      0x270FC32BD6A3F5DE_D0161EEDC28F1027,
      hasher.get_hash(TEST_CASE_9)
    );
  }

  #[test]
  fn murmur3_128_with_seed_verify() {
    let seed: u64 = 0x9747B28C;
    let hasher = Murmur3Hash128::new_with_seed(seed);

    assert_eq!(
      0x93B0608FE302957A_392B208A1DAABBB3,
      hasher.get_hash(TEST_CASE_0)
    );
    assert_eq!(
      0x64A6E65666D07937_A066A6B76C553018,
      hasher.get_hash(TEST_CASE_1)
    );
    assert_eq!(
      0xF85E7E7631D576BA_EDC485D662A8392E,
      hasher.get_hash(TEST_CASE_2)
    );
    assert_eq!(
      0xF94573727EC016E5_738A7F3BD2633121,
      hasher.get_hash(TEST_CASE_3)
    );
    assert_eq!(
      0x60A64FCEA1001606_E94B201FDADB0A73,
      hasher.get_hash(TEST_CASE_4)
    );
    assert_eq!(
      0x8B6A978A6926EAD7_D4888F3A46F792E3,
      hasher.get_hash(TEST_CASE_5)
    );
    assert_eq!(
      0x48A9D6B322A1C07A_295373671326D416,
      hasher.get_hash(TEST_CASE_6)
    );
    assert_eq!(
      0xC37755B336F3111B_D1117CC8DEEFFB95,
      hasher.get_hash(TEST_CASE_7)
    );
    assert_eq!(
      0xE5AECDBFD78688E6_ABA229BC5FEDE8E3,
      hasher.get_hash(TEST_CASE_8)
    );
    assert_eq!(
      0x373BF069175817C5_4204D1605934BBF1,
      hasher.get_hash(TEST_CASE_9)
    );
  }

  // TODO: move this func to common/mod.rs for other hash function tests
  fn smhasher_verification<'a, F>(hash_bitsize: usize, hash_fn: F) -> Option<u32>
  where
    F: Fn(u64, &[u8]) -> Box<[u8]>,
  {
    let hash_bytes = hash_bitsize / 8;
    let mut hashes = vec![0u8; hash_bytes * 256];
    let hashes_ptr: *mut u8 = hashes.as_mut_ptr();

    for i in 0..=255 {
      // Funny note: Do not use (0..=i) for the keys. Actual first key is [] and last key is [0..254].
      // In C version, array len parameter is `i`, which makes the len 0(empyt array) for the first iteration.
      let key: Vec<u8> = (0..i).collect();
      let hash = hash_fn(256 - (i as u64), &key);

      unsafe {
        copy_nonoverlapping(
          hash.as_ptr(),
          hashes_ptr.byte_add((i as usize) * hash_bytes),
          hash_bytes,
        )
      };
    }

    let final_hash = hash_fn(0, &hashes);

    if final_hash.len() < 4 {
      None
    } else {
      let verification: u32 = *final_hash.get(0).unwrap() as u32
        | (*final_hash.get(1).unwrap() as u32) << 8
        | (*final_hash.get(2).unwrap() as u32) << 16
        | (*final_hash.get(3).unwrap() as u32) << 24;

      Some(verification)
    }
  }

  #[test]
  fn murmur3_128_smhasher_verification() {
    let verification = smhasher_verification(128, |seed, bytes| {
      let result = murmurhash3_128(seed, bytes);
      let mut hash_bytes = Box::new([0u8; 16]);
      unsafe { hash_bytes.as_mut_ptr().cast::<u128>().write(result) };

      hash_bytes
    });

    assert_eq!(Some(0x6384BA69), verification);
  }

  #[test]
  fn murmur3_32_smhasher_verification() {
    let verification = smhasher_verification(32, |seed, bytes| {
      let result = murmurhash3_32(seed as u32, bytes);
      let mut hash_bytes = Box::new([0u8; 8]);
      unsafe { hash_bytes.as_mut_ptr().cast::<u32>().write(result) };

      hash_bytes
    });

    assert_eq!(Some(0xB0F57EE3), verification);
  }
}
