use reqwest::{
    Result,
    header::{HeaderMap,AUTHORIZATION, CONTENT_TYPE},
    blocking::{Client}, 
};
use edn_rs::Serialize;
use crate::types::{StateResponse, TxLogResponse, TxLogsResponse};


/// Struct to connect define parameters to connect to Crux
/// `host` and `port` are reuired.
pub struct Crux {
    host: String,
    port: String,
    headers: HeaderMap
}

impl Crux{
    /// Define Crux instance with `host:port`
    pub fn new(host: &str, port: &str) -> Self {
        Self{host: host.to_string(), port: port.to_string(), headers: HeaderMap::new()}
    }

    /// Function to add `AUTHORIZATION` token to the Crux Client
    pub fn with_authorization(mut self, authorization: &str) -> Self {
        self.headers.insert(AUTHORIZATION, authorization.parse().unwrap());
        self
    }

    #[cfg(not(test))]
    fn uri(&self) -> String {
        format!("http://{}:{}", self.host, self.port)
    }

    #[cfg(test)]
    fn uri(&self) -> String {
        use mockito::server_url;
        server_url()
    }

    /// To query database via http it is necessary to use `CruxClient` 
    pub fn client(&mut self) -> CruxClient {
        self.headers.insert(CONTENT_TYPE, "application/edn".parse().unwrap());
        CruxClient {
            client: reqwest::blocking::Client::new(),
            uri: self.uri().clone(),
            headers: self.headers.clone()
        }
    }
}

/// `CruxClient` has the `reqwest::Client`, the `uri` to query and the `HeaderMap` with
/// all the possible headers. Default header is `Content-Type: "application/edn"`
pub struct CruxClient {
    client: Client,
    uri: String, 
    headers: HeaderMap,
}

/// Action to perform in Crux. Receives a serialized Edn
/// **First firld of your edn should be `crux__db___id: CruxId`**
/// Allowed actions:
/// * `PUT` - inserts into Crux
///  * `Delete` - updates Crux state to previous value
pub enum Action {
    Put(String),
    Delete(String)
}

impl Serialize for Action {
    fn serialize(self) -> String {
        match self {
            Action::Put(edn) => format!("[:crux.tx/put {}]", edn),
            Action::Delete(edn) => format!("[:crux.tx/delete {}]", edn)
        }
    }
}

impl CruxClient {
    /// Function `state` queries endpoint `/` with a `GET` Returned information consists of
    /// various details about the state of the database and it can be used as a health check.
    pub fn state(&self) -> Result<StateResponse> {
        let resp = self.client.get(&self.uri)
            .headers(self.headers.clone())
            .send()?
            .text()?;
        Ok(StateResponse::deserialize(resp))
    }

    /// Function `tx_log` interacts with endpoint `/tx-log` via `POST` which allow you to send actions `Action`
    /// to CruxDB.
    pub fn tx_log(&self, actions: Vec<Action>) -> Result<TxLogResponse> {
        let actions_str = actions.into_iter().map(|edn| edn.serialize()).collect::<Vec<String>>().join(", ");
        let mut s = String::new();
        s.push_str("[");
        s.push_str(&actions_str);
        s.push_str("]");
        
        println!("{:?}", s);
        let resp = self.client.post(&format!("{}/tx-log", self.uri))
            .headers(self.headers.clone())
            .body(s)
            .send()?
            .text()?;
        Ok(TxLogResponse::deserialize(resp))
    }

    pub fn tx_logs(&self) -> Result<TxLogsResponse> {
        let resp = self.client.get(&format!("{}/tx-log", self.uri))
            .headers(self.headers.clone())
            .send()?
            .text()?;
        Ok(TxLogsResponse::deserialize(resp))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        let actual = Crux::new("host", "port");
        let expected = Crux {
            host: String::from("host"),
            port: String::from("port"),
            headers: HeaderMap::new(),
        };

        assert_eq!(actual.host, expected.host);
        assert_eq!(actual.port, expected.port);
        assert_eq!(actual.headers, expected.headers);
    }

    #[test]
    fn authorization() {
        let crux = Crux::new("host", "port").with_authorization("auth");
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, "auth".parse().unwrap());

        assert_eq!(crux.headers, headers);
    }

    #[test]
    fn uri() {
        let crux = Crux::new("localhost", "1234");

        assert_eq!(crux.uri(), "http://127.0.0.1:1234")
    }

    #[test]
    fn client() {
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, "auth".parse().unwrap());
        headers.insert(CONTENT_TYPE, "application/edn".parse().unwrap());

        let actual = Crux::new("127.0.0.1", "1234").with_authorization("auth").client();
        let expected = CruxClient {
            client: reqwest::blocking::Client::new(),
            uri: "http://127.0.0.1:1234".to_string(),
            headers: headers,
        };

        assert_eq!(actual.uri, expected.uri);
        assert_eq!(actual.headers, expected.headers);
    }
}

#[cfg(test)]
mod client {
    use super::{Crux, Action};
    use crate::types::{StateResponse, TxLogResponse, CruxId};
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

        let response = Crux::new("localhost", "4000").client().state();

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
            last_name: "Jorge".to_string()
        };
    
        let person2 = Person {
            crux__db___id: CruxId::new("manuel-1"), 
            first_name: "Diego".to_string(), 
            last_name: "Manuel".to_string()
        };

    
        let action1 = Action::Put(person1.serialize());
        let action2 = Action::Put(person2.serialize());

        let response = Crux::new("localhost", "4000").client().tx_log(vec![action1, action2]);

        assert_eq!(response.unwrap(), TxLogResponse::default())
    }

    #[test]
    fn tx_logs() {
        let _m = mock("GET", "/tx-log")
        .with_status(200)
        .with_header("content-type", "text/plain")
        .with_body("({:crux.tx/tx-id 0, :crux.tx/tx-time #inst \"2020-07-09T23:38:06.465-00:00\", :crux.tx.event/tx-events [[:crux.tx/put \"a15f8b81a160b4eebe5c84e9e3b65c87b9b2f18e\" \"125d29eb3bed1bf51d64194601ad4ff93defe0e2\"]]}{:crux.tx/tx-id 1, :crux.tx/tx-time #inst \"2020-07-09T23:39:33.815-00:00\", :crux.tx.event/tx-events [[:crux.tx/put \"a15f8b81a160b4eebe5c84e9e3b65c87b9b2f18e\" \"1b42e0d5137e3833423f7bb958622bee29f91eee\"]]})")
        .create();

        let response = Crux::new("localhost", "4000").client().tx_logs();

        assert_eq!(response.unwrap().tx_events.len(), 2);    
    }
}

