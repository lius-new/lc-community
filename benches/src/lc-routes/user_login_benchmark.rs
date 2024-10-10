use criterion::async_executor::FuturesExecutor;
use criterion::{criterion_group, criterion_main, Criterion};

async fn perform_login(server: &axum_test::TestServer) -> axum_test::TestResponse {
    server.get("/login").await
}

fn benchmark_login(c: &mut Criterion) {
    let server = axum_test::TestServer::new(lc_routes::users::build_api_users_router()).unwrap();

    c.bench_function("route login", |b| {
        b.to_async(FuturesExecutor)
            .iter(|| async { perform_login(&server).await })
    });
}

criterion_group!(benches, benchmark_login);
criterion_main!(benches);
