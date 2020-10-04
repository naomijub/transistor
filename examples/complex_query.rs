use edn_derive::Serialize;
use transistor::client::Crux;
use transistor::types::Actions;
use transistor::types::{
    error::CruxError,
    {query::Query, CruxId},
};

#[cfg(not(feature = "async"))]
fn query() -> Result<(), CruxError> {
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

    let _ = client.tx_log(actions)?;

    let query_is_sql = Query::find(vec!["?p1", "?n"])?
        .where_clause(vec!["?p1 :name ?n", "?p1 :is-sql ?sql"])?
        .order_by(vec!["?n :asc"])?
        .args(vec!["?sql true"])?
        .build();
    // {:query\n {:find [?p1 ?n]\n:where [[?p1 :name ?n]\n[?p1 :is-sql ?sql]]\n:args [{?sql true}]\n:order-by [[?n :asc]]\n}}

    let _ = client.query(query_is_sql?)?;
    // {[":mysql", "MySQL"], [":postgres", "Postgres"], [":sqlserver", "Sql Server"]}

    Ok(())
}

#[cfg(not(feature = "async"))]
fn main() {
    let _ = query();
}

#[test]
#[cfg(not(feature = "async"))]
fn test_query() {
    query().unwrap();
}

#[derive(Debug, Clone, Serialize)]
#[allow(non_snake_case)]
pub struct Database {
    crux__db___id: CruxId,
    name: String,
    is_sql: bool,
}
