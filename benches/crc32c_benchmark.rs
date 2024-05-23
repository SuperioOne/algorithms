use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
  c.bench_function("crc32c 16 byte", |b| {
    b.iter(|| algorithms::hash::crc::hash_fn::crc32c(black_box(b"1234567890123456")))
  });

  c.bench_function("crc32c 8 byte", |b| {
    b.iter(|| algorithms::hash::crc::hash_fn::crc32c(black_box(b"12345678")))
  });

  c.bench_function("crc32c 12 byte", |b| {
    b.iter(|| algorithms::hash::crc::hash_fn::crc32c(black_box(b"123456789012")))
  });

  c.bench_function("crc32c 38 byte", |b| {
    b.iter(|| {
      algorithms::hash::crc::hash_fn::crc32c(black_box(b"12345678901234567890123456789012345678"))
    })
  });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
