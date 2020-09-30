use edn_derive::Serialize;
use transistor::client::Crux;
use transistor::types::http::Actions;
use transistor::types::{
    error::CruxError,
    {query::Query, CruxId},
};

fn query() -> Result<(), CruxError> {
    let crux = Database {
        crux__db___id: CruxId::new("crux"),
        name: "Crux Datalog".to_string(),
        is_sql: false,
    };
    // edn_rs::to_string(crux) { :crux.db/id :crux, :name \"Crux Datalog\", :is-sql false, }

    let psql = Database {
        crux__db___id: CruxId::new("postgres"),
        name: "Postgres".to_string(),
        is_sql: true,
    };
    // edn_rs::to_string(psql) { :crux.db/id :postgres, :name \"Postgres\", :is-sql true, }

    let mysql = Database {
        crux__db___id: CruxId::new("mysql"),
        name: "MySQL".to_string(),
        is_sql: true,
    };
    // edn_rs::to_string(mysql) { :crux.db/id :mysql, :name \"MySQL\", :is-sql true, }

    let client = Crux::new("localhost", "3000").http_client();
    let actions = Actions::new()
        .append_put(crux)
        .append_put(psql)
        .append_put(mysql);
    //  [[:crux.tx/put { :crux.db/id :crux, :name \"Crux Datalog\", :is-sql false, }],
    //   [:crux.tx/put { :crux.db/id :mysql, :name \"MySQL\", :is-sql true, }],
    //   [:crux.tx/put { :crux.db/id :postgres, :name \"Postgres\", :is-sql true, }]]

    let _ = client.tx_log(actions)?;

    let query_is_sql = Query::find(vec!["?p1", "?n"])?
        .where_clause(vec!["?p1 :name ?n", "?p1 :is-sql true"])?
        .build();
    // Query:
    // {:query
    //     {:find [?p1 ?n]
    //      :where [[?p1 :name ?n]
    //              [?p1 :is-sql true]]}}

    let _ = client.query(query_is_sql?)?;
    // BTreeSet{[":mysql", "MySQL"], [":postgres", "Postgres"]}

    let query_is_no_sql = Query::find(vec!["?p1", "?n", "?s"])?
        .where_clause(vec!["?p1 :name ?n", "?p1 :is-sql ?s", "?p1 :is-sql false"])?
        .with_full_results()
        .build();
    // Query:
    // {:query
    //     {:find [?p1]
    //      :where [[?p1 :name ?n]
    //              [?p1 :is-sql ?s]
    //              [?p1 :is-sql false]]}}

    let _ = client.query(query_is_no_sql?)?;
    // BTreeSet{
    //  ["{:crux.db/id: Key(\":cassandra\"), :is-sql: Bool(false), :name: Str(\"Cassandra\"), }", "Cassandra", "false"],
    //  ["{:crux.db/id: Key(\":crux\"), :is-sql: Bool(false), :name: Str(\"Crux Datalog\"), }", "Crux Datalog", "false"]}

    Ok(())
}

fn main() {
    let _ = query();
}

#[test]
fn test_query() {
    query().unwrap()
}

#[derive(Debug, Clone, Serialize)]
#[allow(non_snake_case)]
pub struct Database {
    crux__db___id: CruxId,
    name: String,
    is_sql: bool,
}
