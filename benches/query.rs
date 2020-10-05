use criterion::{criterion_group, criterion_main, Criterion};
use edn_derive::Serialize;
use transistor::client::Crux;
use transistor::types::Actions;
use transistor::types::{query::Query, CruxId};

fn criterion_benchmark(c: &mut Criterion) {
    let client = Crux::new("localhost", "3000").http_client();
    tx_log_entities();
    let query_is_sql = Query::find(vec!["?p1", "?n"])
        .unwrap()
        .where_clause(vec!["?p1 :name ?n", "?p1 :is-sql ?sql"])
        .unwrap()
        .order_by(vec!["?n :asc"])
        .unwrap()
        .args(vec!["?sql true"])
        .unwrap()
        .build()
        .unwrap();

    c.bench_function("query", |b| {
        b.iter(|| client.query(query_is_sql.clone()).unwrap())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

fn tx_log_entities() {
    let crux = Database {
        crux__db___id: CruxId::new("crux"),
        name: "Crux Datalog".to_string(),
        is_sql: false,
    };

    let psql = Database {
        crux__db___id: CruxId::new("postgres"),
        name: "Postgres".to_string(),
        is_sql: true,
    };

    let mysql = Database {
        crux__db___id: CruxId::new("mysql"),
        name: "MySQL".to_string(),
        is_sql: true,
    };

    let cassandra = Database {
        crux__db___id: CruxId::new("cassandra"),
        name: "Cassandra".to_string(),
        is_sql: false,
    };

    let sqlserver = Database {
        crux__db___id: CruxId::new("sqlserver"),
        name: "Sql Server".to_string(),
        is_sql: true,
    };

    let client = Crux::new("localhost", "3000").http_client();
    let actions = Actions::new()
        .append_put(crux)
        .append_put(psql)
        .append_put(mysql)
        .append_put(cassandra)
        .append_put(sqlserver);

    let _ = client.tx_log(actions).unwrap();
}

#[derive(Debug, Clone, Serialize)]
#[allow(non_snake_case)]
pub struct Database {
    crux__db___id: CruxId,
    name: String,
    is_sql: bool,
}
