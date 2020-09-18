use crate::types::error::CruxError;
use chrono::prelude::*;
use edn_rs::{Deserialize, Edn, EdnError};
use std::collections::BTreeSet;
use std::str::FromStr;

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

impl Deserialize for TxLogResponse {
    fn deserialize(edn: &Edn) -> Result<Self, EdnError> {
        #[cfg(not(feature = "time_as_str"))]
        let tx_time: String = edn_rs::from_edn(&edn[":crux.tx/tx-time"])?;

        Ok(Self {
            tx___tx_id: edn_rs::from_edn(&edn[":crux.tx/tx-id"]).unwrap_or(0usize),
            #[cfg(feature = "time_as_str")]
            tx___tx_time: edn_rs::from_edn(&edn[":crux.tx/tx-time"])?,
            #[cfg(not(feature = "time_as_str"))]
            tx___tx_time: tx_time
                .parse::<DateTime<FixedOffset>>()
                .map_err(|_| 
                    EdnError::Deserialize("Unable to deserialize `:crux.tx/tx-time`, verify if the transaction time you're sending is correct".to_string())
                )?,
            tx__event___tx_events: edn_rs::from_edn(&edn[":crux.tx.event/tx-events"])?,
        })
    }
}

impl TxLogResponse {
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

impl FromStr for TxLogsResponse {
    type Err = CruxError;
    fn from_str(resp: &str) -> Result<Self, CruxError> {
        let clean_edn = resp.replace("#crux/id", "").replace("#inst", "");
        edn_rs::from_str(&clean_edn).map_err(|e| e.into())
    }
}

impl Deserialize for TxLogsResponse {
    fn deserialize(edn: &Edn) -> Result<Self, EdnError> {
        Ok(Self {
            tx_events: edn
                .iter()
                .ok_or(EdnError::Deserialize(format!(
                    "The following Edn cannot be deserialized to TxLogs: {:?}",
                    edn
                )))?
                .map(edn_rs::from_edn)
                .collect::<Result<Vec<TxLogResponse>, EdnError>>()?,
        })
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

impl FromStr for EntityTxResponse {
    type Err = CruxError;
    fn from_str(resp: &str) -> Result<Self, CruxError> {
        let clean_edn = resp.replace("#crux/id", "");
        edn_rs::from_str(&clean_edn).map_err(|e| e.into())
    }
}

impl EntityTxResponse {
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

impl Deserialize for EntityTxResponse {
    fn deserialize(edn: &Edn) -> Result<Self, EdnError> {
        #[cfg(not(feature = "time_as_str"))]
        let valid_time: String = edn_rs::from_edn(&edn[":crux.db/valid-time"])?;
        #[cfg(not(feature = "time_as_str"))]
        let tx_time: String = edn_rs::from_edn(&edn[":crux.tx/tx-time"])?;

        Ok(Self {
            db___id: edn_rs::from_edn(&edn[":crux.db/id"])?,
            db___content_hash: edn_rs::from_edn(&edn[":crux.db/content-hash"])?,
            #[cfg(feature = "time_as_str")]
            db___valid_time: edn_rs::from_edn(&edn[":crux.db/valid-time"]),
            #[cfg(not(feature = "time_as_str"))]
            db___valid_time: valid_time.parse::<DateTime<FixedOffset>>().unwrap(),
            tx___tx_id: edn_rs::from_edn(&edn[":crux.tx/tx-id"]).unwrap_or(0usize),
            #[cfg(feature = "time_as_str")]
            tx___tx_time: edn_rs::from_edn(&edn[":crux.tx/tx-time"]),
            #[cfg(not(feature = "time_as_str"))]
            tx___tx_time: tx_time.parse::<DateTime<FixedOffset>>().unwrap(),
        })
    }
}

#[doc(hidden)]
#[cfg(not(feature = "async"))]
pub(crate) struct QueryResponse(pub(crate) BTreeSet<Vec<String>>);

#[cfg(not(feature = "async"))]
impl Deserialize for QueryResponse {
    fn deserialize(edn: &Edn) -> Result<Self, EdnError> {
        if edn.set_iter().is_some() {
            Ok(Self(
                edn.set_iter()
                    .ok_or(EdnError::Deserialize(format!(
                        "The following Edn cannot be deserialized to BTreeSet: {:?}",
                        edn
                    )))?
                    .map(|e| {
                        e.to_vec().ok_or(EdnError::Deserialize(format!(
                            "The following Edn cannot be deserialized to Vec: {:?}",
                            edn
                        )))
                    })
                    .collect::<Result<BTreeSet<Vec<String>>, EdnError>>()?,
            ))
        } else {
            Ok(Self(
                edn.iter()
                    .ok_or(EdnError::Deserialize(format!(
                        "The following Edn cannot be deserialized to BTreeSet: {:?}",
                        edn
                    )))?
                    .map(|e| {
                        e.to_vec().ok_or(EdnError::Deserialize(format!(
                            "The following Edn cannot be deserialized to Vec: {:?}",
                            edn
                        )))
                    })
                    .collect::<Result<BTreeSet<Vec<String>>, EdnError>>()?,
            ))
        }
    }
}

#[cfg(feature = "async")]
#[derive(Clone, Debug, PartialEq)]
/// When feature `async` is enabled this is the response type for endpoint `/query`.
pub struct QueryAsyncResponse(pub(crate) BTreeSet<Vec<String>>);

#[cfg(feature = "async")]
impl Deserialize for QueryAsyncResponse {
    fn deserialize(edn: &Edn) -> Result<Self, EdnError> {
        if edn.set_iter().is_some() {
            Ok(Self {
                0: edn
                    .set_iter()
                    .ok_or(EdnError::Deserialize(format!(
                        "The following Edn cannot be deserialize to BTreeSet: {:?}",
                        edn
                    )))?
                    .map(|e| {
                        e.to_vec().ok_or(EdnError::Deserialize(format!(
                            "The following Edn cannot be deserialized to Vec: {:?}",
                            edn
                        )))
                    })
                    .collect::<Result<BTreeSet<Vec<String>>, EdnError>>()?,
            })
        } else {
            Ok(Self {
                0: edn
                    .iter()
                    .ok_or(EdnError::Deserialize(format!(
                        "The following Edn cannot be deserialize to BTreeSet: {:?}",
                        edn
                    )))?
                    .map(|e| {
                        e.to_vec().ok_or(EdnError::Deserialize(format!(
                            "The following Edn cannot be deserialized to Vec: {:?}",
                            edn
                        )))
                    })
                    .collect::<Result<BTreeSet<Vec<String>>, EdnError>>()?,
            })
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

impl Deserialize for EntityHistoryElement {
    fn deserialize(edn: &Edn) -> Result<Self, EdnError> {
        #[cfg(not(feature = "time_as_str"))]
        let valid_time: String = edn_rs::from_edn(&edn[":crux.db/valid-time"])?;
        #[cfg(not(feature = "time_as_str"))]
        let tx_time: String = edn_rs::from_edn(&edn[":crux.tx/tx-time"])?;

        Ok(Self {
            db___content_hash: edn_rs::from_edn(&edn[":crux.db/content-hash"])?,
            #[cfg(feature = "time_as_str")]
            db___valid_time: edn_rs::from_edn(&edn[":crux.db/valid-time"])?,
            #[cfg(not(feature = "time_as_str"))]
            db___valid_time: valid_time.parse::<DateTime<FixedOffset>>().unwrap(),
            tx___tx_id: edn_rs::from_edn(&edn[":crux.tx/tx-id"]).unwrap_or(0usize),
            #[cfg(feature = "time_as_str")]
            tx___tx_time: edn_rs::from_edn(&edn[":crux.tx/tx-time"])?,
            #[cfg(not(feature = "time_as_str"))]
            tx___tx_time: tx_time.parse::<DateTime<FixedOffset>>().unwrap(),
            db__doc: edn.get(":crux.db/doc").map(|d| d.to_owned()),
        })
    }
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

/// Definition for the response of a `GET` at `/entity-history` endpoint. This returns a Vec of  `EntityHistoryElement`.
#[derive(Debug, PartialEq, Clone)]
pub struct EntityHistoryResponse {
    pub history: Vec<EntityHistoryElement>,
}

impl FromStr for EntityHistoryResponse {
    type Err = CruxError;
    fn from_str(resp: &str) -> Result<Self, CruxError> {
        let clean_edn = resp.replace("#crux/id", "").replace("#inst", "");
        edn_rs::from_str(&clean_edn).map_err(|e| e.into())
    }
}

impl Deserialize for EntityHistoryResponse {
    fn deserialize(edn: &Edn) -> Result<Self, EdnError> {
        Ok(Self {
            history: edn
                .iter()
                .ok_or(EdnError::Deserialize(format!(
                    "The following Edn cannot be deserialize to entity-history: {:?}",
                    edn
                )))?
                .map(edn_rs::from_edn)
                .collect::<Result<Vec<EntityHistoryElement>, EdnError>>()?,
        })
    }
}
