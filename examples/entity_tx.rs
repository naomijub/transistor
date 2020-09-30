use edn_derive::Serialize;
use transistor::client::Crux;
use transistor::types::http::Actions;
use transistor::types::response::EntityTxResponse;
use transistor::types::CruxId;

fn entity_tx() -> EntityTxResponse {
    let person = Person {
        crux__db___id: CruxId::new("hello-entity"),
        first_name: "Hello".to_string(),
        last_name: "World".to_string(),
    };
    // { :crux.db/id :hello-entity, :first-name \"Hello\", :last-name \"World\", }

    let client = Crux::new("localhost", "3000").http_client();
    let put_person = Actions::new().append_put(person.clone());

    let _ = client.tx_log(put_person).unwrap();
    // {:crux.tx/tx-id 7, :crux.tx/tx-time #inst \"2020-07-16T21:50:39.309-00:00\"}

    let tx_body = client.entity_tx(person.crux__db___id).unwrap();
    return tx_body;
}

fn main() {
    let entity_tx = entity_tx();
    println!("Tx Body = {:#?}", entity_tx);
    // Tx Body = EntityTxResponse {
    //     db___id: "d72ccae848ce3a371bd313865cedc3d20b1478ca",
    //     db___content_hash: "1828ebf4466f98ea3f5252a58734208cd0414376",
    //     db___valid_time: "2020-07-20T20:38:27.515-00:00",
    //     tx___tx_id: 31,
    //     tx___tx_time: "2020-07-20T20:38:27.515-00:00",
    // }
}

#[test]
fn test_entity_tx() {
    let entity_tx = entity_tx();

    assert!(entity_tx.tx___tx_id > 0);
}

#[derive(Debug, Clone, Serialize)]
#[allow(non_snake_case)]
pub struct Person {
    crux__db___id: CruxId,
    first_name: String,
    last_name: String,
}
