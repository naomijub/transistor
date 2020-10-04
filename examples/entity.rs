use edn_derive::Serialize;
use transistor::client::Crux;
use transistor::edn_rs::{Deserialize, EdnError};
use transistor::types::Actions;
use transistor::types::CruxId;

#[cfg(not(feature = "async"))]
fn entity() -> edn_rs::Edn {
    let person = Person {
        crux__db___id: CruxId::new("hello-entity"),
        first_name: "Hello".to_string(),
        last_name: "World".to_string(),
    };
    // { :crux.db/id :hello-entity, :first-name \"Hello\", :last-name \"World\", }

    let client = Crux::new("localhost", "3000").http_client();
    let put_person = Actions::new().append_put(person.clone());

    let _ = client.tx_log(put_person).unwrap();
    // {:crux.tx/tx-id 7, :crux.tx/tx-time #inst \"2020-07-16T21:50:39.309-00:00\"}

    let edn_body = client.entity(person.crux__db___id).unwrap();

    return edn_body;
}

#[test]
#[cfg(not(feature = "async"))]
fn test_entity() {
    let edn_body = entity();
    let person = edn_rs::from_edn::<Person>(&edn_body);
    let expected = Person {
        crux__db___id: CruxId::new("hello-entity"),
        first_name: "Hello".to_string(),
        last_name: "World".to_string(),
    };

    assert_eq!(person.unwrap(), expected);
}

#[cfg(not(feature = "async"))]
fn main() {
    let edn_body = entity();
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

#[derive(Debug, PartialEq, Clone, Serialize)]
#[allow(non_snake_case)]
pub struct Person {
    crux__db___id: CruxId,
    first_name: String,
    last_name: String,
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
