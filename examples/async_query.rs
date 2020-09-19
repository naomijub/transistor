use transistor::client::Crux;
use transistor::edn_rs::{ser_struct, Serialize};
use transistor::types::http::Action;
use transistor::types::{query::Query, CruxId};

#[tokio::main]
async fn main() {
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

    let client = Crux::new("localhost", "3000").http_client();
    let action1 = Action::Put(edn_rs::to_string(crux), None);
    let action2 = Action::Put(edn_rs::to_string(psql), None);
    let action3 = Action::Put(edn_rs::to_string(mysql), None);

    let _ = client
        .tx_log(vec![action1, action2, action3])
        .await
        .unwrap();

    let query_is_sql = Query::find(vec!["?p1", "?n"])
        .unwrap()
        .where_clause(vec!["?p1 :name ?n", "?p1 :is-sql true"])
        .unwrap()
        .build();

    let is_sql = client.query(query_is_sql.unwrap()).await.unwrap();
    println!("{:?}", is_sql);
    // QueryAsyncResponse({[":mysql", "MySQL"], [":postgres", "Postgres"]}) BTreeSet

    let query_is_no_sql = Query::find(vec!["?p1", "?n", "?s"])
        .unwrap()
        .where_clause(vec!["?p1 :name ?n", "?p1 :is-sql ?s", "?p1 :is-sql false"])
        .unwrap()
        .with_full_results()
        .build()
        .unwrap();

    let is_no_sql = client.query(query_is_no_sql).await.unwrap();
    println!("{:?}", is_no_sql);
    // {["{:crux.db/id: Key(\":cassandra\"), :is-sql: Bool(false), :name: Str(\"Cassandra\"), }", "Cassandra", "false"],
    //  ["{:crux.db/id: Key(\":crux\"), :is-sql: Bool(false), :name: Str(\"Crux Datalog\"), }", "Crux Datalog", "false"]}
}

ser_struct! {
    #[derive(Debug, Clone)]
    #[allow(non_snake_case)]
    pub struct Database {
        crux__db___id: CruxId,
        name: String,
        is_sql: bool
    }
}
