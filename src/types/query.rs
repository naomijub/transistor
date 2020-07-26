use edn_rs::Serialize;

/// A [`Query`](https://opencrux.com/docs#queries) is a special kind of body that we submit to the `query` function. It has the following fields:
/// * `find` is responsible for defining which elements of the query you want shown in the response, it is **required**. Argument is a vector with elements to be queried, `vec!["a", "b", "c"]`. It is parsed as `:find [a b c]`, qhere `a, b, c` are the elements defined in `where` clause.
/// * `where_clause` is responsible for defining which rules will be applied to filter elements, it is **required**. Argument is a vector with the strings containing the filtering function, `vec!["a :db-key1 b", "a :db-key2 c", "a :db-key3 <some value>"]`. It is parsed as `:where [ [a :db-key1 b] [a :db-key2 c] [a :db-key3 <some value>] ]`.
/// * `args` is responsible for defining arguments to be replaced in `where_clause`, **optional**. Argument is a vector with strings containing the matches `vec!["?n \"Ivan\" ?l \"Ivanov\"", "?n \"Petr\" ?l \"Petrov\""]`.
/// * `order_by` is responsible for defining the order in which the response will be represented, **optional**. Argument is a vector with strings containing the element and how to order (`:asc` or `:desc`) `vec!["time :desc", "device-id :asc"]`.
/// * `limit` is responsible for defining the limit size of the response, **optional**. Argument is a usize.
/// * `offset` is responsible for defining the offset of the response, **optional**. Argument is a usize.
#[derive(Clone)]
pub struct Query {
    find: Find,
    where_: Option<Where>,
    args: Option<Args>,
    order_by: Option<OrderBy>,
    limit: Option<Limit>,
    offset: Option<Offset>,
}
#[derive(Clone)]
struct Find(Vec<String>);
#[derive(Clone)]
struct Where(Vec<String>);
#[derive(Clone)]
struct Args(Vec<String>);
#[derive(Clone)]
struct OrderBy(Vec<String>);
#[derive(Clone)]
struct Limit(usize);
#[derive(Clone)]
struct Offset(usize);

impl Query {
    /// `find` is the function responsible for defining the `:find` key in the query.
    /// Input should be the elements to be queried by the `where_clause`.
    /// Ex: `vec!["time", "device-id", "temperature", "humidity"]`.
    /// Becomes: `:find [time, device-id, temperature, humidity]`.
    pub fn find(find: Vec<&str>) -> Self {
        Self {
            find: Find {
                0: find.into_iter().map(String::from).collect::<Vec<String>>(),
            },
            where_: None,
            args: None,
            order_by: None,
            limit: None,
            offset: None,
        }
    }

    /// `where_clause` is the function responsible for defining the required `:where` key in the query.
    /// Input should be `element1 :key element2`, `element2` may have a modifier like `#inst`. The order matters.
    /// Ex: `vec!["c :condition/time time", "c :condition/device-id device-id", "c :condition/temperature temperature", "c :condition/humidity humidity"]`.
    /// Becomes:
    /// `:where [[c :condition/time time] [c :condition/device-id device-id] [c :condition/temperature temperature] [c :condition/humidity humidity]]`.
    pub fn where_clause(mut self, where_: Vec<&str>) -> Self {
        let w = where_
            .iter()
            .map(|s| s.replace("[", "").replace("]", ""))
            .collect::<Vec<String>>();
        self.where_ = Some(Where { 0: w });
        self
    }

    /// `args` is the function responsible for defining the optional `:args` key in the query.
    /// Input are elements you want to replace in the `where_clause`, a good practice is to name them with `?` before.
    /// Ex: `vec!["?n \"Ivan\" ?l \"Ivanov\"", "?n \"Petr\" ?l \"Petrov\""]`.
    /// Becomes: `:args [{?n "Ivan" ?l "Ivanov"} {?n "Petr" ?l "Petrov"}]`.
    pub fn args(mut self, args: Vec<&str>) -> Self {
        let a = args
            .iter()
            .map(|s| s.replace("{", "").replace("}", ""))
            .collect::<Vec<String>>();
        self.args = Some(Args { 0: a });
        self
    }

    /// `order_by` is the function responsible for defining the optional `:order-by` key in the query.
    /// Input is the elements to be ordered by, the first element is the first order, the second is the further orthers. Allowed keys are `:Asc`and `:desc`.
    /// Ex: `vec!["time :desc", "device-id :asc"]`.
    /// Becomes: `:order-by [[time :desc] [device-id :asc]]`.
    pub fn order_by(mut self, order_by: Vec<&str>) -> Self {
        let o = order_by
            .iter()
            .map(|s| s.replace("{", "").replace("}", ""))
            .collect::<Vec<String>>();
        self.order_by = Some(OrderBy { 0: o });
        self
    }

    /// `limit` is the function responsible for defining the optional `:limit` key in the query.
    /// Input is a usize with the query limit size.
    /// `.limit(5usize)` Becomes: `:limit 5`.
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(Limit { 0: limit });
        self
    }

    /// `offset` is the function responsible for defining the optional `:offset` key in the query.
    /// Input is a usize with the query offset.
    /// `.offset(5usize)` Becomes: `:offset 5`.
    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = Some(Offset { 0: offset });
        self
    }

    /// `build` function helps you assert that required fields were implemented.
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
        let mut q = String::from("{:query\n {");
        q.push_str(&self.find.serialize());
        q.push_str(&self.where_.unwrap().serialize());
        if self.args.is_some() {
            q.push_str(&self.args.unwrap().serialize());
        }
        if self.order_by.is_some() {
            q.push_str(&self.order_by.unwrap().serialize());
        }
        if self.limit.is_some() {
            q.push_str(&self.limit.unwrap().serialize());
        }
        if self.offset.is_some() {
            q.push_str(&self.offset.unwrap().serialize());
        }
        q.push_str("}}");
        q
    }
}

impl Serialize for Find {
    fn serialize(self) -> String {
        let mut q = String::from(":find [");
        q.push_str(&self.0.join(" "));
        q.push_str("]\n");
        q
    }
}

impl Serialize for Where {
    fn serialize(self) -> String {
        let mut q = String::from(":where [[");
        q.push_str(&self.0.join("]\n["));
        q.push_str("]]\n");
        q
    }
}

impl Serialize for Args {
    fn serialize(self) -> String {
        let mut q = String::from(":args [{");
        q.push_str(&self.0.join("}\n{"));
        q.push_str("}]\n");
        q
    }
}

impl Serialize for OrderBy {
    fn serialize(self) -> String {
        let mut q = String::from(":order-by [[");
        q.push_str(&self.0.join("]\n["));
        q.push_str("]]\n");
        q
    }
}

impl Serialize for Limit {
    fn serialize(self) -> String {
        let mut q = String::from(":limit ");
        q.push_str(&self.0.to_string());
        q.push_str("\n");
        q
    }
}

impl Serialize for Offset {
    fn serialize(self) -> String {
        let mut q = String::from(":offset ");
        q.push_str(&self.0.to_string());
        q.push_str("\n");
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
            "{:query\n {:find [p1]\n:where [[p1 :first-name n]\n[p1 :last-name \"Jorge\"]]\n}}";
        let q = Query::find(vec!["p1"])
            .where_clause(vec!["p1 :first-name n", "p1 :last-name \"Jorge\""])
            .build();

        assert_eq!(q.unwrap().serialize(), expected);
    }

    #[test]
    fn query_with_order() {
        let expected =
            "{:query\n {:find [p1]\n:where [[p1 :first-name n]\n[p1 :last-name \"Jorge\"]]\n:order-by [[p1 :Asc]]\n}}";
        let q = Query::find(vec!["p1"])
            .where_clause(vec!["p1 :first-name n", "p1 :last-name \"Jorge\""])
            .order_by(vec!["p1 :Asc"])
            .build();

        assert_eq!(q.unwrap().serialize(), expected);
    }

    #[test]
    fn query_with_args() {
        let expected =
            "{:query\n {:find [p1]\n:where [[p1 :first-name n]\n[p1 :last-name ?n]]\n:args [{?n \"Jorge\"}]\n}}";
        let q = Query::find(vec!["p1"])
            .where_clause(vec!["p1 :first-name n", "p1 :last-name ?n"])
            .args(vec!["?n \"Jorge\""])
            .build();

        assert_eq!(q.unwrap().serialize(), expected);
    }

    #[test]
    fn query_with_limit_and_offset() {
        let expected =
            "{:query\n {:find [p1]\n:where [[p1 :first-name n]\n[p1 :last-name n]]\n:limit 5\n:offset 10\n}}";
        let q = Query::find(vec!["p1"])
            .where_clause(vec!["p1 :first-name n", "p1 :last-name n"])
            .limit(5)
            .offset(10)
            .build();

        assert_eq!(q.unwrap().serialize(), expected);
    }

    #[test]
    fn full_query() {
        let expected =
            "{:query\n {:find [p1]\n:where [[p1 :first-name n]\n[p1 :last-name ?n]]\n:args [{?n \"Jorge\"}]\n:order-by [[p1 :Asc]]\n:limit 5\n:offset 10\n}}";
        let q = Query::find(vec!["p1"])
            .where_clause(vec!["p1 :first-name n", "p1 :last-name ?n"])
            .args(vec!["?n \"Jorge\""])
            .order_by(vec!["p1 :Asc"])
            .limit(5)
            .offset(10)
            .build();

        assert_eq!(q.unwrap().serialize(), expected);
    }
}
