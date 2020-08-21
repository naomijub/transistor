use chrono::prelude::*;
use transistor::client::Crux;
use transistor::edn_rs::{ser_struct, Serialize};
use transistor::types::http::Action;
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

    let action1 = Action::Put(person1.clone().serialize(), Some(timed));
    let action2 = Action::Put(person2.serialize(), Some(timed));

    let _ = Crux::new("localhost", "3000")
        .http_client()
        .tx_log(vec![action1, action2])
        .await;

    let edn_body = client
        .entity_tx_timed(person1.crux__db___id.serialize(), None, Some(timed))
        .await;

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

impl From<edn_rs::Edn> for Person {
    fn from(edn: edn_rs::Edn) -> Self {
        Self {
            crux__db___id: CruxId::new(&edn[":crux.db/id"].to_string()),
            first_name: edn[":first-name"].to_string(),
            last_name: edn[":last-name"].to_string(),
        }
    }
}
