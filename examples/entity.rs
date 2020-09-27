use transistor::client::Crux;
use transistor::edn_rs::{ser_struct, Deserialize, EdnError, Serialize};
use transistor::types::http::Action;
use transistor::types::CruxId;

fn main() {
    let person = Person {
        crux__db___id: CruxId::new("hello-entity"),
        first_name: "Hello".to_string(),
        last_name: "World".to_string(),
    };
    println!("{:?}", edn_rs::to_string(person.clone()));
    //"{ :crux.db/id :hello-entity, :first-name \"Hello\", :last-name \"World\", }"

    let client = Crux::new("localhost", "3000").http_client();
    let put_person = Action::put(person.clone());

    let body = client.tx_log(vec![put_person]).unwrap();
    // "[[:crux.tx/put { :crux.db/id :hello-entity, :first-name \"Hello\", :last-name \"World\", }]]"
    println!("\n Body = {:?}", body);
    //  Body = "{:crux.tx/tx-id 7, :crux.tx/tx-time #inst \"2020-07-16T21:50:39.309-00:00\"}"

    let edn_body = client.entity(person.crux__db___id).unwrap();
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

    println!(
        "\n Person Parsed Response = {:#?}",
        edn_rs::from_edn::<Person>(&edn_body)
    );
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

impl Deserialize for Person {
    fn deserialize(edn: &edn_rs::Edn) -> Result<Self, EdnError> {
        Ok(Self {
            crux__db___id: edn_rs::from_edn(&edn[":crux.db/id"])?,
            first_name: edn_rs::from_edn(&edn[":first-name"])?,
            last_name: edn_rs::from_edn(&edn[":last-name"])?,
        })
    }
}
