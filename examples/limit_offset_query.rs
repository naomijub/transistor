use edn_derive::Serialize;
use transistor::client::Crux;
use transistor::types::Actions;
use transistor::types::{
    error::CruxError,
    {query::Query, CruxId},
};

#[cfg(not(feature = "async"))]
fn limit_offset() -> Result<(), CruxError> {
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
    //  [[:crux.tx/put { :crux.db/id :crux, :name \"Crux Datalog\", :is-sql false, }],
    //   [:crux.tx/put { :crux.db/id :mysql, :name \"MySQL\", :is-sql true, }],
    //   [:crux.tx/put { :crux.db/id :postgres, :name \"Postgres\", :is-sql true, }]]

    let _ = client.tx_log(actions)?;

    let query_is_sql = Query::find(vec!["?p1", "?n"])?
        .where_clause(vec!["?p1 :name ?n"])?
        .order_by(vec!["?n :desc"])?
        .limit(3)
        .offset(1)
        .build();
    // "{:query\n {:find [?p1 ?n]\n:where [[?p1 :name ?n]]\n:order-by [[?n :desc]]\n:limit 3\n:offset 1\n}}"

    let _ = client.query(query_is_sql?)?;
    // {[":crux", "Crux Datalog"], [":mysql", "MySQL"], [":postgres", "Postgres"]}

    Ok(())
}

#[cfg(not(feature = "async"))]
fn main() {
    let _ = limit_offset();
}

#[test]
#[cfg(not(feature = "async"))]
fn test_limit_offset() {
    limit_offset().unwrap();
}

#[derive(Debug, Clone, Serialize)]
#[allow(non_snake_case)]
pub struct Database {
    crux__db___id: CruxId,
    name: String,
    is_sql: bool,
}
