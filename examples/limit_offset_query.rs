use transistor::client::Crux;
use transistor::edn_rs::{ser_struct, Serialize};
use transistor::types::http::Action;
use transistor::types::{
    error::CruxError,
    {query::Query, CruxId},
};

fn main() -> Result<(), CruxError> {
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
    let action1 = Action::put(crux);
    let action2 = Action::put(psql);
    let action3 = Action::put(mysql);
    let action4 = Action::put(cassandra);
    let action5 = Action::put(sqlserver);

    let _ = client.tx_log(vec![action1, action2, action3, action4, action5])?;
    // Request body for vec![action1, action2]
    // "[[:crux.tx/put { :crux.db/id :crux, :name \"Crux Datalog\", :is-sql false, }],
    //   [:crux.tx/put { :crux.db/id :mysql, :name \"MySQL\", :is-sql true, }],
    //   [:crux.tx/put { :crux.db/id :postgres, :name \"Postgres\", :is-sql true, }]]"

    let query_is_sql = Query::find(vec!["?p1", "?n"])?
        .where_clause(vec!["?p1 :name ?n"])?
        .order_by(vec!["?n :desc"])?
        .limit(3)
        .offset(1)
        .build();
    // "{:query\n {:find [?p1 ?n]\n:where [[?p1 :name ?n]]\n:order-by [[?n :desc]]\n:limit 3\n:offset 1\n}}"

    let is_sql = client.query(query_is_sql?)?;
    println!("{:?}", is_sql);
    // {[":crux", "Crux Datalog"], [":mysql", "MySQL"], [":postgres", "Postgres"]}

    Ok(())
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
