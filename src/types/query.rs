use crate::types::error::CruxError;
use edn_rs::Serialize;
use std::collections::BTreeSet;
use std::convert::TryFrom;

/// A [`Query`](https://opencrux.com/reference/queries.html) is a special kind of body that we submit to the `query` function. It has the following fields:
/// * `find` is responsible for defining which elements of the query you want shown in the response, it is **required**. Argument is a vector with elements to be queried, `vec!["a", "b", "c"]`. It is parsed as `:find [a b c]`, qhere `a, b, c` are the elements defined in `where` clause.
/// * `where_clause` is responsible for defining which rules will be applied to filter elements, it is **required**. Argument is a vector with the strings containing the filtering function, `vec!["a :db-key1 b", "a :db-key2 c", "a :db-key3 <some value>"]`. It is parsed as `:where [ [a :db-key1 b] [a :db-key2 c] [a :db-key3 <some value>] ]`.
/// * `args` is responsible for defining arguments to be replaced in `where_clause`, **optional**. Argument is a vector with strings containing the matches `vec!["?n \"Ivan\" ?l \"Ivanov\"", "?n \"Petr\" ?l \"Petrov\""]`.
/// * `order_by` is responsible for defining the order in which the response will be represented, **optional**. Argument is a vector with strings containing the element and how to order (`:asc` or `:desc`) `vec!["time :desc", "device-id :asc"]`.
/// * `limit` is responsible for defining the limit size of the response, **optional**. Argument is a usize.
/// * `offset` is responsible for defining the offset of the response, **optional**. Argument is a usize.
#[derive(Clone, Debug)]
pub struct Query {
    find: Find,
    aggregates: Option<std::collections::HashSet<String>>,
    where_: Option<Where>,
    args: Option<Args>,
    order_by: Option<OrderBy>,
    limit: Option<Limit>,
    offset: Option<Offset>,
    full_results: bool,
}
#[derive(Clone, Debug)]
struct Find(Vec<String>);
#[derive(Clone, Debug)]
struct Where(Vec<String>);
#[derive(Clone, Debug)]
struct Args(Vec<String>);
#[derive(Clone, Debug)]
struct OrderBy(Vec<String>);
#[derive(Clone, Debug)]
struct Limit(usize);
#[derive(Clone, Debug)]
struct Offset(usize);

/// `Aggregate` is an enum of possible aggregation to use with `find_by_aggregates` clause.
#[derive(Clone)]
pub enum Aggregate {
    /// Accumulates as single value via the Clojure + function
    Sum(String),
    /// Return the single minimal value via the Clojure compare function which may operates on many types (integers, strings, collections etc.)
    Min(String),
    /// Return the single maximal value via the Clojure compare function which may operates on many types (integers, strings, collections etc.)
    Max(String),
    /// Returns a sorted set of the N minimum items. N must be a positive integer and cannot be referenced via an additional logic-var.
    MinN(usize, String),
    /// Returns a sorted set of the N maximum items. N must be a positive integer and cannot be referenced via an additional logic-var.
    MaxN(usize, String),
    /// Returns a single count of all values including any duplicates
    Count(String),
    /// Returns a single count of all unique values
    CountDistinct(String),
    /// Returns a single value equivalent to `sum / count`
    Avg(String),
    /// Return single value corresponding to the statistical definition of median
    Median(String),
    /// Return single value corresponding to the statistical definition of variance
    Variance(String),
    /// Return single value corresponding to the statistical definition of stddev
    Stddev(String),
    /// Returns a vector of exactly N values, where some values may be duplicates if N is larger than the range
    Rand(usize, String),
    /// Returns a vector of at-most N distinct values
    Sample(usize, String),
    /// Returns a set of distinct values
    Distinct(String),
}

impl Aggregate {
    fn string_value(&self) -> String {
        match self {
            Aggregate::Sum(s) => s.to_string(),
            Aggregate::Min(s) => s.to_string(),
            Aggregate::Max(s) => s.to_string(),
            Aggregate::MinN(_, s) => s.to_string(),
            Aggregate::MaxN(_, s) => s.to_string(),
            Aggregate::Count(s) => s.to_string(),
            Aggregate::CountDistinct(s) => s.to_string(),
            Aggregate::Avg(s) => s.to_string(),
            Aggregate::Median(s) => s.to_string(),
            Aggregate::Variance(s) => s.to_string(),
            Aggregate::Stddev(s) => s.to_string(),
            Aggregate::Rand(_, s) => s.to_string(),
            Aggregate::Sample(_, s) => s.to_string(),
            Aggregate::Distinct(s) => s.to_string(),
        }
    }
}

impl std::fmt::Display for Aggregate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Aggregate::Sum(s) => write!(f, "(sum {})", &s),
            Aggregate::Min(s) => write!(f, "(min {})", &s),
            Aggregate::Max(s) => write!(f, "(max {})", &s),
            Aggregate::MinN(n, s) => write!(f, "(min {} {})", n, &s),
            Aggregate::MaxN(n, s) => write!(f, "(max {} {})", n, &s),
            Aggregate::Count(s) => write!(f, "(count {})", &s),
            Aggregate::CountDistinct(s) => write!(f, "(count-distinct {})", &s),
            Aggregate::Avg(s) => write!(f, "(avg {})", &s),
            Aggregate::Median(s) => write!(f, "(median {})", &s),
            Aggregate::Variance(s) => write!(f, "(variance {})", &s),
            Aggregate::Stddev(s) => write!(f, "(stddev {})", &s),
            Aggregate::Rand(n, s) => write!(f, "(rand {} {})", n, &s),
            Aggregate::Sample(n, s) => write!(f, "(sample {} {})", n, &s),
            Aggregate::Distinct(s) => write!(f, "(distinct {})", &s),
        }
    }
}

impl Query {
    /// `find` is the function responsible for defining the `:find` key in the query.
    /// Input should be the elements to be queried by the `where_clause`.
    /// Ex: `vec!["time", "device-id", "temperature", "humidity"]`.
    /// Becomes: `:find [time, device-id, temperature, humidity]`.
    ///
    /// Error cases:
    /// * All elements should start with `?`, example `vec!["?p1", "?n", "?g"]`. If theey do not start the CruxError::QueryFormatError containing `All elements of find clause should start with '?', element '{}' doesn't conform` is thrown.
    pub fn find(find: Vec<&str>) -> Result<Self, CruxError> {
        if find.iter().any(|e| !e.starts_with("?")) {
            let error = find.iter().find(|e| !e.starts_with("?")).unwrap();
            return Err(CruxError::QueryFormatError(format!(
                "All elements of find clause should start with '?', element '{}' doesn't conform",
                error
            )));
        }

        Ok(Self {
            find: Find {
                0: find.into_iter().map(String::from).collect::<Vec<String>>(),
            },
            aggregates: None,
            where_: None,
            args: None,
            order_by: None,
            limit: None,
            offset: None,
            full_results: false,
        })
    }

    /// `find_by_aggregates` is the function responsible for defining the `:find` key in the query similar to `find`.
    ///  However, it supports sending aggregates to `:find` keys.
    /// Input should be the elements to be queried by the `where_clause`.
    /// Ex: `vec![(Sum("?heads"), Min("?heads"), Max("?heads"), Count("?heads"),  CountDistinct("?heads")]`.
    /// Becomes: `:find [(sum ?heads) (min ?heads) (max ?heads) (count ?heads)  (count-distinct ?heads)]`.
    ///
    /// Error cases:
    /// * All elements should start with `?`, example `vec!["(min ?heads)"]`. If theey do not start the CruxError::QueryFormatError containing `All elements of find clause should start with '?', element '{}' doesn't conform` is thrown.
    pub fn find_by_aggregates(find: Vec<Aggregate>) -> Result<Self, CruxError> {
        // if find.iter()
        //     .any(|e| !e.starts_with("?")) {
        //     let error = find.iter().find(|e| !e.starts_with("?")).unwrap();
        //     return Err(CruxError::QueryFormatError(format!(
        //         "All elements of find clause should start with '?', element '{}' doesn't conform",
        //         error
        //     )));
        // }
        if find
            .iter()
            .map(|a| a.string_value())
            .any(|e| !e.starts_with("?"))
        {
            let error = find
                .iter()
                .filter(|e| !e.string_value().starts_with("?"))
                .take(1)
                .next()
                .unwrap();
            return Err(CruxError::QueryFormatError(format!(
                "All elements of find clause should start with '?', element '{}' doesn't conform",
                error
            )));
        }
        let aggregates = Some(
            find.iter()
                .map(|a| a.string_value())
                .collect::<std::collections::HashSet<String>>(),
        );

        Ok(Self {
            find: Find {
                0: find
                    .into_iter()
                    .map(|a| a.to_string())
                    .collect::<Vec<String>>(),
            },
            aggregates: aggregates,
            where_: None,
            args: None,
            order_by: None,
            limit: None,
            offset: None,
            full_results: false,
        })
    }

    /// `where_clause` is the function responsible for defining the required `:where` key in the query.
    /// Input should be `element1 :key element2`, `element2` may have a modifier like `#inst`. The order matters.
    /// Ex: `vec!["c :condition/time time", "c :condition/device-id device-id", "c :condition/temperature temperature", "c :condition/humidity humidity"]`.
    /// Becomes:
    /// `:where [[c :condition/time time] [c :condition/device-id device-id] [c :condition/temperature temperature] [c :condition/humidity humidity]]`.
    ///
    /// Error cases:
    /// * All elements present in find clause should be present in where clause. If your find clause is `"?p", "?n", "?s"`, and your where clause is `"?p1 :alpha ?n", "?p1 :beta true"` an error `Not all element of find, `"?p", "?n", "?s"`, are present in the where clause, ?s is missing` is thrown.
    pub fn where_clause(mut self, where_: Vec<&str>) -> Result<Self, CruxError> {
        if self.aggregates.is_none() && self.find.0.iter().any(|e| !where_.join(" ").contains(e)) {
            let error = self
                .find
                .0
                .iter()
                .find(|e| !where_.join(" ").contains(*e))
                .unwrap();
            return Err(CruxError::QueryFormatError(format!(
                "Not all element of find, {}, are present in the where clause, {} is missing",
                self.find.0.join(", "),
                error
            )));
        }

        if self.clone().aggregates.is_some()
            && self
                .aggregates
                .clone()
                .unwrap_or(std::collections::HashSet::new())
                .iter()
                .any(|e| !where_.join(" ").contains(e))
        {
            return Err(CruxError::QueryFormatError(format!(
                "Not all element of find, {}, are present in the where clause",
                self.find.0.join(", "),
            )));
        }

        let w = where_
            .iter()
            .map(|s| s.replace("[", "").replace("]", ""))
            .collect::<Vec<String>>();
        self.where_ = Some(Where { 0: w });
        Ok(self)
    }

    /// `args` is the function responsible for defining the optional `:args` key in the query.
    /// Input are elements you want to replace in the `where_clause`, a good practice is to name them with `?` before.
    /// Ex: `vec!["?n \"Ivan\" ?l \"Ivanov\"", "?n \"Petr\" ?l \"Petrov\""]`.
    /// Becomes: `:args [{?n "Ivan" ?l "Ivanov"} {?n "Petr" ?l "Petrov"}]`.
    ///
    /// Error cases:
    /// * The first element of the argument key-value tuple should start with `?`. An input `vec!["n true"]` will return an error `All elements should start with '?'`.
    /// * All arguments key should be present in the where clause. If the where clause `?p1 :name ?n", "?p1 :is-sql ?s", "?p1 :is-sql true"` and an args clause `vec!["?s true ?x 1243"]` will return an error `All elements should be present in where clause`.
    pub fn args(mut self, args: Vec<&str>) -> Result<Self, CruxError> {
        let where_ = self.where_.clone().unwrap().0.join(" ");
        self.args = Some(Args::try_from((args, where_))?);
        Ok(self)
    }

    /// `order_by` is the function responsible for defining the optional `:order-by` key in the query.
    /// Input is the elements to be ordered by, the first element is the first order, the second is the further orthers. Allowed keys are `:Asc`and `:desc`.
    /// Ex: `vec!["time :desc", "device-id :asc"]`.
    /// Becomes: `:order-by [[time :desc] [device-id :asc]]`.
    ///
    /// Error cases:
    /// * The second element of each order clause should be `:asc` or `:desc`, if different, like `:eq` in `"?p1 :asc", "?n :desc", "?s :eq"`, error `Order element should be ':asc' or ':desc'` is thrown.
    /// * The first element of each order clause should be present in the find clause. If the order clause is `"?p1 :asc", "?n :desc", "?g :asc"` and the find clause is `"?p1", "?n"` the error `All elements to be ordered should be present in find clause, ?g not present` is thrown.
    pub fn order_by(mut self, order_by: Vec<&str>) -> Result<Self, CruxError> {
        let f = self.find.0.join(" ");
        if !order_by
            .iter()
            .map(|e| e.split(" ").collect::<Vec<&str>>())
            .map(|e| e[1])
            .all(|e| e.to_lowercase() == ":asc" || e.to_lowercase() == ":desc")
        {
            return Err(CruxError::QueryFormatError(
                "Order element should be ':asc' or ':desc'".to_string(),
            ));
        }
        if !order_by
            .iter()
            .map(|e| e.split(" ").collect::<Vec<&str>>())
            .map(|e| e[0])
            .all(|e| f.contains(e))
        {
            let error = order_by
                .iter()
                .map(|e| e.split(" ").collect::<Vec<&str>>())
                .map(|e| e[0])
                .find(|e| !f.contains(e))
                .unwrap();
            return Err(CruxError::QueryFormatError(format!(
                "All elements to be ordered should be present in find clause, {} not present",
                error
            )));
        }

        let o = order_by
            .iter()
            .map(|s| s.replace("[", "").replace("]", ""))
            .collect::<Vec<String>>();
        self.order_by = Some(OrderBy { 0: o });
        Ok(self)
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

    /// `with_full_results` adds `:full-results? true` to the query map to easily retrieve the source documents relating to the entities in the result set.
    pub fn with_full_results(mut self) -> Self {
        self.full_results = true;
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
        let mut q = String::from("{:query\n {");
        q.push_str(&edn_rs::to_string(self.find));
        q.push_str(&edn_rs::to_string(self.where_.unwrap()));
        if self.args.is_some() {
            q.push_str(&edn_rs::to_string(self.args.unwrap()));
        }
        if self.order_by.is_some() {
            q.push_str(&edn_rs::to_string(self.order_by.unwrap()));
        }
        if self.limit.is_some() {
            q.push_str(&edn_rs::to_string(self.limit.unwrap()));
        }
        if self.offset.is_some() {
            q.push_str(&edn_rs::to_string(self.offset.unwrap()));
        }
        if self.full_results == true {
            q.push_str(" :full-results? true\n")
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

type RawArgsWithWhere<'a> = (Vec<&'a str>, String);

impl TryFrom<RawArgsWithWhere<'_>> for Args {
    type Error = CruxError;

    fn try_from(value: RawArgsWithWhere) -> Result<Self, Self::Error> {
        let (args, where_) = value;

        let args_key_set = args_key_bset(&args);

        let all_elements_in_where = args_key_set.iter().any(|e| !where_.contains(e));
        let has_question = args_key_set.iter().any(|e| !e.starts_with("?"));

        match (all_elements_in_where, has_question) {
            (true, false) =>  Err(CruxError::QueryFormatError("All elements should be present in where clause".to_string())),
            (false, true) =>  Err(CruxError::QueryFormatError("All elements should start with '?'".to_string())),
            (true, true) =>  Err(CruxError::QueryFormatError("All elements should be present in where clause and all elements should start with '?'".to_string())),
            (false, false) => Ok(Args{0: args.iter().map(|s| s.replace("{", "").replace("}", "")).collect::<Vec<String>>()}),
        }
    }
}

fn args_key_bset(args: &Vec<&str>) -> BTreeSet<String> {
    let args_without_inst = args.join(" ").replace("#inst", "");
    args_without_inst
        .split(" ")
        .filter(|i| !i.is_empty())
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(_, s)| s.to_owned())
        .collect::<BTreeSet<String>>()
}

#[cfg(test)]
mod test {
    use super::{Aggregate, Query};
    use crate::client::Crux;

    #[test]
    fn query_with_find_and_where() {
        let expected =
            "{:query\n {:find [?p1]\n:where [[?p1 :first-name n]\n[?p1 :last-name \"Jorge\"]]\n}}";
        let q = Query::find(vec!["?p1"])
            .unwrap()
            .where_clause(vec!["?p1 :first-name n", "?p1 :last-name \"Jorge\""])
            .unwrap()
            .build();

        assert_eq!(edn_rs::to_string(q.unwrap()), expected);
    }

    #[test]
    #[should_panic(expected = "Where clause is required")]
    fn expect_query_format_error() {
        let client = Crux::new("", "").http_client();
        let query_where_is_none = Query::find(vec!["?p1", "?n"]).unwrap().build().unwrap();

        let _ = client.query(query_where_is_none).unwrap();
    }

    #[test]
    fn query_with_order() {
        let expected =
            "{:query\n {:find [?p1]\n:where [[?p1 :first-name n]\n[?p1 :last-name \"Jorge\"]]\n:order-by [[?p1 :asc]]\n}}";
        let q = Query::find(vec!["?p1"])
            .unwrap()
            .where_clause(vec!["?p1 :first-name n", "?p1 :last-name \"Jorge\""])
            .unwrap()
            .order_by(vec!["?p1 :asc"])
            .unwrap()
            .build();

        assert_eq!(edn_rs::to_string(q.unwrap()), expected);
    }

    #[test]
    fn query_with_args() {
        let expected =
            "{:query\n {:find [?p1]\n:where [[?p1 :first-name n]\n[?p1 :last-name ?n]]\n:args [{?n \"Jorge\"}]\n}}";
        let q = Query::find(vec!["?p1"])
            .unwrap()
            .where_clause(vec!["?p1 :first-name n", "?p1 :last-name ?n"])
            .unwrap()
            .args(vec!["?n \"Jorge\""])
            .unwrap()
            .build();

        assert_eq!(edn_rs::to_string(q.unwrap()), expected);
    }

    #[test]
    fn query_with_limit_and_offset() {
        let expected =
            "{:query\n {:find [?p1]\n:where [[?p1 :first-name n]\n[?p1 :last-name n]]\n:limit 5\n:offset 10\n}}";
        let q = Query::find(vec!["?p1"])
            .unwrap()
            .where_clause(vec!["?p1 :first-name n", "?p1 :last-name n"])
            .unwrap()
            .limit(5)
            .offset(10)
            .build();

        assert_eq!(edn_rs::to_string(q.unwrap()), expected);
    }

    #[test]
    fn full_query() {
        let expected =
            "{:query\n {:find [?p1]\n:where [[?p1 :first-name n]\n[?p1 :last-name ?n]]\n:args [{?n \"Jorge\"}]\n:order-by [[?p1 :Asc]]\n:limit 5\n:offset 10\n}}";
        let q = Query::find(vec!["?p1"])
            .unwrap()
            .where_clause(vec!["?p1 :first-name n", "?p1 :last-name ?n"])
            .unwrap()
            .args(vec!["?n \"Jorge\""])
            .unwrap()
            .order_by(vec!["?p1 :Asc"])
            .unwrap()
            .limit(5)
            .offset(10)
            .build();

        assert_eq!(edn_rs::to_string(q.unwrap()), expected);
    }

    #[test]
    #[should_panic(
        expected = "Not all element of find, ?p1, ?n, ?s, are present in the where clause, ?n is missing"
    )]
    fn where_query_format_error() {
        let _query = Query::find(vec!["?p1", "?n", "?s"])
            .unwrap()
            .where_clause(vec!["?p1 :name ?g", "?p1 :is-sql ?s", "?p1 :is-sql true"])
            .unwrap()
            .build();
    }

    #[test]
    #[should_panic(expected = "Order element should be \\\':asc\\\' or \\\':desc\\\'")]
    fn order_should_panic_for_unknow_order_element() {
        let _query = Query::find(vec!["?p1", "?n", "?s"])
            .unwrap()
            .where_clause(vec!["?p1 :name ?n", "?p1 :is-sql ?s", "?p1 :is-sql true"])
            .unwrap()
            .order_by(vec!["?p1 :asc", "?n :desc", "?s :eq"])
            .unwrap()
            .build();
    }

    #[test]
    #[should_panic(
        expected = "All elements to be ordered should be present in find clause, ?g not present"
    )]
    fn order_element_should_be_present_in_find_clause() {
        let _query = Query::find(vec!["?p1", "?n", "?s"])
            .unwrap()
            .where_clause(vec!["?p1 :name ?n", "?p1 :is-sql ?s", "?p1 :is-sql true"])
            .unwrap()
            .order_by(vec!["?p1 :asc", "?n :desc", "?g :asc"])
            .unwrap()
            .build();
    }

    #[test]
    #[should_panic(expected = "All elements should be present in where clause")]
    fn all_args_present_in_where() {
        let _query = Query::find(vec!["?p1", "?n"])
            .unwrap()
            .where_clause(vec!["?p1 :name ?n", "?p1 :is-sql ?s", "?p1 :is-sql true"])
            .unwrap()
            .args(vec!["?s true ?x 1243"])
            .unwrap()
            .build();
    }

    #[test]
    #[should_panic(expected = "All elements should start with \\\'?\\\'")]
    fn all_args_should_start_with_question() {
        let _query = Query::find(vec!["?p1", "?n"])
            .unwrap()
            .where_clause(vec!["?p1 :name ?n", "?p1 :is-sql s", "?p1 :is-sql true"])
            .unwrap()
            .args(vec!["s    true"])
            .unwrap()
            .build();
    }

    #[test]
    fn query_with_full_results() {
        let expected =
            "{:query\n {:find [?p1]\n:where [[?p1 :first-name n]\n[?p1 :last-name \"Jorge\"]]\n :full-results? true\n}}";
        let q = Query::find(vec!["?p1"])
            .unwrap()
            .where_clause(vec!["?p1 :first-name n", "?p1 :last-name \"Jorge\""])
            .unwrap()
            .with_full_results()
            .build();

        assert_eq!(edn_rs::to_string(q.unwrap()), expected);
    }

    #[test]
    fn query_with_aggregates() {
        let expected = "{:query\n {:find [(min ?e) (max ?e) (count ?e) (min 5 ?e) (count-distinct ?e)]\n:where [[?e :type :burger]]\n}}";
        let q = Query::find_by_aggregates(vec![
            Aggregate::Min("?e".to_string()),
            Aggregate::Max("?e".to_string()),
            Aggregate::Count("?e".to_string()),
            Aggregate::MinN(5, "?e".to_string()),
            Aggregate::CountDistinct("?e".to_string()),
        ])
        .unwrap()
        .where_clause(vec!["?e :type :burger"])
        .unwrap()
        .build()
        .unwrap();

        assert_eq!(edn_rs::to_string(q), expected);
    }
}
