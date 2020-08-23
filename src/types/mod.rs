pub mod error;
pub mod http;
pub mod query;
pub mod response;

use edn_rs::{Serialize, Deserialize, Edn, EdnError};

/// Id to use as reference in Crux, similar to `ids` with `Uuid`. This id is supposed to be a KEYWORD, `Edn::Key`.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct CruxId(String);

impl Serialize for CruxId {
    fn serialize(mut self) -> String {
        self.0.insert(0, ':');

        format!("{}", self.0.replace(" ", "-"))
    }
}

impl Deserialize for CruxId {
    fn deserialize(edn: &Edn) -> Result<Self, EdnError> {
        match edn {
            Edn::Str(s) => Ok(Self::new(s)),
            _ => Err(EdnError::Deserialize(format!("couldn't convert {} into CruxId", edn))),
        }
    }
}

impl CruxId {
    /// `CruxId::new` receives a regular string and parses it to the `Edn::Key` format.
    /// `CruxId::new("Jorge da Silva") -> Edn::Key(":Jorge-da-Silva")`
    pub fn new(id: &str) -> Self {
        let clean_id = id.replace(":", "");
        Self {
            0: clean_id.to_string(),
        }
    }
}
