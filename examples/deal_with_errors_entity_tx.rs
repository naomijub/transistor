use edn_derive::Serialize;
use transistor::client::Crux;
use transistor::types::error::CruxError;
use transistor::types::response::EntityTxResponse;
use transistor::types::CruxId;

#[cfg(not(feature = "async"))]
fn entity_tx() -> Result<EntityTxResponse, CruxError> {
    let person = Person {
        crux__db___id: CruxId::new("error-id"),
        first_name: "Hello".to_string(),
        last_name: "World".to_string(),
    };
    // { :crux.db/id :hello-entity, :first-name \"Hello\", :last-name \"World\", }

    let client = Crux::new("localhost", "3000").http_client();

    let tx_body = client.entity_tx(person.crux__db___id);
    return tx_body;
}

#[cfg(not(feature = "async"))]
fn main() {
    let entity_tx = entity_tx();
    println!("Tx Body = {:#?}", entity_tx);
    // Tx Body = Err(
    //     BadRequestError(
    //         "entity-tx responded with 404 for id :error-id",
    //     ),
    // )
}

#[test]
#[cfg(not(feature = "async"))]
fn test_entity_tx() {
    let entity_tx = entity_tx();

    match entity_tx {
        Ok(_) => assert!(false),
        Err(e) => assert_eq!(
            format!("{:?}", e),
            format!(
                "{:?}",
                CruxError::BadRequestError(
                    "entity-tx responded with 404 for id \":error-id\" ".to_string()
                )
            )
        ),
    }
}

#[derive(Debug, Clone, Serialize)]
#[allow(non_snake_case)]
pub struct Person {
    crux__db___id: CruxId,
    first_name: String,
    last_name: String,
}
