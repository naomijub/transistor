use edn_rs::Serialize;

/// Action to perform in Crux. Receives a serialized Edn.
///
/// **First field of your struct should be `crux__db___id: CruxId`**
///
/// Allowed actions:
/// * `PUT` - Write a version of a document
/// * `Delete` - Deletes the specific document at a given valid time
/// * `Evict` - Evicts a document entirely, including all historical versions (receives only the ID to evict)
#[derive(Debug, PartialEq)]
pub enum Action {
    Put(String),
    Delete(String),
    Evict(String),
}

impl Serialize for Action {
    fn serialize(self) -> String {
        match self {
            Action::Put(edn) => format!("[:crux.tx/put {}]", edn),
            Action::Delete(edn) => format!("[:crux.tx/delete {}]", edn),
            Action::Evict(id) => {
                if id.starts_with(":") {
                    format!("[:crux.tx/evict {}]", id)
                } else {
                    "".to_string()
                }
            }
        }
    }
}

/// `Order` enum to define how the `entity_history` response will be ordered. Options are `Asc` and `Desc`.
#[derive(Debug, PartialEq)]
pub enum Order {
    Asc,
    Desc,
}

impl Serialize for Order {
    fn serialize(self) -> String {
        match self {
            Order::Asc => String::from("asc"),
            Order::Desc => String::from("desc"),
        }
    }
}

#[cfg(feature = "time")]
pub mod time {
    use chrono::prelude::*;
    use edn_rs::Serialize;

    #[derive(Debug, PartialEq)]
    pub enum TimeHistory {
        ValidTime(Option<DateTime<Utc>>, Option<DateTime<Utc>>),
        TransactionTime(Option<DateTime<Utc>>, Option<DateTime<Utc>>),
    }

    impl Serialize for TimeHistory {
        fn serialize(self) -> String {
            use crate::types::http::time::TimeHistory::TransactionTime;
            use crate::types::http::time::TimeHistory::ValidTime;

            match self {
                ValidTime(Some(start), Some(end)) => format!(
                    "&start-valid-time={}&end-valid-time={}",
                    start.format("%Y-%m-%dT%H:%M:%S").to_string(),
                    end.format("%Y-%m-%dT%H:%M:%S").to_string()
                ),
                ValidTime(None, Some(end)) => format!(
                    "&end-valid-time={}",
                    end.format("%Y-%m-%dT%H:%M:%S").to_string()
                ),
                ValidTime(Some(start), None) => format!(
                    "&start-valid-time={}",
                    start.format("%Y-%m-%dT%H:%M:%S").to_string()
                ),
                ValidTime(None, None) => format!(""),

                TransactionTime(Some(start), Some(end)) => format!(
                    "&start-transaction-time={}&end-transaction-time={}",
                    start.format("%Y-%m-%dT%H:%M:%S").to_string(),
                    end.format("%Y-%m-%dT%H:%M:%S").to_string()
                ),
                TransactionTime(None, Some(end)) => format!(
                    "&end-transaction-time={}",
                    end.format("%Y-%m-%dT%H:%M:%S").to_string()
                ),
                TransactionTime(Some(start), None) => format!(
                    "&start-transaction-time={}",
                    start.format("%Y-%m-%dT%H:%M:%S").to_string()
                ),
                TransactionTime(None, None) => format!(""),
            }
        }
    }

    pub trait VecSer {
        fn serialize(self) -> String;
    }

    impl VecSer for Vec<TimeHistory> {
        fn serialize(self) -> String {
            if self.len() > 2 || self.len() == 0 {
                String::new()
            } else {
                self.into_iter()
                    .map(|e| e.serialize())
                    .collect::<Vec<String>>()
                    .join("")
            }
        }
    }
}
