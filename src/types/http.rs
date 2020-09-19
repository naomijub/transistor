use chrono::prelude::*;
use edn_rs::Serialize;
static ACTION_DATE_FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S%Z";
static DATETIME_FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S";

/// Action to perform in Crux. Receives a serialized Edn.
///
/// **First field of your struct should be `crux__db___id: CruxId`**
///
/// Allowed actions:
/// * `PUT` - Write a version of a document can receive an `Option<DateTime<FixedOffset>>` as second argument which corresponds to a `valid-time`.
/// * `Delete` - Deletes the specific document at a given valid time, if `Option<DateTime<FixedOffset>>` is `None` it deletes the last `valid-`time` else it deletes the passed `valid-time`.
/// * `Evict` - Evicts a document entirely, including all historical versions (receives only the ID to evict).
/// * `Match` - Matches the current state of an entity, if the state doesn't match the provided document, the transaction will not continue. First argument is struct's `crux__db___id`,  the second is the serialized document that you want to match and the third argument is an `Option<DateTime<FixedOffset>>` which corresponds to a `valid-time` for the `Match`
#[derive(Debug, PartialEq)]
pub enum Action {
    Put(String, Option<DateTime<FixedOffset>>),
    Delete(String, Option<DateTime<FixedOffset>>),
    Evict(String),
    Match(String, String, Option<DateTime<FixedOffset>>),
}

impl Serialize for Action {
    fn serialize(self) -> String {
        match self {
            Action::Put(edn, None) => format!("[:crux.tx/put {}]", edn),
            Action::Put(edn, Some(date)) => format!(
                "[:crux.tx/put {} #inst \"{}\"]",
                edn,
                date.format(ACTION_DATE_FORMAT).to_string()
            ),
            Action::Delete(edn, None) => format!("[:crux.tx/delete {}]", edn),
            Action::Delete(edn, Some(date)) => format!(
                "[:crux.tx/delete {} #inst \"{}\"]",
                edn,
                date.format(ACTION_DATE_FORMAT).to_string()
            ),
            Action::Evict(id) => {
                if id.starts_with(":") {
                    format!("[:crux.tx/evict {}]", id)
                } else {
                    "".to_string()
                }
            }
            Action::Match(id, edn, None) => format!("[:crux.tx/match {} {}]", id, edn),
            Action::Match(id, edn, Some(date)) => format!(
                "[:crux.tx/match {} {} #inst \"{}\"]",
                id,
                edn,
                date.format(ACTION_DATE_FORMAT).to_string()
            ),
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

/// enum `TimeHistory` is used as an argument in the function `entity_history_timed`. It is responsible for defining `valid-time` and `transaction-times` ranges for the query.
/// The possible options are `ValidTime` and `TransactionTime`, both of them receive two `Option<DateTime<Utc>>`. The first parameter will transform into an start time and the second into and end-time, and they will be formated as `%Y-%m-%dT%H:%M:%S`.
/// The query params will become:
/// * ValidTime(Some(start), Some(end)) => "&start-valid-time={}&end-valid-time={}"
/// * ValidTime(None, Some(end)) => "&end-valid-time={}"
/// * ValidTime(Some(start), None) => "&start-valid-time={}"
/// * ValidTime(None, None) => "",
/// * TransactionTime(Some(start), Some(end)) => "&start-transaction-time={}&end-transaction-time={}"
/// * TransactionTime(None, Some(end)) => "&end-transaction-time={}"
/// * TransactionTime(Some(start), None) => "&start-transaction-time={}"
/// * TransactionTime(None, None) => "",
#[derive(Debug, PartialEq)]
pub enum TimeHistory {
    ValidTime(Option<DateTime<Utc>>, Option<DateTime<Utc>>),
    TransactionTime(Option<DateTime<Utc>>, Option<DateTime<Utc>>),
}

impl Serialize for TimeHistory {
    fn serialize(self) -> String {
        use crate::types::http::TimeHistory::TransactionTime;
        use crate::types::http::TimeHistory::ValidTime;

        match self {
            ValidTime(Some(start), Some(end)) => format!(
                "&start-valid-time={}&end-valid-time={}",
                start.format(DATETIME_FORMAT).to_string(),
                end.format(DATETIME_FORMAT).to_string()
            ),
            ValidTime(None, Some(end)) => format!(
                "&end-valid-time={}",
                end.format(DATETIME_FORMAT).to_string()
            ),
            ValidTime(Some(start), None) => format!(
                "&start-valid-time={}",
                start.format(DATETIME_FORMAT).to_string()
            ),
            ValidTime(None, None) => format!(""),

            TransactionTime(Some(start), Some(end)) => format!(
                "&start-transaction-time={}&end-transaction-time={}",
                start.format(DATETIME_FORMAT).to_string(),
                end.format(DATETIME_FORMAT).to_string()
            ),
            TransactionTime(None, Some(end)) => format!(
                "&end-transaction-time={}",
                end.format(DATETIME_FORMAT).to_string()
            ),
            TransactionTime(Some(start), None) => format!(
                "&start-transaction-time={}",
                start.format(DATETIME_FORMAT).to_string()
            ),
            TransactionTime(None, None) => format!(""),
        }
    }
}

#[doc(hidden)]
pub trait VecSer {
    fn serialize(self) -> String;
}
#[doc(hidden)]
impl VecSer for Vec<TimeHistory> {
    fn serialize(self) -> String {
        if self.len() > 2 || self.len() == 0 {
            String::new()
        } else {
            self.into_iter()
                .map(edn_rs::to_string)
                .collect::<Vec<String>>()
                .join("")
        }
    }
}
