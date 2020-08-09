use crate::types::{
    error::CruxError,
    http::{Action, Order},
    query::Query,
    response::{
        EntityHistoryResponse, EntityTxResponse, QueryResponse, StateResponse, TxLogResponse,
        TxLogsResponse,
    },
};
use edn_rs::{edn, Edn, Map, Serialize};
use reqwest::{blocking::Client, header::HeaderMap};
use std::collections::BTreeSet;

/// `HttpClient` has the `reqwest::blocking::Client`,  the `uri` to query and the `HeaderMap` with
/// all the possible headers. Default header is `Content-Type: "application/edn"`. Synchronous request.
pub struct HttpClient {
    pub(crate) client: Client,
    pub(crate) uri: String,
    pub(crate) headers: HeaderMap,
}

impl HttpClient {
    /// Function `state` queries endpoint `/` with a `GET`. Returned information consists of
    /// various details about the state of the database and it can be used as a health check.
    pub fn state(&self) -> Result<StateResponse, CruxError> {
        let resp = self
            .client
            .get(&self.uri)
            .headers(self.headers.clone())
            .send()?
            .text()?;
        StateResponse::deserialize(resp)
    }

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

    #[cfg(feature = "time")]
    pub fn entity_history_timed(
        &self,
        hash: String,
        order: Order,
        with_docs: bool,
        time: Vec<crate::types::http::time::TimeHistory>,
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

#[cfg(test)]
mod http {
    use crate::client::Crux;
    use crate::types::http::Action;
    use crate::types::http::Order;
    use crate::types::{
        query::Query,
        response::{
            EntityHistoryElement, EntityHistoryResponse, EntityTxResponse, StateResponse,
            TxLogResponse,
        },
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
    fn state() {
        let _m = mock("GET", "/")
        .with_status(200)
        .with_header("content-type", "text/plain")
        .with_body("{:crux.index/index-version 5, :crux.doc-log/consumer-state nil, :crux.tx-log/consumer-state nil, :crux.kv/kv-store \"crux.kv.rocksdb.RocksKv\", :crux.kv/estimate-num-keys 34, :crux.kv/size 88489}")
        .create();

        let response = Crux::new("localhost", "4000").http_client().state();

        assert_eq!(response.unwrap(), StateResponse::default())
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

        let action1 = Action::Put(person1.serialize());
        let action2 = Action::Put(person2.serialize());

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
