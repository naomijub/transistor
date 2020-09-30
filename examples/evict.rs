use edn_derive::Serialize;
use transistor::client::Crux;
use transistor::types::http::Actions;
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

    let actions = Actions::new().append_put(person.clone());
    let body = client.tx_log(actions).unwrap();
    // "[[:crux.tx/put { :crux.db/id :jorge-3, :first-name \"Michael\", :last-name \"Jorge\", }]]"
    println!("\n Body = {:?}", body);
    //  Body = "{:crux.tx/tx-id 7, :crux.tx/tx-time #inst \"2020-07-16T21:50:39.309-00:00\"}"

    let actions = Actions::new().append_evict(person.crux__db___id);
    let evict_body = client.tx_log(actions).unwrap();
    println!("\n Evict Body = {:?}", evict_body);
}

#[derive(Debug, Clone, Serialize)]
#[allow(non_snake_case)]
pub struct Person {
    crux__db___id: CruxId,
    first_name: String,
    last_name: String,
}
