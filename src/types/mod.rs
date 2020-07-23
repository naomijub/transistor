pub mod response;
pub mod query;

use edn_rs::{
    Serialize,
};

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