use chrono::prelude::*;
use transistor::client::Crux;
use transistor::types::error::CruxError;
use transistor::types::CruxId;

async fn entity_timed() -> Result<edn_rs::Edn, CruxError> {
    let client = Crux::new("localhost", "3000").http_client();
    let timed = "2014-11-28T21:00:09-09:00"
        .parse::<DateTime<FixedOffset>>()
        .unwrap();

    let edn_body = client
        .entity_timed(CruxId::new("unknown-id"), None, Some(timed))
        .await;

    return edn_body;
}

#[tokio::main]
async fn main() {
    let edn_body = entity_timed().await;

    println!("\n Edn Body = {:#?}", edn_body);
    // Edn Body = Err(
    //     BadResponse(
    //         "entity responded with 404 for id \":unknown-id\" ",
    //     ),
    // )
}

#[tokio::test]
async fn test_entity_timed() {
    let entity = entity_timed().await;

    match entity {
        Ok(_) => assert!(false),
        Err(e) => assert_eq!(
            format!("{:?}", e),
            format!(
                "{:?}",
                CruxError::BadResponse(
                    "entity responded with 404 for id \":unknown-id\" ".to_string()
                )
            )
        ),
    }
}
