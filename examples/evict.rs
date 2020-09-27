use transistor::client::Crux;
use transistor::edn_rs::{ser_struct, Serialize};
use transistor::types::http::Action;
use transistor::types::CruxId;

fn main() {
    let person = Person {
        crux__db___id: CruxId::new("hello-evict"),
        first_name: "Hello".to_string(),
        last_name: "World".to_string(),
    };
    println!("{:?}", edn_rs::to_string(person.clone()));
    //"{ :crux.db/id :hello-evict, :first-name \"Hello\", :last-name \"World\", }"

    let client = Crux::new("localhost", "3000").http_client();

    let put_person = Action::put(person.clone(), None);
    let body = client.tx_log(vec![put_person]).unwrap();
    // "[[:crux.tx/put { :crux.db/id :jorge-3, :first-name \"Michael\", :last-name \"Jorge\", }]]"
    println!("\n Body = {:?}", body);
    //  Body = "{:crux.tx/tx-id 7, :crux.tx/tx-time #inst \"2020-07-16T21:50:39.309-00:00\"}"

    let evict_person = Action::evict(person.crux__db___id);
    let evict_body = client.tx_log(vec![evict_person]).unwrap();
    println!("\n Evict Body = {:?}", evict_body);
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
