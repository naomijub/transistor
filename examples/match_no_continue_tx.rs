use transistor::client::Crux;
use transistor::edn_rs::{ser_struct, Serialize};
use transistor::types::http::Actions;
use transistor::types::{
    error::CruxError,
    {query::Query, CruxId},
};

fn main() -> Result<(), CruxError> {
    let mut crux = Database {
        crux__db___id: CruxId::new("crux"),
        name: "Crux Datalog".to_string(),
        is_sql: false,
    };

    let client = Crux::new("localhost", "3000").http_client();
    let actions = Actions::new().append_put(crux.clone());

    let _ = client.tx_log(actions)?;

    let query = Query::find(vec!["?d"])?
        .where_clause(vec!["?d :is-sql false"])?
        .build()?;

    let query_response = client.query(query)?;

    let id = CruxId::new(&query_response.iter().next().unwrap()[0]);
    let edn_body = client.entity(id).unwrap();
    println!("{:?}", edn_body);
    // Map(Map({":crux.db/id": Key(":crux"), ":is-sql": Bool(false), ":name": Str("Crux Datalog")}))

    crux.name = "banana".to_string();
    let actions = Actions::new()
        .append_match_doc(CruxId::new(":crux"), crux.clone())
        .append_put(crux.clone());

    let result = client.tx_log(actions)?;

    println!("{:?}", result);
    // TxLogResponse { tx___tx_id: 54, tx___tx_time: "2020-08-09T03:54:20.730-00:00", tx__event___tx_events: None }

    let query = Query::find(vec!["?d"])?
        .where_clause(vec!["?d :is-sql false"])?
        .build()?;

    let query_response = client.query(query)?;

    let id = CruxId::new(&query_response.iter().next().unwrap()[0]);
    let edn_body = client.entity(id).unwrap();
    println!("{:?}", edn_body);
    // Map(Map({":crux.db/id": Key(":crux"), ":is-sql": Bool(false), ":name": Str("Crux Datalog")}))

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
