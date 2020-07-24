use edn_rs::Serialize;

#[derive(Clone)]
pub struct Query {
    find: Vec<String>,
    where_: Option<Vec<String>>,
}

impl Query {
    pub fn find(find: Vec<&str>) -> Self {
        Self {
            find: find.into_iter().map(String::from).collect::<Vec<String>>(),
            where_: None,
        }
    }

    pub fn where_clause(mut self, where_: Vec<&str>) -> Self {
        self.where_ = Some(
            where_
                .into_iter()
                .map(String::from)
                .collect::<Vec<String>>(),
        );
        self
    }

    pub fn build(self) -> Result<Self, String> {
        if self.where_.is_none() {
            Err(String::from("Where clause is required"))
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

    #[test]
    fn query_with_find_and_where() {
        let expected =
            "{:query\n {:find [p1]\n  :where [[p1 :first-name n]\n[p1 :last-name \"Jorge\"]]}}";
        let q = Query::find(vec!["p1"])
            .where_clause(vec!["p1 :first-name n", "p1 :last-name \"Jorge\""])
            .build();

        assert_eq!(q.unwrap().serialize(), expected);
    }
}
