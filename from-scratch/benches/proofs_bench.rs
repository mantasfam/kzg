use criterion::{criterion_group, criterion_main, Criterion};
use kzg::Poly;

fn poly_division_in_finite_field(c: &mut Criterion) {
    c.bench_function(
        "poly_division_in_finite_field",
        |b| b.iter(|| {
            Poly::div(8)
        })
    );
 }
/*
long run_bench(int scale, int max_seconds) {
    timespec_t t0, t1;
    unsigned long total_time = 0, nits = 0;

    uint64_t width = (uint64_t)1 << scale;

    int dividend_length = width;
    int divisor_length = width / 2; // What would be a relevant value of kzg multi-proofs?

    poly dividend, divisor, q;
    new_poly(&dividend, dividend_length);
    new_poly(&divisor, divisor_length);

    for (int i = 0; i < dividend_length; i++) {
        dividend.coeffs[i] = rand_fr();
    }
    for (int i = 0; i < divisor_length; i++) {
        divisor.coeffs[i] = rand_fr();
    }

    // Ensure that the polynomials' orders corresponds to their lengths
    if (fr_is_zero(&dividend.coeffs[dividend.length - 1])) {
        dividend.coeffs[dividend.length - 1] = fr_one;
    }
    if (fr_is_zero(&divisor.coeffs[divisor.length - 1])) {
        divisor.coeffs[divisor.length - 1] = fr_one;
    }

    while (total_time < max_seconds * NANO) {
        clock_gettime(CLOCK_REALTIME, &t0);

        assert(C_KZG_OK == new_poly_div(&q, &dividend, &divisor));

        clock_gettime(CLOCK_REALTIME, &t1);
        nits++;
        total_time += tdiff(t0, t1);

        free_poly(&q);
    }

    free_poly(&dividend);
    free_poly(&divisor);

    return total_time / nits;
} */
