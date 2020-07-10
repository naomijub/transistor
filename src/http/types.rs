use edn_rs::{
    ser_struct,
    serialize::Serialize
};

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

impl StateResponse {
    pub fn deserialize(resp: String) -> StateResponse {
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
    pub fn default() -> StateResponse{
        StateResponse {
            index__index_version: 5usize,
            doc_log__consumer_state: Some(String::from("nil")),
            tx_log__consumer_state:  Some(String::from("nil")),
            kv__kv_store: String::from("crux.kv.rocksdb.RocksKv"),
            kv__estimate_num_keys: 34usize,
            kv__size: 88489usize,
        }
    }
}

impl From<std::collections::HashMap<String,String>> for StateResponse {
    fn from(hm: std::collections::HashMap<String,String>) -> Self {
        StateResponse {
            index__index_version: hm[":crux.index/index-version"].parse::<usize>().unwrap_or(0usize),
            doc_log__consumer_state: nullable_str(hm[":crux.doc-log/consumer-state"].clone()),
            tx_log__consumer_state:  nullable_str(hm[":crux.tx-log/consumer-state"].clone()),
            kv__kv_store: hm[":crux.kv/kv-store"].clone().replace("\"", ""),
            kv__estimate_num_keys: hm[":crux.kv/estimate-num-keys"].parse::<usize>().unwrap_or(0usize),
            kv__size: hm[":crux.kv/size"].parse::<usize>().unwrap_or(0usize),
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