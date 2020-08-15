use crate::types::{
    error::CruxError,
    http::{Action, Order},
    query::Query,
    response::{
        EntityHistoryResponse, EntityTxResponse, QueryResponse, TxLogResponse, TxLogsResponse,
    },
};
use chrono::prelude::*;
use edn_rs::{edn, Edn, Map, Serialize};
use reqwest::{blocking, header::HeaderMap};
use std::collections::BTreeSet;

static DATE_FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S%Z";

/// `HttpClient` has the `reqwest::blocking::Client`,  the `uri` to query and the `HeaderMap` with
/// all the possible headers. Default header is `Content-Type: "application/edn"`. Synchronous request.
pub struct HttpClient {
    #[cfg(not(feature = "async"))]
    pub(crate) client: blocking::Client,
    #[cfg(feature = "async")]
    pub(crate) client: reqwest::Client,
    pub(crate) uri: String,
    pub(crate) headers: HeaderMap,
}

#[cfg(not(feature = "async"))]
impl HttpClient {
    /// Function `tx_log` requests endpoint `/tx-log` via `POST` which allow you to send actions `Action`
    /// to CruxDB.
    /// The "write" endpoint, to post transactions.
    pub fn tx_log(&self, actions: Vec<Action>) -> Result<TxLogResponse, CruxError> {
        let actions_str = actions
            .into_iter()
            .map(|edn| edn.serialize())
            .collect::<Vec<String>>()
            .join(", ");
        let mut s = String::new();
        s.push_str("[");
        s.push_str(&actions_str);
        s.push_str("]");

        let resp = self
            .client
            .post(&format!("{}/tx-log", self.uri))
            .headers(self.headers.clone())
            .body(s)
            .send()?
            .text()?;
        TxLogResponse::deserialize(resp)
    }

    /// Function `tx_logs` requests endpoint `/tx-log` via `GET` and returns a list of all transactions
    pub fn tx_logs(&self) -> Result<TxLogsResponse, CruxError> {
        let resp = self
            .client
            .get(&format!("{}/tx-log", self.uri))
            .headers(self.headers.clone())
            .send()?
            .text()?;
        TxLogsResponse::deserialize(resp)
    }

    /// Function `entity` requests endpoint `/entity` via `POST` which retrieves the last document
    /// in CruxDB.
    /// Field with `CruxId` is required.
    /// Response is a `reqwest::Result<edn_rs::Edn>` with the last Entity with that ID.
    pub fn entity(&self, id: String) -> Result<Edn, CruxError> {
        if !id.starts_with(":") {
            return Ok(edn!({:status ":bad-request", :message "ID required", :code 400}));
        }

        let mut s = String::new();
        s.push_str("{:eid ");
        s.push_str(&id);
        s.push_str("}");

        let resp = self
            .client
            .post(&format!("{}/entity", self.uri))
            .headers(self.headers.clone())
            .body(s)
            .send()?
            .text()?;

        let edn_resp = edn_rs::parse_edn(&resp);
        Ok(match edn_resp {
            Ok(e) => e,
            Err(err) => {
                println!(":CRUX-CLIENT POST /entity [ERROR]: {:?}", err);
                edn!({:status ":internal-server-error", :code 500})
            }
        })
    }

    /// Function `entity_timed` is like `entity` but with two optional fields `transaction_time` and `valid_time` that are of type `Option<DateTime<FixedOffset>>`.
    pub fn entity_timed(
        &self,
        id: String,
        transaction_time: Option<DateTime<FixedOffset>>,
        valid_time: Option<DateTime<FixedOffset>>,
    ) -> Result<Edn, CruxError> {
        if !id.starts_with(":") {
            return Ok(edn!({:status ":bad-request", :message "ID required", :code 400}));
        }

        let mut s = String::new();
        s.push_str("{:eid ");
        s.push_str(&id);
        s.push_str("}");

        let url = build_timed_url(self.uri.clone(), "entity", transaction_time, valid_time);

        let resp = self
            .client
            .post(&url)
            .headers(self.headers.clone())
            .body(s)
            .send()?
            .text()?;

        let edn_resp = edn_rs::parse_edn(&resp);
        Ok(match edn_resp {
            Ok(e) => e,
            Err(err) => {
                println!(":CRUX-CLIENT POST /entity [ERROR]: {:?}", err);
                edn!({:status ":internal-server-error", :code 500})
            }
        })
    }

    /// Function `entity_tx` requests endpoint `/entity-tx` via `POST` which retrieves the docs and tx infos
    /// for the last document for that ID saved in CruxDB.
    pub fn entity_tx(&self, id: String) -> Result<EntityTxResponse, CruxError> {
        let mut s = String::new();
        s.push_str("{:eid ");
        s.push_str(&id);
        s.push_str("}");

        let resp = self
            .client
            .post(&format!("{}/entity-tx", self.uri))
            .headers(self.headers.clone())
            .body(s)
            .send()?
            .text()?;

        EntityTxResponse::deserialize(resp)
    }

    /// Function `entity_tx_timed` is like `entity_tx` but with two optional fields `transaction_time` and `valid_time` that are of type `Option<DateTime<FixedOffset>>`.
    pub fn entity_tx_timed(
        &self,
        id: String,
        transaction_time: Option<DateTime<FixedOffset>>,
        valid_time: Option<DateTime<FixedOffset>>,
    ) -> Result<EntityTxResponse, CruxError> {
        let mut s = String::new();
        s.push_str("{:eid ");
        s.push_str(&id);
        s.push_str("}");

        let url = build_timed_url(self.uri.clone(), "entity-tx", transaction_time, valid_time);

        let resp = self
            .client
            .post(&url)
            .headers(self.headers.clone())
            .body(s)
            .send()?
            .text()?;

        EntityTxResponse::deserialize(resp)
    }

    /// Function `entity_history` requests endpoint `/entity-history` via `GET` which returns a list with all entity's transaction history.
    /// It is possible to order it with [`Order`](../types/http/enum.Order.html) , `types::http::Order::Asc` and `types::http::Order:Desc`, (second argument) and to include the document for each transaction with the boolean flag `with_docs` (third argument).
    pub fn entity_history(
        &self,
        hash: String,
        order: Order,
        with_docs: bool,
    ) -> Result<EntityHistoryResponse, CruxError> {
        let url = format!(
            "{}/entity-history/{}?sort-order={}&with-docs={}",
            self.uri,
            hash,
            order.serialize(),
            with_docs
        );
        let resp = self
            .client
            .get(&url)
            .headers(self.headers.clone())
            .send()?
            .text()?;

        EntityHistoryResponse::deserialize(resp)
    }

    /// Function `entity_history_timed` is an txtension of the function `entity_history`.
    /// This function receives as the last argument a vector containing [`TimeHistory`](../types/http/enum.TimeHistory.html)  elements.
    /// `TimeHistory` can be `ValidTime` or `TransactionTime` and both have optional `DateTime<Utc>` params corresponding to the start-time and end-time to be queried.
    pub fn entity_history_timed(
        &self,
        hash: String,
        order: Order,
        with_docs: bool,
        time: Vec<crate::types::http::TimeHistory>,
    ) -> Result<EntityHistoryResponse, CruxError> {
        let url = format!(
            "{}/entity-history/{}?sort-order={}&with-docs={}{}",
            self.uri,
            hash,
            order.serialize(),
            with_docs,
            time.serialize().replace("[", "").replace("]", ""),
        );

        println!("{:?}", url);
        let resp = self
            .client
            .get(&url)
            .headers(self.headers.clone())
            .send()?
            .text()?;

        EntityHistoryResponse::deserialize(resp)
    }

    /// Function `query` requests endpoint `/query` via `POST` which retrives a Set containing a vector of the values defined by the function [`Query::find` - github example](https://github.com/naomijub/transistor/blob/master/examples/simple_query.rs#L53).
    /// Argument is a `query` of the type `Query`.
    pub fn query(&self, query: Query) -> Result<BTreeSet<Vec<String>>, CruxError> {
        let resp = self
            .client
            .post(&format!("{}/query", self.uri))
            .headers(self.headers.clone())
            .body(query.serialize())
            .send()?
            .text()?;

        QueryResponse::deserialize(resp)
    }
}

#[cfg(feature = "async")]
use futures::prelude::*;

#[cfg(feature = "async")]
impl HttpClient {
    pub async fn tx_log(&self, actions: Vec<Action>) -> impl Future<Output = TxLogResponse> + Send {
        let actions_str = actions
            .into_iter()
            .map(|edn| edn.serialize())
            .collect::<Vec<String>>()
            .join(", ");
        let mut s = String::new();
        s.push_str("[");
        s.push_str(&actions_str);
        s.push_str("]");

        let resp = self
            .client
            .post(&format!("{}/tx-log", self.uri))
            .headers(self.headers.clone())
            .body(s)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        TxLogResponse::deserialize(resp).unwrap()
    }

    pub async fn tx_logs(&self) -> impl Future<Output = TxLogsResponse> + Send {
        let resp = self
            .client
            .get(&format!("{}/tx-log", self.uri))
            .headers(self.headers.clone())
            .send().await.unwrap()
            .text().await.unwrap();

        TxLogsResponse::deserialize(resp).unwrap()
    }
}

fn build_timed_url(
    url: String,
    endpoint: &str,
    transaction_time: Option<DateTime<FixedOffset>>,
    valid_time: Option<DateTime<FixedOffset>>,
) -> String {
    match (transaction_time, valid_time) {
        (None, None) => format!("{}/{}", url, endpoint),
        (Some(tx), None) => format!(
            "{}/{}?transaction-time={}",
            url,
            endpoint,
            tx.format(DATE_FORMAT).to_string()
        ),
        (None, Some(valid)) => format!(
            "{}/{}?valid-time={}",
            url,
            endpoint,
            valid.format(DATE_FORMAT).to_string()
        ),
        (Some(tx), Some(valid)) => format!(
            "{}/{}?transaction-time={}&valid-time={}",
            url,
            endpoint,
            tx.format(DATE_FORMAT).to_string(),
            valid.format(DATE_FORMAT).to_string()
        ),
    }
}

#[cfg(test)]
mod http {
    use crate::client::Crux;
    use crate::types::http::Action;
    use crate::types::http::Order;
    use crate::types::{
        query::Query,
        response::{EntityHistoryElement, EntityHistoryResponse, EntityTxResponse, TxLogResponse},
        CruxId,
    };
    use edn_rs::{ser_struct, Serialize};
    use mockito::mock;

    ser_struct! {
        #[derive(Debug, Clone)]
        #[allow(non_snake_case)]
        pub struct Person {
            crux__db___id: CruxId,
            first_name: String,
            last_name: String
        }
    }

    #[test]
    fn tx_log() {
        let _m = mock("POST", "/tx-log")
        .with_status(200)
        .match_body("[[:crux.tx/put { :crux.db/id :jorge-3, :first-name \"Michael\", :last-name \"Jorge\", }], [:crux.tx/put { :crux.db/id :manuel-1, :first-name \"Diego\", :last-name \"Manuel\", }]]")
        .with_header("content-type", "text/plain")
        .with_body("{:crux.tx/tx-id 8, :crux.tx/tx-time #inst \"2020-07-16T21:53:14.628-00:00\"}")
        .create();

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

        let action1 = Action::Put(person1.serialize(), None);
        let action2 = Action::Put(person2.serialize(), None);

        let response = Crux::new("localhost", "4000")
            .http_client()
            .tx_log(vec![action1, action2]);

        assert_eq!(response.unwrap(), TxLogResponse::default())
    }

    #[test]
    fn tx_logs() {
        let _m = mock("GET", "/tx-log")
        .with_status(200)
        .with_header("content-type", "application/edn")
        .with_body("({:crux.tx/tx-id 0, :crux.tx/tx-time #inst \"2020-07-09T23:38:06.465-00:00\", :crux.tx.event/tx-events [[:crux.tx/put \"a15f8b81a160b4eebe5c84e9e3b65c87b9b2f18e\" \"125d29eb3bed1bf51d64194601ad4ff93defe0e2\"]]}{:crux.tx/tx-id 1, :crux.tx/tx-time #inst \"2020-07-09T23:39:33.815-00:00\", :crux.tx.event/tx-events [[:crux.tx/put \"a15f8b81a160b4eebe5c84e9e3b65c87b9b2f18e\" \"1b42e0d5137e3833423f7bb958622bee29f91eee\"]]})")
        .create();

        let response = Crux::new("localhost", "4000").http_client().tx_logs();

        assert_eq!(response.unwrap().tx_events.len(), 2);
    }

    #[test]
    #[should_panic(expected = "The following Edn cannot be parsed to TxLogs: Symbol(\\\"Holy\\\")")]
    fn tx_log_error() {
        let _m = mock("GET", "/tx-log")
            .with_status(200)
            .with_header("content-type", "application/edn")
            .with_body("Holy errors!")
            .create();

        let _error = Crux::new("localhost", "4000")
            .http_client()
            .tx_logs()
            .unwrap();
    }

    #[test]
    fn entity() {
        let expected_body = "Map(Map({\":crux.db/id\": Key(\":hello-entity\"), \":first-name\": Str(\"Hello\"), \":last-name\": Str(\"World\")}))";
        let _m = mock("POST", "/entity")
            .with_status(200)
            .match_body("{:eid :ivan}")
            .with_header("content-type", "application/edn")
            .with_body(expected_body)
            .create();

        let edn_body = Crux::new("localhost", "3000")
            .http_client()
            .entity(":ivan".to_string())
            .unwrap();

        assert!(edn_body.to_string().contains("Map"));
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
            .http_client()
            .entity_tx(":ivan".to_string())
            .unwrap();

        assert_eq!(body, EntityTxResponse::default());
    }

    #[test]
    fn simple_query() {
        let expected_body = "#{[:postgres \"Postgres\" true] [:mysql \"MySQL\" true]}";
        let _m = mock("POST", "/query")
            .with_status(200)
            .with_header("content-type", "application/edn")
            .with_body(expected_body)
            .create();

        let query = Query::find(vec!["?p1", "?n", "?s"])
            .unwrap()
            .where_clause(vec!["?p1 :name ?n", "?p1 :is-sql ?s", "?p1 :is-sql true"])
            .unwrap()
            .build();
        let body = Crux::new("localhost", "3000")
            .http_client()
            .query(query.unwrap())
            .unwrap();

        let response = format!("{:?}", body);
        assert_eq!(
            response,
            "{[\":mysql\", \"MySQL\", \"true\"], [\":postgres\", \"Postgres\", \"true\"]}"
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

        let edn_body = Crux::new("localhost", "3000")
            .http_client()
            .entity_history(
                "ecc6475b7ef9acf689f98e479d539e869432cb5e".to_string(),
                Order::Asc,
                false,
            )
            .unwrap();

        let expected = EntityHistoryResponse {
            history: vec![EntityHistoryElement::default()],
        };

        assert_eq!(edn_body, expected);
    }

    #[test]
    fn entity_history_docs() {
        let expected_body = "({:crux.tx/tx-time \"2020-07-19T04:12:13.788-00:00\", :crux.tx/tx-id 28, :crux.db/valid-time \"2020-07-19T04:12:13.788-00:00\", :crux.db/content-hash  \"1828ebf4466f98ea3f5252a58734208cd0414376\", :crux.db/doc :docs})";
        let _m = mock("GET", "/entity-history/ecc6475b7ef9acf689f98e479d539e869432cb5e?sort-order=asc&with-docs=true")
            .with_status(200)
            .with_header("content-type", "application/edn")
            .with_body(expected_body)
            .create();

        let edn_body = Crux::new("localhost", "3000")
            .http_client()
            .entity_history(
                "ecc6475b7ef9acf689f98e479d539e869432cb5e".to_string(),
                Order::Asc,
                true,
            )
            .unwrap();

        let expected = EntityHistoryResponse {
            history: vec![EntityHistoryElement::default_docs()],
        };

        assert_eq!(edn_body, expected);
    }
}

#[cfg(test)]
mod build_url {
    use super::build_timed_url;
    use chrono::prelude::*;

    #[test]
    fn both_times_are_none() {
        let url = build_timed_url("localhost:3000".to_string(), "entity", None, None);

        assert_eq!(url, "localhost:3000/entity");
    }

    #[test]
    fn both_times_are_some() {
        let url = build_timed_url(
            "localhost:3000".to_string(),
            "entity",
            Some(
                "2020-08-09T18:05:29.301-03:00"
                    .parse::<DateTime<FixedOffset>>()
                    .unwrap(),
            ),
            Some(
                "2020-11-09T18:05:29.301-03:00"
                    .parse::<DateTime<FixedOffset>>()
                    .unwrap(),
            ),
        );

        assert_eq!(url, "localhost:3000/entity?transaction-time=2020-08-09T18:05:29-03:00&valid-time=2020-11-09T18:05:29-03:00");
    }

    #[test]
    fn only_tx_time_is_some() {
        let url = build_timed_url(
            "localhost:3000".to_string(),
            "entity",
            Some(
                "2020-08-09T18:05:29.301-03:00"
                    .parse::<DateTime<FixedOffset>>()
                    .unwrap(),
            ),
            None,
        );

        assert_eq!(
            url,
            "localhost:3000/entity?transaction-time=2020-08-09T18:05:29-03:00"
        );
    }

    #[test]
    fn only_valid_time_is_some() {
        let url = build_timed_url(
            "localhost:3000".to_string(),
            "entity",
            None,
            Some(
                "2020-08-09T18:05:29.301-03:00"
                    .parse::<DateTime<FixedOffset>>()
                    .unwrap(),
            ),
        );

        assert_eq!(
            url,
            "localhost:3000/entity?valid-time=2020-08-09T18:05:29-03:00"
        );
    }
}
