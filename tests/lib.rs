use chrono::prelude::*;
use mockito::mock;
use transistor::client::Crux;
use transistor::edn_rs::{ser_struct, Serialize};
use transistor::http::Action;
use transistor::types::CruxId;

#[test]
fn mock_client() {
    let _m = mock("POST", "/tx-log")
        .with_status(200)
        .match_body("[[:crux.tx/put { :crux.db/id :jorge-3, :first-name \"Michael\", :last-name \"Jorge\", }], [:crux.tx/put { :crux.db/id :manuel-1, :first-name \"Diego\", :last-name \"Manuel\", }]]")
        .with_header("content-type", "text/plain")
        .with_body("{:crux.tx/tx-id 8, :crux.tx/tx-time #inst \"2020-07-16T21:53:14.628-00:00\"}")
        .create();

    let body = Crux::new("localhost", "3000")
        .http_mock()
        .tx_log(actions())
        .unwrap();

    assert_eq!(
        format!("{:?}", body),
        String::from("TxLogResponse { tx___tx_id: 8, tx___tx_time:  \"2020-07-16T21:53:14.628-00:00\", tx__event___tx_events: None }")
    );
}

#[cfg(feature = "time")]
#[test]
fn chrono() {
    let _m = mock("POST", "/tx-log")
        .with_status(200)
        .match_body("[[:crux.tx/put { :crux.db/id :jorge-3, :first-name \"Michael\", :last-name \"Jorge\", }], [:crux.tx/put { :crux.db/id :manuel-1, :first-name \"Diego\", :last-name \"Manuel\", }]]")
        .with_header("content-type", "text/plain")
        .with_body("{:crux.tx/tx-id 8, :crux.tx/tx-time #inst \"2020-07-16T21:53:14.628-00:00\"}")
        .create();

    let body = Crux::new("localhost", "3000")
        .http_mock()
        .tx_log(actions())
        .unwrap();

    assert_eq!(
        body.tx___tx_time,
        "2020-07-16T21:53:14.628-00:00"
            .parse::<DateTime<Utc>>()
            .unwrap()
    );
}

fn actions() -> Vec<Action> {
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

    vec![
        Action::Put(person1.serialize()),
        Action::Put(person2.serialize()),
    ]
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
