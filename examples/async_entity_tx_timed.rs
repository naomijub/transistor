use chrono::prelude::*;
use edn_derive::Serialize;
use transistor::client::Crux;
use transistor::edn_rs::EdnError;
use transistor::types::http::Actions;
use transistor::types::response::EntityTxResponse;
use transistor::types::CruxId;

async fn entity_tx_timed() -> EntityTxResponse {
    let person1 = Person {
        crux__db___id: CruxId::new("calor-jorge-3"),
        first_name: "Calors Michael".to_string(),
        last_name: "Jorge".to_string(),
    };

    let person2 = Person {
        crux__db___id: CruxId::new("silva-manuel-1"),
        first_name: "Silva Diego".to_string(),
        last_name: "Manuel".to_string(),
    };

    let client = Crux::new("localhost", "3000").http_client();
    let timed = "2014-11-28T21:00:09-09:00"
        .parse::<DateTime<FixedOffset>>()
        .unwrap();

    let actions = Actions::new()
        .append_put_timed(person1.clone(), timed.clone())
        .append_put_timed(person2, timed.clone());

    let _ = Crux::new("localhost", "3000")
        .http_client()
        .tx_log(actions)
        .await
        .unwrap();

    let entity_tx_body = client
        .entity_tx_timed(person1.crux__db___id, None, Some(timed))
        .await
        .unwrap();

    return entity_tx_body;
}

#[tokio::main]
async fn main() {
    let entity = entity_tx_timed().await;
    println!("\n Edn Body = {:#?}", entity);
    // Edn Body = EntityTxResponse {
    //     db___id: "f936408359776345394b07809bf1fd9bf0f70046",
    //     db___content_hash: "621f30a89898d2c55bc81b0b1e0db0be2878486c",
    //     db___valid_time: 2014-11-29T06:00:09+00:00,
    //     tx___tx_id: 111,
    //     tx___tx_time: 2020-09-30T13:22:02.795+00:00,
    // }
}

#[tokio::test]
async fn test_entity_tx_timed() {
    let entity = entity_tx_timed().await;

    assert!(entity.tx___tx_id > 0);
}

#[derive(Debug, Clone, Serialize)]
#[allow(non_snake_case)]
pub struct Person {
    crux__db___id: CruxId,
    first_name: String,
    last_name: String,
}
