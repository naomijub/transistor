use edn_rs::{
    ser_struct,
    Serialize,
    parse_edn,
    Edn
};
use std::collections::BTreeMap;

/// Id to use as reference in Crux, similar to `ids` with `Uuid`. This id is supposed to be a KEYWORD, `Edn::Key`.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct CruxId(String);

impl Serialize for CruxId {
    fn serialize(mut self) -> String {
        self.0.insert(0, ':');
        format!("{}", self.0)
    }
}

impl CruxId {
    pub fn new(id: &str) -> Self {
        Self {0: id.to_string()}
    }
}

ser_struct!{
    #[derive(Debug, PartialEq, Clone)]
    #[allow(non_snake_case)]
    /// Definition for the response of a `GET` at `state` endpoint
    pub struct StateResponse {
        index___index_version: usize,
        doc_log___consumer_state: Option<String>,
        tx_log___consumer_state:  Option<String>,
        kv___kv_store: String,
        kv___estimate_num_keys: usize,
        kv___size: usize
    }
}

impl StateResponse {
    pub fn deserialize(resp: String) -> Self {
        let edn = parse_edn(&resp).unwrap();
        Self {
            index___index_version: edn[":crux.index/index-version"].to_uint().unwrap_or(0usize),
            doc_log___consumer_state: nullable_str(edn[":crux.doc-log/consumer-state"].to_string()),
            tx_log___consumer_state:  nullable_str(edn[":crux.tx-log/consumer-state"].to_string()),
            kv___kv_store: edn[":crux.kv/kv-store"].to_string().replace("\"", ""),
            kv___estimate_num_keys: edn[":crux.kv/estimate-num-keys"].to_uint().unwrap_or(0usize),
            kv___size: edn[":crux.kv/size"].to_uint().unwrap_or(0usize),
        }
    }

    #[cfg(test)]
    pub fn default() -> Self{
        Self {
            index___index_version: 5usize,
            doc_log___consumer_state: None,
            tx_log___consumer_state:  None,
            kv___kv_store: String::from("crux.kv.rocksdb.RocksKv"),
            kv___estimate_num_keys: 34usize,
            kv___size: 88489usize,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
#[allow(non_snake_case)]
/// Definition for the response of a `POST` at `tx-log` endpoint
pub struct TxLogResponse {
    pub tx___tx_id: usize, 
    pub tx___tx_time: String,
    pub tx__event___tx_events: Option<Vec<Vec<String>>>
}


impl TxLogResponse {
    pub fn deserialize(resp: String) -> Self {
        let edn = parse_edn(&resp).unwrap();
        edn.into()
    }

    #[cfg(test)]
    pub fn default() -> Self {
        Self {
            tx___tx_id: 8usize, 
            tx___tx_time: "2020-07-16T21:53:14.628-00:00".to_string(),
            tx__event___tx_events: None
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
#[allow(non_snake_case)]
/// Definition for the response of a `GET` at `tx-log` endpoint
pub struct TxLogsResponse {
    pub tx_events: Vec<TxLogResponse>,
}

impl TxLogsResponse {
    pub fn deserialize(resp: String) -> Self {
        let edn = parse_edn(&resp).unwrap();
        edn.into()
    }
}

impl From<Edn> for TxLogsResponse {
    fn from(edn: Edn) -> Self {
        Self {
            tx_events: edn.iter().unwrap()
                .map(|e| 
                    e.to_owned().into()
                )
                .collect::<Vec<TxLogResponse>>()
        }
    }
}

impl From<Edn> for TxLogResponse {
    fn from(edn: Edn) -> Self {
        Self {
            tx___tx_id: edn[":crux.tx/tx-id"].to_uint().unwrap_or(0usize), 
            tx___tx_time: edn[":crux.tx/tx-time"].to_string(),
            tx__event___tx_events: edn.get(":crux.tx.event/tx-events").map(|e| e.iter().unwrap().map(|el| el.to_vec().unwrap()).collect::<Vec<Vec<String>>>())
        }
    }
}


#[derive(Debug, PartialEq, Clone)]
#[allow(non_snake_case)]
/// Definition for the response of a `POST` at `/entity-tx` endpoint
pub struct EntityTxResponse {
    pub db___id: String,
    pub db___content_hash: String,
    pub db___valid_time: String,
    pub tx___tx_id: usize,
    pub tx___tx_time: String
}

impl EntityTxResponse {
    pub fn deserialize(resp: String) -> Self {
        let edn = parse_edn(&resp).unwrap();
        edn.into()
    }

    #[cfg(test)]
    pub fn default() -> Self { 
        Self{
            db___id: "d72ccae848ce3a371bd313865cedc3d20b1478ca".to_string(),
            db___content_hash: "1828ebf4466f98ea3f5252a58734208cd0414376".to_string(),
            db___valid_time: "2020-07-19T04:12:13.788-00:00".to_string(),
            tx___tx_id: 28usize,
            tx___tx_time: "2020-07-19T04:12:13.788-00:00".to_string(),
        }
    }
}

impl From<Edn> for EntityTxResponse {
    fn from(edn: Edn) -> Self {
        Self {
            db___id: edn[":crux.db/id"].to_string(),
            db___content_hash: edn[":crux.db/content-hash"].to_string(),
            db___valid_time: edn[":crux.db/valid-time"].to_string(),
            tx___tx_id: edn[":crux.tx/tx-id"].to_uint().unwrap_or(0usize),
            tx___tx_time: edn[":crux.tx/tx-time"].to_string(),
        }
    }
}

pub (crate) struct Documents;

impl Documents {
    pub fn deserialize(resp: String, hashes: Vec<String>) -> BTreeMap<String, Edn> {
        let edn = parse_edn(&resp).unwrap();

        let mut hm = BTreeMap::new();
        hashes.iter().for_each(|h| {
            let edn_value = edn[h].clone();
            hm.insert(String::from(h), edn_value);
        });

        hm
    }
}

fn nullable_str(s: String) -> Option<String> {
    if s.contains("nil") {
        None
    } else {
        Some(s)
    }
}

