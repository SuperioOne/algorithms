use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[inline]
pub fn basic_find(input: &[u8], byte: u8) -> Option<usize> {
  input.iter().enumerate().find_map(
    |(idx, element)| {
      if *element == byte {
        Some(idx)
      } else {
        None
      }
    },
  )
}

fn criterion_benchmark(c: &mut Criterion) {
  let byte_array_7: &[u8] = b"0123456";
  let byte_array_63: Vec<u8> = (0..63u8).collect();
  let byte_array_128: Vec<u8> = (0..128).collect();
  let byte_array_250: Vec<u8> = (0..250).collect();

  let target_for_7: u8 = 4;
  let target_for_63: u8 = 38;
  let target_for_128: u8 = 89;
  let target_for_250: u8 = 226;

  c.bench_function("basic byte find 250 byte length", |b| {
    b.iter(|| basic_find(black_box(&byte_array_250), target_for_250))
  });

  c.bench_function("7 byte length", |b| {
    b.iter(|| algorithms::find_byte_index::find_byte_index(black_box(&byte_array_7), target_for_7))
  });

  c.bench_function("63 byte length", |b| {
    b.iter(|| {
      algorithms::find_byte_index::find_byte_index(black_box(&byte_array_63), target_for_63)
    })
  });

  c.bench_function("128 byte length", |b| {
    b.iter(|| {
      algorithms::find_byte_index::find_byte_index(black_box(&byte_array_128), target_for_128)
    })
  });

  c.bench_function("250 byte length", |b| {
    b.iter(|| {
      algorithms::find_byte_index::find_byte_index(black_box(&byte_array_250), target_for_250)
    })
  });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
