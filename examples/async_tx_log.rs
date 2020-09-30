use transistor::client::Crux;
use transistor::edn_rs::{ser_struct, Serialize};
use transistor::types::http::Actions;
use transistor::types::CruxId;

#[tokio::main]
async fn main() {
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

    println!("body = {:?}", body);
    //  Body = "{:crux.tx/tx-id 7, :crux.tx/tx-time #inst \"2020-07-16T21:50:39.309-00:00\"}"
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
