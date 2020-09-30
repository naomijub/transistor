use edn_derive::Serialize;
use transistor::client::Crux;
use transistor::types::http::{Actions, Order};
use transistor::types::response::EntityTxResponse;
use transistor::types::CruxId;

fn entity_tx() -> EntityTxResponse {
    let person = Person {
        crux__db___id: CruxId::new("hello-history"),
        first_name: "Hello".to_string(),
        last_name: "World".to_string(),
    };

    let put_person = Actions::new().append_put(person.clone());

    let client = Crux::new("localhost", "3000").http_client();
    let _ = client.tx_log(put_person).unwrap();

    let tx_body = client.entity_tx(person.crux__db___id).unwrap();
    return tx_body;
}

#[test]
fn tent_entity_history_with_docs() {
    let client = Crux::new("localhost", "3000").http_client();
    let tx_body = entity_tx();
    let docs = client
        .entity_history(tx_body.db___id.clone(), Order::Asc, true)
        .unwrap();
    assert!(docs.history[0].db__doc.is_some())
}

#[test]
fn tent_entity_history_without_docs() {
    let client = Crux::new("localhost", "3000").http_client();
    let tx_body = entity_tx();
    let docs = client
        .entity_history(tx_body.db___id.clone(), Order::Asc, false)
        .unwrap();
    assert!(docs.history[0].db__doc.is_none())
}

fn main() {
    let client = Crux::new("localhost", "3000").http_client();
    let tx_body = entity_tx();
    let _ = client.entity_history(tx_body.db___id.clone(), Order::Asc, true);
    // EntityHistoryResponse { history: [
    //     EntityHistoryElement {
    //         db___valid_time: "2020-08-05T03:00:06.476-00:00",
    //         tx___tx_id: 37, tx___tx_time: "2020-08-05T03:00:06.476-00:00",
    //         db___content_hash: "2da097a2dffbb9828cd4377f1461a59e8454674b",
    //         db__doc: Some(Map(Map({":crux.db/id": Key(":hello-history"), ":first-name": Str("Hello"), ":last-name": Str("World")})))
    //     }
    // ]}

    let _ = client.entity_history(tx_body.db___id, Order::Asc, false);
    // EntityHistoryResponse {
    //     history: [
    //         EntityHistoryElement {
    //              db___valid_time: "2020-08-05T03:00:06.476-00:00",
    //              tx___tx_id: 37,
    //              tx___tx_time: "2020-08-05T03:00:06.476-00:00",
    //              db___content_hash: "2da097a2dffbb9828cd4377f1461a59e8454674b",
    //              db__doc: None
    //             }
    //         }
    //     ]}
}

#[derive(Debug, Clone, Serialize)]
#[allow(non_snake_case)]
pub struct Person {
    crux__db___id: CruxId,
    first_name: String,
    last_name: String,
}
