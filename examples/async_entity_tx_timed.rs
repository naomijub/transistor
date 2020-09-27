use chrono::prelude::*;
use transistor::client::Crux;
use transistor::edn_rs::{ser_struct, Deserialize, EdnError, Serialize};
use transistor::types::http::Actions;
use transistor::types::CruxId;

#[tokio::main]
async fn main() {
    let person1 = Person {
        crux__db___id: CruxId::new("calor-jorge-3"),
        first_name: "Calors Michael".to_string(),
        last_name: "Jorge".to_string(),
    };

    let person2 = Person {
        crux__db___id: CruxId::new("silva-manuel-1"),
        first_name: "Silva Diego".to_string(),
        last_name: "Manuel".to_string(),
    };

    let client = Crux::new("localhost", "3000").http_client();
    let timed = "2014-11-28T21:00:09-09:00"
        .parse::<DateTime<FixedOffset>>()
        .unwrap();

    let actions = Actions::new()
        .append_put_timed(person1, timed)
        .append_put_timed(person2, timed)
        .build();

    let _ = Crux::new("localhost", "3000")
        .http_client()
        .tx_log(actions)
        .await
        .unwrap();

    let edn_body = client
        .entity_tx_timed(person1.crux__db___id, None, Some(timed))
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
