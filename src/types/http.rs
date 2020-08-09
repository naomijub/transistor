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
