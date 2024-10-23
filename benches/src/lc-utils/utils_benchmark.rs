use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn benchmark_sign_with_value_by_utils(c: &mut Criterion) {
    c.bench_function("benchmark  sign_with_value_by_utils", |b| {
        b.iter(|| {
            let _ = lc_utils::sign_with_value("abc");
        })
    });
}

pub fn benchmark_verify_password_by_utils(c: &mut Criterion) {
    let password = "1";
    let password_hash =  "$argon2id$v=19$m=19456,t=2,p=1$MPZeayHODZCNHJlrhG+Xjw$03F3y5b0elI7bz27b+rGHI2BWNEm/WU0KEmv0GbfqT0";
    c.bench_function("benchmark  sign_with_value_by_utils", |b| {
        b.iter(|| {
            let _ =
                lc_utils::verify_password(black_box(password.as_bytes()), black_box(password_hash));
        })
    });
}
//criterion_group!(benches, benchmark_sign_with_value_by_utils);
//criterion_group!(benches, benchmark_verify_password_by_utils);
criterion_main!(benches);
