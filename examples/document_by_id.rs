use transistor::docker::{Action};
use transistor::client::Crux;
use transistor::types::{CruxId};
use transistor::edn_rs::{ser_struct, Serialize};

fn main() {
    let person = Person {
        crux__db___id: CruxId::new("hello-entity"), 
        first_name: "Hello".to_string(), 
        last_name: "World".to_string()
    };
    println!("{:?}", person.clone().serialize());
    //"{ :crux.db/id :hello-entity, :first-name \"Hello\", :last-name \"World\", }"

    let client = Crux::new("localhost", "3000").docker_client();
    let put_person = Action::Put(person.clone().serialize());

    let body = client.tx_log(vec![put_person]).unwrap();
    // "[[:crux.tx/put { :crux.db/id :hello-entity, :first-name \"Hello\", :last-name \"World\", }]]"
    println!("\n Body = {:?}", body);
    //  Body = "{:crux.tx/tx-id 7, :crux.tx/tx-time #inst \"2020-07-16T21:50:39.309-00:00\"}"

    let tx_body = client.entity_tx(person.crux__db___id.serialize()).unwrap();
    println!("\n Tx Body = {:#?}", tx_body.clone());
    // Tx Body = EntityTxResponse {
    //     db___id: "d72ccae848ce3a371bd313865cedc3d20b1478ca",
    //     db___content_hash: "1828ebf4466f98ea3f5252a58734208cd0414376",
    //     db___valid_time: "2020-07-20T20:38:27.515-00:00",
    //     tx___tx_id: 31,
    //     tx___tx_time: "2020-07-20T20:38:27.515-00:00",
    // }

    let document = client.document_by_id(tx_body.db___content_hash).unwrap();
    println!("\n Document Body = {:#?}", Person::from(document));
    // Document Body = Person {
    //     crux__db___id: CruxId(
    //         ":hello-entity",
    //     ),
    //     first_name: "Hello",
    //     last_name: "World",
    // }
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

impl From<edn_rs::Edn> for Person {
    fn from(edn: edn_rs::Edn) -> Self {
        Self {
            crux__db___id: CruxId::new(&edn[":crux.db/id"].to_string()),
            first_name: edn[":first-name"].to_string(),
            last_name: edn[":last-name"].to_string(),
        }
    }
}