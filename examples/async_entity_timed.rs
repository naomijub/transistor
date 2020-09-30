use chrono::prelude::*;
use edn_derive::{Deserialize, Serialize};
use transistor::client::Crux;
use transistor::types::http::Actions;
use transistor::types::CruxId;

async fn entity_timed() -> edn_rs::Edn {
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

    let actions = Actions::new()
        .append_put_timed(person1.clone(), timed)
        .append_put_timed(person2, timed);

    let _ = Crux::new("localhost", "3000")
        .http_client()
        .tx_log(actions)
        .await
        .unwrap();

    let edn_body = client
        .entity_timed(person1.crux__db___id, None, Some(timed))
        .await
        .unwrap();

    return edn_body;
}

#[tokio::main]
async fn main() {
    let edn_body = entity_timed().await;

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

#[tokio::test]
async fn test_entity_timed() {
    let edn_body = entity_timed().await;
    let entity = edn_rs::from_edn::<Person>(&edn_body).unwrap();
    let expected = Person {
        crux__db___id: CruxId::new("jorge-3"),
        first_name: "Michael".to_string(),
        last_name: "Jorge".to_string(),
    };

    assert_eq!(entity, expected);
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Person {
    crux__db___id: CruxId,
    first_name: String,
    last_name: String,
}
