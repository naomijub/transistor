use reqwest::{
    header::{HeaderMap,AUTHORIZATION},
    blocking::{Client}, 
    Result
};
use edn_rs::{
    ser_struct,
    serialize::Serialize
};

pub struct Crux {
    host: String,
    port: String,
    headers: HeaderMap
}

impl Crux{
    pub fn new(host: &str, port: &str) -> Self {
        Self{host: host.to_string(), port: port.to_string(), headers: HeaderMap::new()}
    }

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

    pub fn client(&self) -> CruxClient {
        CruxClient {
            client: reqwest::blocking::Client::new(),
            uri: self.uri().clone(),
            headers: self.headers.clone()
        }
    }
}

pub struct CruxClient {
    client: Client,
    uri: String, 
    headers: HeaderMap,
}

impl CruxClient {
    pub fn state(&self) -> Result<CruxState> {
        let resp = self.client.get(&self.uri)
            .headers(self.headers.clone())
            .send()?
            .text()?;
        Ok(CruxState::deserialize(resp))
    }
}

ser_struct!{
    #[derive(Debug, PartialEq)]
    #[allow(non_snake_case)]
    pub struct CruxState {
        ndex__index_version: usize,
        doc_log__consumer_state: String,
        tx_log__consumer_state:  String,
        kv__kv_store: String,
        kv__estimate_num_keys: usize,
        kv__size: usize
    }
}

impl CruxState {
    fn deserialize(resp: String) -> CruxState {
        use std::collections::HashMap;
        let mut hashmap = HashMap::new();
        let data = resp.replace("{","").replace("}","");
        let key_val = data.split(", ").collect::<Vec<&str>>().iter()
            .map(|v| v.split(" ").collect::<Vec<&str>>())
            .map(|v| (v[0], v[1]))
            .collect::<Vec<(&str, &str)>>();
        for (key, val) in key_val.iter() {
            hashmap.insert(String::from(*key), String::from(*val));
        }

        hashmap.into()
    }

    #[cfg(test)]
    fn default() -> CruxState{
        CruxState {
            ndex__index_version: 5usize,
            doc_log__consumer_state: String::from("nil"),
            tx_log__consumer_state:  String::from("nil"),
            kv__kv_store: String::from("crux.kv.rocksdb.RocksKv"),
            kv__estimate_num_keys: 34usize,
            kv__size: 88489usize,
        }
    }
}

impl From<std::collections::HashMap<String,String>> for CruxState {
    fn from(hm: std::collections::HashMap<String,String>) -> Self {
        CruxState {
            ndex__index_version: hm[":crux.index/index-version"].parse::<usize>().unwrap_or(0usize),
            doc_log__consumer_state: hm[":crux.doc-log/consumer-state"].clone(),
            tx_log__consumer_state:  hm[":crux.tx-log/consumer-state"].clone(),
            kv__kv_store: hm[":crux.kv/kv-store"].clone().replace("\"", ""),
            kv__estimate_num_keys: hm[":crux.kv/estimate-num-keys"].parse::<usize>().unwrap_or(0usize),
            kv__size: hm[":crux.kv/size"].parse::<usize>().unwrap_or(0usize),
        }
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
    use super::{Crux, CruxState};
    use mockito::mock;

    #[test]
    fn state() {
        let _m = mock("GET", "/")
        .with_status(200)
        .with_header("content-type", "text/plain")
        .with_body("{:crux.index/index-version 5, :crux.doc-log/consumer-state nil, :crux.tx-log/consumer-state nil, :crux.kv/kv-store \"crux.kv.rocksdb.RocksKv\", :crux.kv/estimate-num-keys 34, :crux.kv/size 88489}")
        .create();

        let response = Crux::new("localhost", "4000").client().state();

        assert_eq!(response.unwrap(), CruxState::default())
    }
}