use transistor::client::Crux;
use transistor::edn_rs::{ser_struct, Serialize};
use transistor::types::http::{Action, Order};
use transistor::types::CruxId;

fn main() {
    let person = Person {
        crux__db___id: CruxId::new("hello-history"),
        first_name: "Hello".to_string(),
        last_name: "World".to_string(),
    };

    let put_person = Action::put(person.clone(), None);

    let client = Crux::new("localhost", "3000").http_client();
    let _ = client.tx_log(vec![put_person]).unwrap();

    let tx_body = client
        .entity_tx(edn_rs::to_string(person.crux__db___id))
        .unwrap();

    let entity_history = client.entity_history(tx_body.db___id.clone(), Order::Asc, true);
    println!("{:?}", entity_history.unwrap());
    // EntityHistoryResponse { history: [
    //     EntityHistoryElement {
    //         db___valid_time: "2020-08-05T03:00:06.476-00:00",
    //         tx___tx_id: 37, tx___tx_time: "2020-08-05T03:00:06.476-00:00",
    //         db___content_hash: "2da097a2dffbb9828cd4377f1461a59e8454674b",
    //         db__doc: Some(Map(Map({":crux.db/id": Key(":hello-history"), ":first-name": Str("Hello"), ":last-name": Str("World")})))
    //     }
    // ]}

    let entity_history_without_docs = client.entity_history(tx_body.db___id, Order::Asc, false);
    println!("{:?}", entity_history_without_docs.unwrap());
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

ser_struct! {
    #[derive(Debug, Clone)]
    #[allow(non_snake_case)]
    pub struct Person {
        crux__db___id: CruxId,
        first_name: String,
        last_name: String
    }
}
