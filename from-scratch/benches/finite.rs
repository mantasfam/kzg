/*
use criterion::Criterion;
use kzg::{FFTFr, FFTSettings, Fr};
 use test::black_box;

 pub fn criterion_benchmark<TFr: Fr, TFFTSettings: FFTSettings<TFr> + FFTFr<TFr>>(c: &mut Criterion) {
     c.bench_function("add_fr", |b| b.iter(||
         kzg::finite::add_fr(black_box(TFr::default()),
                             black_box(TFr::default()))));
 }
 */
