use chrono::prelude::*;
use transistor::client::Crux;
use transistor::edn_rs::{ser_struct, Deserialize, EdnError, Serialize};
use transistor::types::http::Action;
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

    let client = Crux::new("localhost", "3000").http_client();
    let timed = "2014-11-28T21:00:09-09:00"
        .parse::<DateTime<FixedOffset>>()
        .unwrap();

    let action1 = Action::Put(person1.clone(), Some(timed));
    let action2 = Action::Put(person2, Some(timed));

    let _ = Crux::new("localhost", "3000")
        .http_client()
        .tx_log(vec![action1, action2])
        .await
        .unwrap();

    let edn_body = client
        .entity_timed(person1.crux__db___id, None, Some(timed))
        .await
        .unwrap();

    println!("\n Edn Body = {:#?}", edn_body);
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
