use edn_rs::{
    ser_struct,
    serialize::Serialize
};
use grok::Grok;

ser_struct!{
    #[derive(Debug, PartialEq)]
    #[allow(non_snake_case)]
    /// Definition for the response of the `state` endpoint
    pub struct StateResponse {
        index__index_version: usize,
        doc_log__consumer_state: Option<String>,
        tx_log__consumer_state:  Option<String>,
        kv__kv_store: String,
        kv__estimate_num_keys: usize,
        kv__size: usize
    }
}

#[derive(Debug)]
pub enum Error {
    ParseError(String),
    RequestError(String),
}

impl StateResponse {
    pub fn deserialize(edn: String) -> Result<StateResponse, Error> {
        let mut grok = Grok::default();

        grok.insert_definition("ALL", r"(?=.*:crux.tx-log/consumer-state %{TX_CONSUMER_STATE})(?=.*:crux.index/index-version %{VERSION})(?=.*:crux.doc-log/consumer-state %{DOC_CONSUMER_STATE})(?=.*:crux.kv/kv-store %{KV_STORE})(?=.*:crux.kv/estimate-num-keys %{NUM_KEYS})(?=.*:crux.kv/size %{SIZE})");
        grok.insert_definition("VERSION", r"[0-9]+");
        grok.insert_definition("KV_STORE", r#"\"(\w+[\./]*)+\""#);
        grok.insert_definition("DOC_CONSUMER_STATE", r"{.*?}|nil");
        grok.insert_definition("TX_CONSUMER_STATE", r"{.*?}|nil");
        grok.insert_definition("NUM_KEYS", r"[0-9]+");
        grok.insert_definition("SIZE", r"[0-9]+");

        let pattern = grok.compile("%{ALL}", false)
            .expect("Error while compiling!");

        match pattern.match_against(&edn) {
            Some(m) => {
                Ok(StateResponse {
                    index__index_version: m
                        .get("VERSION")
                        .ok_or_else(|| Error::ParseError(":crux.index/index-version not found".to_string()))?
                        .parse::<usize>()
                        .map_err(|_| Error::ParseError(":crux.index/index-version not a number".to_string()))?,
                    doc_log__consumer_state: m
                        .get("DOC_CONSUMER_STATE")
                        .map(ToString::to_string)
                        .and_then(nullable_str),
                    tx_log__consumer_state: m
                        .get("TX_CONSUMER_STATE")
                        .map(ToString::to_string)
                        .and_then(nullable_str),
                    kv__kv_store: m
                        .get("KV_STORE")
                        .ok_or_else(|| Error::ParseError(":crux.kv/kv-store not found".to_string()))?
                        .to_string()
                        .replace("\"", ""),
                    kv__estimate_num_keys: m
                        .get("NUM_KEYS")
                        .ok_or_else(|| Error::ParseError(":crux.kv/estimate-num-keys not found".to_string()))?
                        .parse::<usize>()
                        .map_err(|_| Error::ParseError(":crux.kv/estimate-num-keys not a number".to_string()))?,
                    kv__size: m
                        .get("SIZE")
                        .ok_or_else(|| Error::ParseError(":crux.kv/size not found".to_string()))?
                        .parse::<usize>()
                        .map_err(|_| Error::ParseError(":crux.kv/size not a number".to_string()))?,
                })
            },
            None => Err(Error::ParseError("Failed to deserialize StateResponse".to_string())),
        }
    }

    #[cfg(test)]
    pub fn default() -> StateResponse{
        StateResponse {
            index__index_version: 5usize,
            doc_log__consumer_state: None,
            tx_log__consumer_state:  None,
            kv__kv_store: String::from("crux.kv.rocksdb.RocksKv"),
            kv__estimate_num_keys: 34usize,
            kv__size: 88489usize,
        }
    }
}

fn nullable_str(s: String) -> Option<String> {
    if s.contains("nil") {
        None
    } else {
        Some(s)
    }
}
