use criterion::{criterion_group, criterion_main, Criterion};
use edn_derive::Serialize;
use transistor::types::http::Actions;
use transistor::types::CruxId;

fn criterion_benchmark(c: &mut Criterion) {
    use transistor::client::Crux;

    c.bench_function("tx-log", |b| {
        b.iter(|| {
            Crux::new("localhost", "3000")
                .http_client()
                .tx_log(actions())
                .unwrap()
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

fn actions() -> Actions {
    let person1 = Person {
        crux__db___id: CruxId::new("jorge-3"),
        first_name: "Michael".to_string(),
        last_name: "Jorge".to_string(),
    };

    let person2 = Person {
        crux__db___id: CruxId::new("manuel-1"),
        first_name: "Diego".to_string(),
        last_name: "Manuel".to_string(),
    };

    Actions::new().append_put(person1).append_put(person2)
}

#[derive(Debug, Clone, Serialize)]
#[allow(non_snake_case)]
pub struct Person {
    crux__db___id: CruxId,
    first_name: String,
    last_name: String,
}
