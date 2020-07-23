use edn_rs::{
    Serialize,
};

pub struct Query {
    find: Vec<String>,
    whr: Option<Vec<String>>,
}

impl Query {
    pub fn find(find: Vec<&str>) -> Self {
        Self {
            find: find.into_iter().map(|f| String::from(f)).collect::<Vec<String>>(),
            whr: None
        }
    }

    pub fn where_clause(mut self, whr: Vec<&str>) -> Self {
        self.whr = Some(whr.into_iter().map(|f| String::from(f)).collect::<Vec<String>>());
        self
    }

    pub fn build(self) -> Result<Self,String> {
        if self.whr.is_none() {
            Err(String::from("Where clause is required"))
        } else {
            Ok(self)
        }
    }
}

impl Serialize for Query {
    fn serialize(self) -> String {
        let mut q = String::from("{:query\n {:find [");
        self.find.iter().for_each(|f| q.push_str(f));
        q.push_str("]\n  :where [[");
        q.push_str(&self.whr.unwrap_or(vec!["".to_string()]).join("]\n["));
        q.push_str("]]}}");
        q
    }
}

#[cfg(test)]
mod test {
    use edn_rs::{
        Serialize,
    };
    use super::Query;
    
    #[test]
    fn query_with_find_andwhere() {
        let exoected = "{:query\n {:find [p1]\n  :where [[p1 :first-name n]\n[p1 :last-name \"Jorge\"]]}}";
        let q = Query::find(vec!["p1"])
        .where_clause(vec!["p1 :first-name n", "p1 :last-name \"Jorge\""])
        .build();

        assert_eq!(q.unwrap().serialize(), exoected);
    }
}