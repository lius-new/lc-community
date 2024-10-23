use criterion::async_executor::FuturesExecutor;
use criterion::{criterion_group, criterion_main, Criterion};

/// 测试登陆
async fn perform_login(server: &axum_test::TestServer) -> () {
    let response = server
        .post("/login")
        .json(&serde_json::json!({
            "nickname":"abc",
            "password":"abc",
        }))
        .await;

    println!("{:?}", response);
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
