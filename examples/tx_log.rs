use edn_derive::Serialize;
use transistor::client::Crux;
use transistor::types::http::Actions;
use transistor::types::response::TxLogResponse;
use transistor::types::CruxId;

fn tx_log() -> TxLogResponse {
    let person1 = Person {
        crux__db___id: CruxId::new("jorge-3"),
        first_name: "Michael".to_string(),
        last_name: "Jorge".to_string(),
    };
    // edn_rs::to_string(person1) { :crux.db/id :jorge-3, :first-name \"Michael\", :last-name \"Jorge\", }

    let person2 = Person {
        crux__db___id: CruxId::new("manuel-1"),
        first_name: "Diego".to_string(),
        last_name: "Manuel".to_string(),
    };
    // edn_rs::to_string(person2): { :crux.db/id :manuel-1, :first-name \"Diego\", :last-name \"Manuel\", }

    let actions = Actions::new().append_put(person1).append_put(person2);
    // "[[:crux.tx/put { :crux.db/id :jorge-3, :first-name \"Michael\", :last-name \"Jorge\", }],
    //   [:crux.tx/put { :crux.db/id :manuel-1, :first-name \"Diego\", :last-name \"Manuel\", }]]"

    let body = Crux::new("localhost", "3000")
        .http_client()
        .tx_log(actions)
        .unwrap();

    return body;
}

#[test]
fn test_tx_log() {
    let tx_log = tx_log();
    assert!(tx_log.tx___tx_id > 0)
}

fn main() {
    let body = tx_log();
    println!("Body = {:?}", body);
    //  Body = "{:crux.tx/tx-id 7, :crux.tx/tx-time #inst \"2020-07-16T21:50:39.309-00:00\"}"
}

#[derive(Debug, Clone, Serialize)]
#[allow(non_snake_case)]
pub struct Person {
    crux__db___id: CruxId,
    first_name: String,
    last_name: String,
}
