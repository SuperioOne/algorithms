use algorithms::buffer_utils::FastBufferUtils;

#[test]
fn find_byte_index_0_to_7() {
  let buffer: [u8; 7] = [2, 7, 17, 3, 19, 36, 25];

  for (idx, val) in buffer.iter().enumerate() {
    let result = buffer.fast_find(*val);
    assert_eq!(Some(idx), result);
  }
}

#[test]
fn find_byte_index_8_to_63() {
  let target_len: Vec<u8> = (8..64).collect();

  for len in target_len {
    let buffer: Vec<u8> = (0..len).collect();

    for (idx, val) in buffer.iter().enumerate() {
      let result = buffer.fast_find(*val);
      assert_eq!(Some(idx), result);
    }
  }
}

#[test]
fn find_byte_index_64_to_256() {
  let target_len: Vec<u8> = (64..=255).collect();

  for len in target_len {
    let buffer: Vec<u8> = (0..len).collect();

    for (idx, val) in buffer.iter().enumerate() {
      let result = buffer.fast_find(*val);
      assert_eq!(Some(idx), result);
    }
  }
}

#[test]
fn find_byte_index_not_exists() {
  let target_len: Vec<u8> = (0..255).collect();

  for len in target_len {
    let buffer: Vec<u8> = (0..len).collect();
    let result = buffer.fast_find(255);
    assert_eq!(None, result);
  }
}
