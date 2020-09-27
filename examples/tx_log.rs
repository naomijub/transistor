use transistor::client::Crux;
use transistor::edn_rs::{ser_struct, Serialize};
use transistor::types::http::Actions;
use transistor::types::CruxId;

fn main() {
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
    println!("{:?}", edn_rs::to_string(person1.clone()));
    //"{ :crux.db/id :jorge-3, :first-name \"Michael\", :last-name \"Jorge\", }"
    println!("{:?}", edn_rs::to_string(person2.clone()));
    //"{ :crux.db/id :manuel-1, :first-name \"Diego\", :last-name \"Manuel\", }"

    let actions = Actions::new().append_put(person1).append_put(person2);

    let body = Crux::new("localhost", "3000")
        .http_client()
        .tx_log(actions)
        .unwrap();
    // Request body for Actions
    // "[[:crux.tx/put { :crux.db/id :jorge-3, :first-name \"Michael\", :last-name \"Jorge\", }],
    //   [:crux.tx/put { :crux.db/id :manuel-1, :first-name \"Diego\", :last-name \"Manuel\", }]]"

    println!("\n Body = {:?}", body);
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
