use edn_derive::Serialize;
use transistor::client::Crux;
use transistor::types::response::TxLogResponse;
use transistor::types::Actions;
use transistor::types::CruxId;

async fn tx_log() -> TxLogResponse {
    let person1 = Person {
        crux__db___id: CruxId::new("jorge-3"),
        first_name: "Michael".to_string(),
        last_name: "Jorge".to_string(),
    };

    let person2 = Person {
        crux__db___id: CruxId::new("manuel-1"),
        first_name: "Diego".to_string(),
        last_name: "Manuel".to_string(),
    };

    let actions = Actions::new().append_put(person1).append_put(person2);

    let body = Crux::new("localhost", "3000")
        .http_client()
        .tx_log(actions)
        .await
        .unwrap();

    return body;
}

#[tokio::main]
async fn main() {
    let tx_log = tx_log().await;
    println!("body = {:?}", tx_log);
    // Body = "{:crux.tx/tx-id 7, :crux.tx/tx-time #inst \"2020-07-16T21:50:39.309-00:00\"}"
}

#[tokio::test]
async fn test_tx_log() {
    let tx_log = tx_log().await;
    assert!(tx_log.tx___tx_id > 0);
}

#[derive(Debug, Clone, Serialize)]
#[allow(non_snake_case)]
pub struct Person {
    crux__db___id: CruxId,
    first_name: String,
    last_name: String,
}
