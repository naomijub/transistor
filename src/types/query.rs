use crate::types::error::CruxError;
use edn_rs::Serialize;

/// A `Query` is a special kind of body that we submit to the `query` function. It has the following fields:
/// * `find` is responsible for defining which elements of the query you want shown in the response, it is **required**. Argument is a vector with elements to be queried, `vec!["a", "b", "c"]`. It is parsed as `:find [a b c]`, qhere `a, b, c` are the elements defined in `where` clause.
/// * `where_clause` is responsible for defining which rules will be applied to filter elements, it is **required**. Argument is a vector with the string containing the filtering function, `vec!["a :db-key1 b", "a :db-key2 c", "a :db-key3 <some value>"]`. It is parsed as `:where [ [a :db-key1 b] [a :db-key2 c] [a :db-key3 <some value>] ]`.
#[derive(Clone, Debug)]
pub struct Query {
    find: Vec<String>,
    where_: Option<Vec<String>>,
}

impl Query {
    /// `find` is the function responsible for defining the `:find` key in the query.
    pub fn find(find: Vec<&str>) -> Self {
        Self {
            find: find.into_iter().map(String::from).collect::<Vec<String>>(),
            where_: None,
        }
    }

    /// `where_clause` is the function responsible for defining the `:where` key in the query.
    pub fn where_clause(mut self, where_: Vec<&str>) -> Self {
        self.where_ = Some(
            where_
                .into_iter()
                .map(String::from)
                .collect::<Vec<String>>(),
        );
        self
    }

    /// `build` function helps you assert that required fields were implemented.
    pub fn build(self) -> Result<Self, CruxError> {
        if self.where_.is_none() {
            Err(CruxError::QueryFormatError(String::from(
                "Where clause is required",
            )))
        } else {
            Ok(self)
        }
    }
}

impl Serialize for Query {
    fn serialize(self) -> String {
        let mut q = String::from("{:query\n {:find [");
        q.push_str(&self.find.join(" "));
        q.push_str("]\n  :where [[");
        q.push_str(&self.where_.unwrap_or(vec!["".to_string()]).join("]\n["));
        q.push_str("]]}}");
        q
    }
}

#[cfg(test)]
mod test {
    use super::Query;
    use edn_rs::Serialize;
    use crate::client::Crux;

    #[test]
    fn query_with_find_and_where() {
        let expected =
            "{:query\n {:find [p1]\n  :where [[p1 :first-name n]\n[p1 :last-name \"Jorge\"]]}}";
        let q = Query::find(vec!["p1"])
            .where_clause(vec!["p1 :first-name n", "p1 :last-name \"Jorge\""])
            .build();

        assert_eq!(q.unwrap().serialize(), expected);
    }

    #[test]
    #[should_panic(expected = "Where clause is required")]
    fn expect_query_format_error() {
        let client = Crux::new("","").docker_client();
        let query_where_is_none = Query::find(vec!["p1", "n"])
        .build().unwrap();

        let _ = client.query(query_where_is_none).unwrap();
    }
}
