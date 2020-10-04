use edn_derive::Serialize;
use transistor::client::Crux;
use transistor::types::error::CruxError;
use transistor::types::Actions;
use transistor::types::{query::Query, CruxId};

async fn query() -> Result<(), CruxError> {
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
    let actions = Actions::new()
        .append_put(crux)
        .append_put(psql)
        .append_put(mysql);

    let _ = client.tx_log(actions).await.unwrap();

    let query_is_sql = Query::find(vec!["?p1", "?n"])
        .unwrap()
        .where_clause(vec!["?p1 :name ?n", "?p1 :is-sql true"])
        .unwrap()
        .build();

    let is_sql = client.query(query_is_sql.unwrap()).await.unwrap();
    // QueryAsyncResponse({[":mysql", "MySQL"], [":postgres", "Postgres"]}) BTreeSet

    let query_is_no_sql = Query::find(vec!["?p1", "?n", "?s"])
        .unwrap()
        .where_clause(vec!["?p1 :name ?n", "?p1 :is-sql ?s", "?p1 :is-sql false"])
        .unwrap()
        .with_full_results()
        .build()
        .unwrap();

    let is_no_sql = client.query(query_is_no_sql).await.unwrap();
    // {["{:crux.db/id: Key(\":cassandra\"), :is-sql: Bool(false), :name: Str(\"Cassandra\"), }", "Cassandra", "false"],
    //  ["{:crux.db/id: Key(\":crux\"), :is-sql: Bool(false), :name: Str(\"Crux Datalog\"), }", "Crux Datalog", "false"]}

    Ok(())
}

#[tokio::main]
async fn main() {
    let _ = query().await.unwrap();
}

#[tokio::test]
async fn test_query() {
    let _ = query().await.unwrap();
}

#[derive(Debug, Clone, Serialize)]
#[allow(non_snake_case)]
pub struct Database {
    crux__db___id: CruxId,
    name: String,
    is_sql: bool,
}
