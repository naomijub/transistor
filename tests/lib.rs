#[cfg(all(feature = "time", feature = "mock"))]
mod integration {
    use chrono::prelude::*;
    use mockito::mock;
    use transistor::client::Crux;
    use transistor::edn_rs::{ser_struct, Serialize};
    use transistor::types::http::{Action, Order};
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
            String::from("TxLogResponse { tx___tx_id: 8, tx___tx_time: 2020-07-16T21:53:14.628Z, tx__event___tx_events: None }")
        );
    }

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

    #[test]
    fn entity_history() {
        let expected_body = "({:crux.tx/tx-time \"2020-07-19T04:12:13.788-00:00\", :crux.tx/tx-id 28, :crux.db/valid-time \"2020-07-19T04:12:13.788-00:00\", :crux.db/content-hash  \"1828ebf4466f98ea3f5252a58734208cd0414376\"})";
        let _m = mock("GET", "/entity-history/ecc6475b7ef9acf689f98e479d539e869432cb5e?sort-order=asc&with-docs=false")
            .with_status(200)
            .with_header("content-type", "application/edn")
            .with_body(expected_body)
            .create();

        let body = Crux::new("localhost", "3000")
            .http_mock()
            .entity_history(
                "ecc6475b7ef9acf689f98e479d539e869432cb5e".to_string(),
                Order::Asc,
                false,
            )
            .unwrap();

        let actual = format!("{:?}", body);
        let expected = "EntityHistoryResponse { history: [EntityHistoryElement { db___valid_time: 2020-07-19T04:12:13.788Z, tx___tx_id: 28, tx___tx_time: 2020-07-19T04:12:13.788Z, db___content_hash: \"1828ebf4466f98ea3f5252a58734208cd0414376\", db__doc: None }] }";
        assert_eq!(actual, expected);
    }

    #[test]
    fn entity_tx() {
        let expected_body = "{:crux.db/id \"d72ccae848ce3a371bd313865cedc3d20b1478ca\", :crux.db/content-hash \"1828ebf4466f98ea3f5252a58734208cd0414376\", :crux.db/valid-time #inst \"2020-07-19T04:12:13.788-00:00\", :crux.tx/tx-time #inst \"2020-07-19T04:12:13.788-00:00\", :crux.tx/tx-id 28}";
        let _m = mock("POST", "/entity-tx")
            .with_status(200)
            .match_body("{:eid :ivan}")
            .with_header("content-type", "application/edn")
            .with_body(expected_body)
            .create();

        let body = Crux::new("localhost", "3000")
            .http_mock()
            .entity_tx(":ivan".to_string())
            .unwrap();

        let actual = format!("{:?}", body);
        let expected = "EntityTxResponse { db___id: \"d72ccae848ce3a371bd313865cedc3d20b1478ca\", db___content_hash: \"1828ebf4466f98ea3f5252a58734208cd0414376\", db___valid_time: 2020-07-19T04:12:13.788Z, tx___tx_id: 28, tx___tx_time: 2020-07-19T04:12:13.788Z }";

        assert_eq!(actual, expected);
    }

    #[test]
    fn match_tx_date_times() {
        use transistor::types::http::time::TimeHistory;

        let date = "2014-11-28T21:00:09+09:00"
            .parse::<DateTime<Utc>>()
            .unwrap();
        let m = mock("GET", "/entity-history/ecc6475b7ef9acf689f98e479d539e869432cb5e?sort-order=asc&with-docs=false&start-transaction-time=2014-11-28T12:00:09&end-transaction-time=2014-11-28T12:00:09")
            .create();

        let _ = Crux::new("localhost", "3000")
            .http_mock()
            .entity_history_timed(
                "ecc6475b7ef9acf689f98e479d539e869432cb5e".to_string(),
                Order::Asc,
                false,
                vec![TimeHistory::TransactionTime(Some(date), Some(date))],
            );

        m.assert();
    }

    #[test]
    fn match_tx_end_date() {
        use transistor::types::http::time::TimeHistory;

        let date = "2014-11-28T21:00:09+09:00"
            .parse::<DateTime<Utc>>()
            .unwrap();
        let m = mock("GET", "/entity-history/ecc6475b7ef9acf689f98e479d539e869432cb5e?sort-order=asc&with-docs=false&end-transaction-time=2014-11-28T12:00:09")
            .create();

        let _ = Crux::new("localhost", "3000")
            .http_mock()
            .entity_history_timed(
                "ecc6475b7ef9acf689f98e479d539e869432cb5e".to_string(),
                Order::Asc,
                false,
                vec![TimeHistory::TransactionTime(None, Some(date))],
            );

        m.assert();
    }

    #[test]
    fn match_valid_start_date() {
        use transistor::types::http::time::TimeHistory;

        let date = "2014-11-28T21:00:09+09:00"
            .parse::<DateTime<Utc>>()
            .unwrap();
        let m = mock("GET", "/entity-history/ecc6475b7ef9acf689f98e479d539e869432cb5e?sort-order=asc&with-docs=false&start-valid-time=2014-11-28T12:00:09")
            .create();

        let _ = Crux::new("localhost", "3000")
            .http_mock()
            .entity_history_timed(
                "ecc6475b7ef9acf689f98e479d539e869432cb5e".to_string(),
                Order::Asc,
                false,
                vec![TimeHistory::ValidTime(Some(date), None)],
            );

        m.assert();
    }

    #[test]
    fn match_none_date() {
        use transistor::types::http::time::TimeHistory;

        let m = mock("GET", "/entity-history/ecc6475b7ef9acf689f98e479d539e869432cb5e?sort-order=asc&with-docs=false")
            .create();

        let _ = Crux::new("localhost", "3000")
            .http_mock()
            .entity_history_timed(
                "ecc6475b7ef9acf689f98e479d539e869432cb5e".to_string(),
                Order::Asc,
                false,
                vec![TimeHistory::ValidTime(None, None)],
            );

        m.assert();
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
}
