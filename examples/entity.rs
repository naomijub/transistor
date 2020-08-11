use transistor::client::Crux;
use transistor::edn_rs::{ser_struct, Serialize};
use transistor::types::http::Action;
use transistor::types::CruxId;

fn main() {
    let person = Person {
        crux__db___id: CruxId::new("hello-entity"),
        first_name: "Hello".to_string(),
        last_name: "World".to_string(),
    };
    println!("{:?}", person.clone().serialize());
    //"{ :crux.db/id :hello-entity, :first-name \"Hello\", :last-name \"World\", }"

    let client = Crux::new("localhost", "3000").http_client();
    let put_person = Action::Put(person.clone().serialize(), None);

    let body = client.tx_log(vec![put_person]).unwrap();
    // "[[:crux.tx/put { :crux.db/id :hello-entity, :first-name \"Hello\", :last-name \"World\", }]]"
    println!("\n Body = {:?}", body);
    //  Body = "{:crux.tx/tx-id 7, :crux.tx/tx-time #inst \"2020-07-16T21:50:39.309-00:00\"}"

    let edn_body = client.entity(person.crux__db___id.serialize()).unwrap();
    println!("\n Edn Body = {:#?}", edn_body.clone());
    // Edn Body = Map(
    //     Map(
    //         {
    //             ":crux.db/id": Key(
    //                 ":hello-entity",
    //             ),
    //             ":first-name": Str(
    //                 "Hello",
    //             ),
    //             ":last-name": Str(
    //                 "World",
    //             ),
    //         },
    //     ),
    // )

    println!("\n Person Parsed Response = {:#?}", Person::from(edn_body));
    // Person Parsed Response = Person {
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
