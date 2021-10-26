use criterion::Criterion;
use kzg::{FFTFr, FFTSettings, Fr};

pub fn bench_fft_fr<TFr: Fr, TFFTSettings: FFTSettings<TFr> + FFTFr<TFr>>(c: &mut Criterion) {
    for scale in 4..16 {
        let fft_settings = TFFTSettings::new(scale as usize).unwrap();
        let data: Vec<TFr> = vec![TFr::rand(); fft_settings.get_max_width()];
        let id = format!("bench_fft_fr scale: '{}'", scale);
        c.bench_function(&id, |b| b.iter(|| fft_settings.fft_fr(&data, false)));
    }
}



/*
fn bench_fft_g1<TFr: Fr, TFFTSettings: FFTSettings<TFr> + FFTFr<TFr>>(c: &mut Criterion) {
    for scale in 4..16
    {
        let fft_settings = TFFTSettings::new(scale as usize).unwrap();
        let curve = Curve::new(&Fr::random(), 2);

        let data: Vec<G1> = (0..(fft_settings.get_max_width() >> 1)).map(|_| &curve.g1_gen * &TFr::random()).collect();
        let id = format!("bench_fft_g1 scale: '{}'", scale);
        c.bench_function(&id, |b| b.iter(|| fft_settings.fft_fr(&data, false)));
    }
} */
