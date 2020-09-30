use edn_derive::Serialize;
use transistor::client::Crux;
use transistor::types::http::Actions;
use transistor::types::response::TxLogResponse;
use transistor::types::CruxId;

fn evict() -> TxLogResponse {
    let person = Person {
        crux__db___id: CruxId::new("hello-evict"),
        first_name: "Hello".to_string(),
        last_name: "World".to_string(),
    };
    // { :crux.db/id :hello-evict, :first-name \"Hello\", :last-name \"World\", }

    let client = Crux::new("localhost", "3000").http_client();

    let actions = Actions::new().append_put(person.clone());
    // [[:crux.tx/put { :crux.db/id :jorge-3, :first-name \"Michael\", :last-name \"Jorge\", }]]"

    let _ = client.tx_log(actions).unwrap();
    //  {:crux.tx/tx-id 7, :crux.tx/tx-time #inst \"2020-07-16T21:50:39.309-00:00\"}

    let actions = Actions::new().append_evict(person.crux__db___id);
    let evict_body = client.tx_log(actions).unwrap();
    return evict_body;
}

fn main() {
    let evict_body = evict();
    println!("\n Evict Body = {:?}", evict_body);
}

#[test]
fn test_evict() {
    let evict = evict();
    assert!(evict.tx___tx_id > 0);
}

#[derive(Debug, Clone, Serialize)]
#[allow(non_snake_case)]
pub struct Person {
    crux__db___id: CruxId,
    first_name: String,
    last_name: String,
}
