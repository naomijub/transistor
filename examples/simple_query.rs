use transistor::client::Crux;
use transistor::docker::Action;
use transistor::edn_rs::{ser_struct, Serialize};
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
    println!("{:?}", crux.clone().serialize());
    println!("{:?}", psql.clone().serialize());
    println!("{:?}", mysql.clone().serialize());
    // "{ :crux.db/id :crux, :name \"Crux Datalog\", :is-sql false, }"
    // "{ :crux.db/id :postgres, :name \"Postgres\", :is-sql true, }"
    // "{ :crux.db/id :mysql, :name \"MySQL\", :is-sql true, }"

    let client = Crux::new("localhost", "3000").docker_client();
    let action1 = Action::Put(crux.serialize());
    let action2 = Action::Put(psql.serialize());
    let action3 = Action::Put(mysql.serialize());

    let _ = client.tx_log(vec![action1, action2, action3])?;
    // Request body for vec![action1, action2]
    // "[[:crux.tx/put { :crux.db/id :crux, :name \"Crux Datalog\", :is-sql false, }],
    //   [:crux.tx/put { :crux.db/id :mysql, :name \"MySQL\", :is-sql true, }],
    //   [:crux.tx/put { :crux.db/id :postgres, :name \"Postgres\", :is-sql true, }]]"

    let query_is_sql = Query::find(vec!["?p1", "?n"])?
        .where_clause(vec!["?p1 :name ?n", "?p1 :is-sql true"])?
        .build();
    // Query:
    // {:query
    //     {:find [?p1 ?n]
    //      :where [[?p1 :name ?n]
    //              [?p1 :is-sql true]]}}

    let is_sql = client.query(query_is_sql?)?;
    println!("{:?}", is_sql);
    // {[":mysql", "MySQL"], [":postgres", "Postgres"]} BTreeSet

    let query_is_no_sql = Query::find(vec!["?p1", "?n", "?s"])?
        .where_clause(vec!["?p1 :name ?n", "?p1 :is-sql ?s", "?p1 :is-sql false"])?
        .build();
    // Query:
    // {:query
    //     {:find [?p1 ?n ?s]
    //      :where [[?p1 :name ?n]
    //              [?p1 :is-sql ?s]
    //              [?p1 :is-sql false]]}}

    let is_no_sql = client.query(query_is_no_sql?)?;
    println!("{:?}", is_no_sql);
    // {[":crux", "Crux Datalog", "false"]} BTreeSet

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
