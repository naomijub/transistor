use chrono::prelude::*;
use edn_rs::Serialize;
static ACTION_DATE_FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S%Z";
static DATETIME_FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S";
#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Action {
    Put(String, Option<DateTime<FixedOffset>>),
    Delete(String, Option<DateTime<FixedOffset>>),
    Evict(String),
    Match(String, String, Option<DateTime<FixedOffset>>),
}

/// Test enum to test and debug `Actions`. Implements `PartialEq` with `Actions`
#[cfg(feature = "mock")]
#[derive(Debug, PartialEq)]
pub enum ActionMock {
    Put(String, Option<DateTime<FixedOffset>>),
    Delete(String, Option<DateTime<FixedOffset>>),
    Evict(String),
    Match(String, String, Option<DateTime<FixedOffset>>),
}

/// Actions to perform in Crux. It is a builder struct to help you create a `Vec<Action>` for `tx_log`.
///
/// Allowed actions:
/// * `PUT` - Write a version of a document. Functions are `append_put` and `append_put_timed`.
/// * `Delete` - Deletes the specific document at a given valid time. Functions are `append_delete` and `append_delete_timed`.
/// * `Evict` - Evicts a document entirely, including all historical versions (receives only the ID to evict). Function is `append_evict`.
/// * `Match` - Matches the current state of an entity, if the state doesn't match the provided document, the transaction will not continue. Functions are `append_match` and `append_match_timed`.
#[derive(Debug, PartialEq, Clone)]
pub struct Actions {
    actions: Vec<Action>,
}

impl Actions {
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
        }
    }

    /// Appends an `Action::Put` enforcing types for `action` field to be a `T: Serialize`
    pub fn append_put<T: Serialize>(mut self, action: T) -> Self {
        self.actions.push(Action::put(action));
        self
    }

    /// Appends an `Action::Put` that includes `date` enforcing types for `action` field to be a `T: Serialize` and `date` to be `DateTime<FixedOffset>`.
    pub fn append_put_timed<T: Serialize>(
        mut self,
        action: T,
        date: DateTime<FixedOffset>,
    ) -> Self {
        self.actions.push(Action::put(action).with_valid_date(date));
        self
    }

    /// Appends an `Action::Delete` enforcing types for `id` field to be a `CruxId`
    pub fn append_delete(mut self, id: crate::types::CruxId) -> Self {
        self.actions.push(Action::delete(id));
        self
    }

    /// Appends an `Action::Delete` that includes `date` enforcing types for `id` field to be a `CruxId` and `date` to be `DateTime<FixedOffset>`.
    pub fn append_delete_timed(
        mut self,
        id: crate::types::CruxId,
        date: DateTime<FixedOffset>,
    ) -> Self {
        self.actions.push(Action::delete(id).with_valid_date(date));
        self
    }

    /// Appends an `Action::Evict` enforcing types for `id` field to be a `CruxId`
    pub fn append_evict(mut self, id: crate::types::CruxId) -> Self {
        self.actions.push(Action::evict(id));
        self
    }

    /// Appends an `Action::Match` enforcing types for `id` field to be a `CruxId` and `action` field to be a `T: Serialize`
    pub fn append_match_doc<T: Serialize>(mut self, id: crate::types::CruxId, action: T) -> Self {
        self.actions.push(Action::match_doc(id, action));
        self
    }

    /// Appends an `Action::Match` that includes `date` enforcing types for `id` field to be a `CruxId`, `action` field to be a `T: Serialize` and `date` to be `DateTime<FixedOffset>`.
    pub fn append_match_doc_timed<T: Serialize>(
        mut self,
        id: crate::types::CruxId,
        action: T,
        date: DateTime<FixedOffset>,
    ) -> Self {
        self.actions
            .push(Action::match_doc(id, action).with_valid_date(date));
        self
    }

    pub(crate) fn build(self) -> String {
        let actions_str = self
            .actions
            .into_iter()
            .map(edn_rs::to_string)
            .collect::<Vec<String>>()
            .join(", ");

        let mut s = String::from("[");
        s.push_str(&actions_str);
        s.push_str("]");
        s
    }
}

impl Action {
    fn put<T: Serialize>(action: T) -> Action {
        Action::Put(edn_rs::to_string(action), None)
    }

    fn with_valid_date(self, date: DateTime<FixedOffset>) -> Action {
        match self {
            Action::Put(action, _) => Action::Put(action, Some(date)),
            Action::Delete(action, _) => Action::Delete(action, Some(date)),
            Action::Match(id, action, _) => Action::Match(id, action, Some(date)),
            action => action,
        }
    }

    fn delete(id: crate::types::CruxId) -> Action {
        Action::Delete(edn_rs::to_string(id), None)
    }

    fn evict(id: crate::types::CruxId) -> Action {
        Action::Evict(edn_rs::to_string(id))
    }

    fn match_doc<T: Serialize>(id: crate::types::CruxId, action: T) -> Action {
        Action::Match(edn_rs::to_string(id), edn_rs::to_string(action), None)
    }
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
            Action::Delete(id, None) => format!("[:crux.tx/delete {}]", id),
            Action::Delete(id, Some(date)) => format!(
                "[:crux.tx/delete {} #inst \"{}\"]",
                id,
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

#[cfg(feature = "mock")]
impl std::cmp::PartialEq<Vec<ActionMock>> for Actions {
    fn eq(&self, other: &Vec<ActionMock>) -> bool {
        self.actions
            .iter()
            .zip(other.iter())
            .map(|(acs, acm)| match (acs, acm) {
                (Action::Put(ap, tp), ActionMock::Put(am, tm)) if ap == am && tp == tm => true,
                (Action::Evict(id), ActionMock::Evict(idm)) if id == idm => true,
                (Action::Delete(id, tp), ActionMock::Delete(idm, tm)) if id == idm && tp == tm => {
                    true
                }
                (Action::Match(id, a, tp), ActionMock::Match(idm, am, tm))
                    if id == idm && a == am && tp == tm =>
                {
                    true
                }
                _ => false,
            })
            .fold(true, |acc, e| acc && e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::CruxId;
    use edn_rs::ser_struct;

    #[test]
    fn actions() {
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

        let person3 = Person {
            crux__db___id: CruxId::new("manuel-1"),
            first_name: "Diego".to_string(),
            last_name: "Manuel".to_string(),
        };

        let timed = "2014-11-28T21:00:09-09:00"
            .parse::<DateTime<FixedOffset>>()
            .unwrap();

        let actions = Actions::new()
            .append_put_timed(person1.clone(), timed)
            .append_put(person2.clone())
            .append_evict(person1.crux__db___id)
            .append_delete(person2.crux__db___id)
            .append_match_doc(person3.clone().crux__db___id, person3);

        assert_eq!(actions.clone(), expected_actions());
    }

    fn expected_actions() -> Actions {
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

        let person3 = Person {
            crux__db___id: CruxId::new("manuel-1"),
            first_name: "Diego".to_string(),
            last_name: "Manuel".to_string(),
        };

        Actions {
            actions: vec![
                Action::Put(
                    person1.clone().serialize(),
                    Some(
                        "2014-11-28T21:00:09-09:00"
                            .parse::<DateTime<FixedOffset>>()
                            .unwrap(),
                    ),
                ),
                Action::Put(person2.clone().serialize(), None),
                Action::Evict(person1.crux__db___id.serialize()),
                Action::Delete(person2.crux__db___id.serialize(), None),
                Action::Match(
                    person3.clone().crux__db___id.serialize(),
                    person3.serialize(),
                    None,
                ),
            ],
        }
    }

    ser_struct! {
        #[derive(Debug, Clone)]
        #[allow(non_snake_case)]
        pub struct Person {
            crux__db___id: CruxId,
            first_name: String,
            last_name: String
        }
    }
}
