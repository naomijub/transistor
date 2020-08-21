use crate::types::error::CruxError;
use chrono::prelude::*;
use edn_rs::{from_str, Edn};
use std::collections::BTreeSet;

#[derive(Debug, PartialEq, Clone)]
#[allow(non_snake_case)]
/// Definition for the response of a `POST` at `tx-log` endpoint
pub struct TxLogResponse {
    pub tx___tx_id: usize,
    #[cfg(feature = "time_as_str")]
    pub tx___tx_time: String,
    #[cfg(not(feature = "time_as_str"))]
    pub tx___tx_time: DateTime<FixedOffset>,
    pub tx__event___tx_events: Option<Vec<Vec<String>>>,
}

impl TxLogResponse {
    pub fn deserialize(resp: String) -> Result<Self, CruxError> {
        let edn = from_str(&resp)?;
        Ok(edn.into())
    }

    #[cfg(test)]
    pub fn default() -> Self {
        Self {
            tx___tx_id: 8usize,
            tx___tx_time: "2020-07-16T21:53:14.628-00:00"
                .parse::<DateTime<FixedOffset>>()
                .unwrap(),
            tx__event___tx_events: None,
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
    pub fn deserialize(resp: String) -> Result<Self, CruxError> {
        let clean_edn = resp.replace("#crux/id", "");
        let edn = from_str(&clean_edn)?;
        Ok(edn.into())
    }
}

impl From<Edn> for TxLogsResponse {
    fn from(edn: Edn) -> Self {
        Self {
            tx_events: edn
                .iter()
                .ok_or(CruxError::ParseEdnError(format!(
                    "The following Edn cannot be parsed to TxLogs: {:?}",
                    edn
                )))
                .unwrap()
                .map(|e| e.to_owned().into())
                .collect::<Vec<TxLogResponse>>(),
        }
    }
}

impl From<Edn> for TxLogResponse {
    fn from(edn: Edn) -> Self {
        Self {
            tx___tx_id: edn[":crux.tx/tx-id"].to_uint().unwrap_or(0usize),
            #[cfg(feature = "time_as_str")]
            tx___tx_time: edn[":crux.tx/tx-time"].to_string(),
            #[cfg(not(feature = "time_as_str"))]
            tx___tx_time: edn[":crux.tx/tx-time"]
                .to_string()
                .parse::<DateTime<FixedOffset>>()
                .unwrap(),
            tx__event___tx_events: edn.get(":crux.tx.event/tx-events").map(|e| {
                e.iter()
                    .ok_or(CruxError::ParseEdnError(format!(
                        "The following Edn cannot be parsed to TxLog: {:?}",
                        edn
                    )))
                    .unwrap()
                    .map(|el| el.to_vec().unwrap())
                    .collect::<Vec<Vec<String>>>()
            }),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
#[allow(non_snake_case)]
/// Definition for the response of a `POST` at `/entity-tx` endpoint
pub struct EntityTxResponse {
    pub db___id: String,
    pub db___content_hash: String,
    #[cfg(feature = "time_as_str")]
    pub db___valid_time: String,
    #[cfg(not(feature = "time_as_str"))]
    pub db___valid_time: DateTime<FixedOffset>,
    pub tx___tx_id: usize,
    #[cfg(feature = "time_as_str")]
    pub tx___tx_time: String,
    #[cfg(not(feature = "time_as_str"))]
    pub tx___tx_time: DateTime<FixedOffset>,
}

impl EntityTxResponse {
    pub fn deserialize(resp: String) -> Result<Self, CruxError> {
        let clean_edn = resp.replace("#crux/id", "");
        let edn = from_str(&clean_edn)?;
        Ok(edn.into())
    }

    #[cfg(test)]
    pub fn default() -> Self {
        Self {
            db___id: "d72ccae848ce3a371bd313865cedc3d20b1478ca".to_string(),
            db___content_hash: "1828ebf4466f98ea3f5252a58734208cd0414376".to_string(),
            db___valid_time: "2020-07-19T04:12:13.788-00:00"
                .parse::<DateTime<FixedOffset>>()
                .unwrap(),
            tx___tx_id: 28usize,
            tx___tx_time: "2020-07-19T04:12:13.788-00:00"
                .parse::<DateTime<FixedOffset>>()
                .unwrap(),
        }
    }
}

impl From<Edn> for EntityTxResponse {
    fn from(edn: Edn) -> Self {
        Self {
            db___id: edn[":crux.db/id"].to_string(),
            db___content_hash: edn[":crux.db/content-hash"].to_string(),
            #[cfg(feature = "time_as_str")]
            db___valid_time: edn[":crux.db/valid-time"].to_string(),
            #[cfg(not(feature = "time_as_str"))]
            db___valid_time: edn[":crux.db/valid-time"]
                .to_string()
                .parse::<DateTime<FixedOffset>>()
                .unwrap(),
            tx___tx_id: edn[":crux.tx/tx-id"].to_uint().unwrap_or(0usize),
            #[cfg(feature = "time_as_str")]
            tx___tx_time: edn[":crux.tx/tx-time"].to_string(),
            #[cfg(not(feature = "time_as_str"))]
            tx___tx_time: edn[":crux.tx/tx-time"]
                .to_string()
                .parse::<DateTime<FixedOffset>>()
                .unwrap(),
        }
    }
}

#[doc(hidden)]
#[cfg(not(feature = "async"))]
pub(crate) struct QueryResponse;

#[cfg(not(feature = "async"))]
impl QueryResponse {
    pub(crate) fn deserialize(resp: String) -> Result<BTreeSet<Vec<String>>, CruxError> {
        let edn = from_str(&resp.clone())?;
        if edn.set_iter().is_some() {
            Ok(edn
                .set_iter()
                .ok_or(CruxError::ParseEdnError(format!(
                    "The following Edn cannot be parsed to BTreeSet: {:?}",
                    edn
                )))?
                .map(|e| e.to_vec().unwrap())
                .collect::<BTreeSet<Vec<String>>>())
        } else {
            Ok(edn
                .iter()
                .ok_or(CruxError::ParseEdnError(format!(
                    "The following Edn cannot be parsed to BTreeSet: {:?}",
                    edn
                )))?
                .map(|e| e.to_vec().unwrap())
                .collect::<BTreeSet<Vec<String>>>())
        }
    }
}

#[cfg(feature = "async")]
#[derive(Clone, Debug, PartialEq)]
/// When feature `async` is enabled this is the response type for endpoint `/query`.
pub struct QueryAsyncResponse(BTreeSet<Vec<String>>);

#[cfg(feature = "async")]
impl QueryAsyncResponse {
    pub(crate) fn deserialize(resp: String) -> QueryAsyncResponse {
        let edn = from_str(&resp.clone()).unwrap();
        if edn.set_iter().is_some() {
            QueryAsyncResponse {
                0: edn
                    .set_iter()
                    .ok_or(CruxError::ParseEdnError(format!(
                        "The following Edn cannot be parsed to BTreeSet: {:?}",
                        edn
                    )))
                    .unwrap()
                    .map(|e| e.to_vec().unwrap())
                    .collect::<BTreeSet<Vec<String>>>(),
            }
        } else {
            QueryAsyncResponse {
                0: edn
                    .iter()
                    .ok_or(CruxError::ParseEdnError(format!(
                        "The following Edn cannot be parsed to BTreeSet: {:?}",
                        edn
                    )))
                    .unwrap()
                    .map(|e| e.to_vec().unwrap())
                    .collect::<BTreeSet<Vec<String>>>(),
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
#[allow(non_snake_case)]
pub struct EntityHistoryElement {
    #[cfg(feature = "time_as_str")]
    pub db___valid_time: String,
    #[cfg(not(feature = "time_as_str"))]
    pub db___valid_time: DateTime<FixedOffset>,
    pub tx___tx_id: usize,
    #[cfg(feature = "time_as_str")]
    pub tx___tx_time: String,
    #[cfg(not(feature = "time_as_str"))]
    pub tx___tx_time: DateTime<FixedOffset>,
    pub db___content_hash: String,
    pub db__doc: Option<Edn>,
}

#[cfg(test)]
impl EntityHistoryElement {
    pub fn default() -> Self {
        Self {
            db___content_hash: "1828ebf4466f98ea3f5252a58734208cd0414376".to_string(),
            db___valid_time: "2020-07-19T04:12:13.788-00:00"
                .parse::<DateTime<FixedOffset>>()
                .unwrap(),
            tx___tx_id: 28usize,
            tx___tx_time: "2020-07-19T04:12:13.788-00:00"
                .parse::<DateTime<FixedOffset>>()
                .unwrap(),
            db__doc: None,
        }
    }

    pub fn default_docs() -> Self {
        Self {
            db___content_hash: "1828ebf4466f98ea3f5252a58734208cd0414376".to_string(),
            db___valid_time: "2020-07-19T04:12:13.788-00:00"
                .parse::<DateTime<FixedOffset>>()
                .unwrap(),
            tx___tx_id: 28usize,
            tx___tx_time: "2020-07-19T04:12:13.788-00:00"
                .parse::<DateTime<FixedOffset>>()
                .unwrap(),
            db__doc: Some(Edn::Key(":docs".to_string())),
        }
    }
}

impl From<Edn> for EntityHistoryElement {
    fn from(edn: Edn) -> Self {
        Self {
            db___content_hash: edn[":crux.db/content-hash"].to_string(),
            #[cfg(feature = "time_as_str")]
            db___valid_time: edn[":crux.db/valid-time"].to_string(),
            #[cfg(not(feature = "time_as_str"))]
            db___valid_time: edn[":crux.db/valid-time"]
                .to_string()
                .parse::<DateTime<FixedOffset>>()
                .unwrap(),
            tx___tx_id: edn[":crux.tx/tx-id"].to_uint().unwrap_or(0usize),
            #[cfg(feature = "time_as_str")]
            tx___tx_time: edn[":crux.tx/tx-time"].to_string(),
            #[cfg(not(feature = "time_as_str"))]
            tx___tx_time: edn[":crux.tx/tx-time"]
                .to_string()
                .parse::<DateTime<FixedOffset>>()
                .unwrap(),
            db__doc: edn.get(":crux.db/doc").map(|d| d.to_owned()),
        }
    }
}

/// Definition for the response of a `GET` at `/entity-history` endpoint. This returns a Vec of  `EntityHistoryElement`.
#[derive(Debug, PartialEq, Clone)]
pub struct EntityHistoryResponse {
    pub history: Vec<EntityHistoryElement>,
}

impl EntityHistoryResponse {
    pub fn deserialize(resp: String) -> Result<Self, CruxError> {
        let clean_edn = resp.replace("#crux/id", "").replace("#inst", "");
        let edn = from_str(&clean_edn)?;
        Ok(edn.into())
    }
}

impl From<Edn> for EntityHistoryResponse {
    fn from(edn: Edn) -> Self {
        Self {
            history: edn
                .iter()
                .ok_or(CruxError::ParseEdnError(format!(
                    "The following Edn cannot be parsed to entity-history: {:?}",
                    edn
                )))
                .unwrap()
                .map(|el| el.to_owned().into())
                .collect::<Vec<EntityHistoryElement>>(),
        }
    }
}
