# Transistor

A Rust Crux Client crate/lib. For now, this crate intends to support 2 ways to interact with Crux:

- [x] Via `Docker` with a [`crux-standalone`](https://opencrux.com/reference/building.html#_docker) version [docker-hub](https://hub.docker.com/r/juxt/crux-standalone). Current Docker image `juxt/crux-standalone:20.07-1.10.0`.
- [x] Via [`HTTP`](https://opencrux.com/reference/http.html#start-http-server) using the [`HTTP API`](https://opencrux.com/reference/http.html#http-api).
- [x] Async support.
- [ ] Clojure.api. (To be evaluated.)
- [ ] FFI. (To be evaluated.)

> Other solutions may be added after the first release.

* [**Crux Getting Started**](https://opencrux.com/reference/get-started.html)
* [**Crux FAQs**](https://opencrux.com/about/faq.html)
* For examples on usage, please refer to [examples directory](https://github.com/naomijub/transistor/tree/master/examples) or to the [`ATM Crux`](https://github.com/naomijub/atm-crux) for more complete and interactive example.

## Bitemporal Crux

Crux is optimised for efficient and globally consistent point-in-time queries using a pair of transaction-time and valid-time timestamps.

Ad-hoc systems for bitemporal recordkeeping typically rely on explicitly tracking either valid-from and valid-to timestamps or range types directly within relations. The bitemporal document model that Crux provides is very simple to reason about and it is universal across the entire database, therefore it does not require you to consider which historical information is worth storing in special "bitemporal tables" upfront.

One or more documents may be inserted into Crux via a put transaction at a specific valid-time, defaulting to the transaction time (i.e. now), and each document remains valid until explicitly updated with a new version via put or deleted via delete.

### Why?

| Time 	| Purpose 	|
|-	|-	|
| transaction-time 	| Used for audit purposes, technical requirements such as event sourcing. 	|
| valid-time 	| Used for querying data across time, historical analysis. 	|

`transaction-time` represents the point at which data arrives into the database. This gives us an audit trail and we can see what the state of the database was at a particular point in time. You cannot write a new transaction with a transaction-time that is in the past.

`valid-time` is an arbitrary time that can originate from an upstream system, or by default is set to transaction-time. Valid time is what users will typically use for query purposes.

Reference [crux bitemporality](https://opencrux.com/about/bitemporality.html) and [value of bitemporality](https://juxt.pro/blog/posts/value-of-bitemporality.html)

## Usage 

To add this crate to your project you should add one of the following line to your `dependencies` field in `Cargo.toml`:
>
> ```
> [dependencies]
> transistor = "2.0.0"
> ```

## Creating a Crux Client
All operations with Transistor start in the module `client` with `Crux::new("localhost", "3000")`.  The struct `Crux` is responsabile for defining request `HeadersMap` and the request `URL`. The `URL` definition is required and it is done by the static function `new`, which receives as argument a `host` and a `port` and returns a `Crux` instance. To change `HeadersMap` info so that you can add `AUTHORIZATION` you can use the function `with_authorization` that receives as argument the authorization token and mutates the `Crux` instance.
* `HeaderMap` already contains the header `Content-Type: application/edn`.

Finally, to create a Crux Client the function `<type>_client` should be called, for example `http_client`. This function returns a struct that contains all possible implementarions to query Crux Docker and Standalone HTTP Server.
```rust
use transistor::client::Crux;

// HttpClient with AUTHORIZATION
let auth_client = Crux::new("127.0.0.1","3000").with_authorization("my-auth-token").http_client();

// HttpClient without AUTHORIZATION
let client = Crux::new("127.0.0.1","3000").http_client();
```

## Http Client
Once you have called `http_client` you will have an instance of the `HttpClient` struct which has a bunch of functions to query Crux on Docker and Standalone HTTP Server:

* [`state`](https://docs.rs/transistor/2.0.0/transistor/http/struct.HttpClient.html#method.state) queries endpoint [`/`](https://opencrux.com/reference/http.html#home) with a `GET`. No args. Returns various details about the state of the database.
```rust
let body = client.state().unwrap();

// StateResponse { 
//     index___index_version: 5, 
//     doc_log___consumer_state: None, 
//     tx_log___consumer_state: None, 
//     kv___kv_store: "crux.kv.rocksdb.RocksKv", 
//     kv___estimate_num_keys: 56, 
//     kv___size: 2271042 
// }
```

* [`tx_log`](https://docs.rs/transistor/2.0.0/transistor/http/struct.HttpClient.html#method.tx_log) requests endpoint [`/tx-log`](https://opencrux.com/reference/http.html#tx-log-post) via `POST`. `Actions` is expected as argument. The "write" endpoint, to post transactions.
```rust
use transistor::http::{Actions};
use transistor::client::Crux;
use transistor::types::{CruxId};

let person1 = Person {
    crux__db___id: CruxId::new("jorge-3"), 
    ..
};

let person2 = Person {
    crux__db___id: CruxId::new("manuel-1"), 
    ..
};

let actions = Actions::new()
    .append_put(person1)
    .append_put(person2);

let body = client.tx_log(actions).unwrap();
// {:crux.tx/tx-id 7, :crux.tx/tx-time #inst \"2020-07-16T21:50:39.309-00:00\"}
```

* [`tx_logs`](https://docs.rs/transistor/2.0.0/transistor/http/struct.HttpClient.html#method.tx_logs) requests endpoint [`/tx-log`](https://opencrux.com/reference/http.html#tx-log) via `GET`. No args. Returns a list of all transactions.
```rust
use transistor::client::Crux;

let body = client.tx_logs().unwrap();

// TxLogsResponse {
//     tx_events: [
//         TxLogResponse {
//             tx___tx_id: 0,
//             tx___tx_time: 2020-07-09T23:38:06.465-00:00,
//             tx__event___tx_events: Some(
//                 [
//                     [
//                         ":crux.tx/put",
//                         "a15f8b81a160b4eebe5c84e9e3b65c87b9b2f18e",
//                         "125d29eb3bed1bf51d64194601ad4ff93defe0e2",
//                     ],
//                 ],
//             ),
//         },
//         TxLogResponse {
//             tx___tx_id: 1,
//             tx___tx_time: 2020-07-09T23:39:33.815-00:00,
//             tx__event___tx_events: Some(
//                 [
//                     [
//                         ":crux.tx/put",
//                         "a15f8b81a160b4eebe5c84e9e3b65c87b9b2f18e",
//                         "1b42e0d5137e3833423f7bb958622bee29f91eee",
//                     ],
//                 ],
//             ),
//         },
//         ...
//     ]
// } 
```

* [`entity`](https://docs.rs/transistor/2.0.0/transistor/http/struct.HttpClient.html#method.entity) requests endpoint [`/entity`](https://opencrux.com/reference/http.html#entity) via `POST`. A serialized `CruxId`, serialized `Edn::Key` or a String containing a [`keyword`](https://github.com/edn-format/edn#keywords) must be passed as argument. Returns an entity for a given ID and optional valid-time/transaction-time co-ordinates.
```rust
let person = Person {
    crux__db___id: CruxId::new("hello-entity"), 
    ...
};

let client = Crux::new("localhost", "3000").http_client();

// entity expects a CruxId
let edn_body = client.entity(person.crux__db___id).unwrap();
// Map(
//     Map(
//         {
//             ":crux.db/id": Key(
//                 ":hello-entity",
//             ),
//             ":first-name": Str(
//                 "Hello",
//             ),
//             ":last-name": Str(
//                 "World",
//             ),
//         },
//     ),
// )
```

* [`entity_timed`](https://docs.rs/transistor/2.0.0/transistor/http/struct.HttpClient.html#method.entity_timed) is similar to `entity` as it requests the same endpoint, the difference is that it can send `transaction-time` and `valid-time` as query-params. This is done by the extra arguments `transaction_time: Option<DateTime<FixedOffset>>` and `valid_time: Option<DateTime<FixedOffset>>`.

* [`entity_tx`](https://docs.rs/transistor/2.0.0/transistor/http/struct.HttpClient.html#method.entity_tx) requests endpoint [`/entity-tx`](https://opencrux.com/reference/http.html#entity-tx) via `POST`. A serialized `CruxId`, serialized `Edn::Key` or a String containing a [`keyword`](https://github.com/edn-format/edn#keywords) must be passed as argument. Returns the transaction that most recently set a key.
```rust
use transistor::http::{Action};
use transistor::client::Crux;
use transistor::types::{CruxId};

let person = Person {
    crux__db___id: CruxId::new("hello-entity"), 
    ...
};

let client = Crux::new("localhost", "3000").http_client();

let tx_body = client.entity_tx(edn_rs::to_string(person.crux__db___id)).unwrap();
// EntityTxResponse {
//     db___id: "d72ccae848ce3a371bd313865cedc3d20b1478ca",
//     db___content_hash: "1828ebf4466f98ea3f5252a58734208cd0414376",
//     db___valid_time: 2020-07-20T20:38:27.515-00:00,
//     tx___tx_id: 31,
//     tx___tx_time: 2020-07-20T20:38:27.515-00:00,
// }
```

* [`entity_tx_timed`](https://docs.rs/transistor/2.0.0/transistor/http/struct.HttpClient.html#method.entity_tx_timed) is similar to `entity_tx` as it requests the same endpoint, the difference is that it can send `transaction-time` and `valid-time` as query-params. This is done by the extra arguments `transaction_time: Option<DateTime<FixedOffset>>` and `valid_time: Option<DateTime<FixedOffset>>`.

* [`entity_history`](https://docs.rs/transistor/2.0.0/transistor/http/struct.HttpClient.html#method.entity_history) requests endpoint [`/entity-history`](https://opencrux.com/reference/http.html#entity-history) via `GET`. Arguments are the `crux.db/id` as a `String`, an ordering argument defined by the enum `http::Order` (`Asc` or `Desc`) and a boolean for the `with-docs?` flag. The response is a Vector containing `EntityHistoryElement`. If `with-docs?` is `true`, thank the field `db__doc`, `:crux.db/doc`, witll return an `Option<Edn>` containing the inserted struct.
```rust
use transistor::client::Crux;
use transistor::http::Order;
use transistor::types::CruxId;

let person = Person {
    crux__db___id: CruxId::new("hello-history"),
    ...

let client = Crux::new("localhost", "3000").http_client();

let tx_body = client.entity_tx(person.crux__db___id).unwrap();

let entity_history = client.entity_history(tx_body.db___id.clone(), Order::Asc, true);
// EntityHistoryResponse { history: [
//     EntityHistoryElement { 
//         db___valid_time: 2020-08-05T03:00:06.476-00:00, 
//         tx___tx_id: 37, tx___tx_time: 2020-08-05T03:00:06.476-00:00, 
//         db___content_hash: "2da097a2dffbb9828cd4377f1461a59e8454674b", 
//         db__doc: Some(Map(Map(
//                 {":crux.db/id": Key(":hello-history"), 
//                 ":first-name": Str("Hello"), 
//                 ":last-name": Str("World")}
//                ))) 
//     }
// ]}

let entity_history_without_docs = client.entity_history(tx_body.db___id, Order::Asc, false);
// EntityHistoryResponse { 
//     history: [
//         EntityHistoryElement {
//              db___valid_time: 2020-08-05T03:00:06.476-00:00, 
//              tx___tx_id: 37, 
//              tx___tx_time: 2020-08-05T03:00:06.476-00:00, 
//              db___content_hash: "2da097a2dffbb9828cd4377f1461a59e8454674b", 
//              db__doc: None 
//             }
//         }
//     ]}
```

* [`entity_history_timed`](https://docs.rs/transistor/2.0.0/transistor/http/struct.HttpClient.html#method.entity_history_timed) is similar to `entity_histoty` as it requests the same endpoint, the difference is that it can send `start-transaction-time`, `end-transaction-time`, `start-valid-time` and `end-valid-time` as query-params. This is done by adding a `Vec<TimeHistory>` containing one `TimeHistory::TransactionTime` and/or one `TimeHistory::ValidTime`, both of them receive two `Option<DateTime<Utc>>`. The first `DateTime` is the `start-<type>-time` and the second is the `end-<type>-time`.


* [`query`](https://docs.rs/transistor/2.0.0/transistor/http/struct.HttpClient.html#method.query) requests endpoint [`/query`](https://opencrux.com/reference/http.html#query) via `POST`. Argument is a `query` of the type `Query`. Retrives a Set containing a vector of the values defined by the function `Query::find`.
Available functions are `find`, `where_clause`, `args`, `order_by`, `limit`, `offset`, examples [`complex_query`](https://github.com/naomijub/transistor/blob/master/examples/complex_query.rs) and [`limit_offset_query`](https://github.com/naomijub/transistor/blob/master/examples/limit_offset_query.rs) have examples on how to use them.
```rust
use transistor::client::Crux;
use transistor::types::{query::Query};

let client = Crux::new("localhost", "3000").http_client();

let query_is_sql = Query::find(vec!["?p1", "?n"])
    .where_clause(vec!["?p1 :name ?n", "?p1 :is-sql true"])
    .build();
// Query:
// {:query
//     {:find [?p1 ?n]
//      :where [[?p1 :name ?n]
//              [?p1 :is-sql true]]}}

let is_sql = client.query(query_is_sql.unwrap()).unwrap();
// {[":mysql", "MySQL"], [":postgres", "Postgres"]} BTreeSet
```

### Transisitor's Structs and Enums

[`Actions`](https://docs.rs/transistor/2.0.0/transistor/http/enum.Actions.html) is a builder struct to help you create a `Vec<Action>` for `tx_log`. Available functions are:
* `new` static method to instantiate struct `Actions`.
* `append_put<T: Serialize>(action: T)` appends a [`Put`](https://opencrux.com/reference/transactions.html#put) to `Actions` with no `valid-time`. `Put` writes a document.
* `append_put_timed<T: Serialize>(action: T, date: DateTime<FixedOffset>)` appends a [`Put`](https://opencrux.com/reference/transactions.html#put) to `Actions` with `valid-time`.
* `append_delete(id: CruxId)` appends a [`Delete`](https://opencrux.com/reference/transactions.html#delete) to `Actions` with no `valid-time`. Deletes the specific document at last `valid-time`.
* `append_delete_timed(id: CruxId, date: DateTime<FixedOffset>)` appends a [`Delete`](https://opencrux.com/reference/transactions.html#delete)  to `Actions` with `valid-time`. Deletes the specific document at the given `valid-time`.
* `append_evict(id: CruxId)` appends an [`Evict`](https://opencrux.com/reference/transactions.html#evict) to `Actions`. Evicts a document entirely, including all historical versions (receives only the ID to evict).
* `append_match_doc<T: Serialize>(id: CruxId, action: T)` appends a [`Match`](https://opencrux.com/reference/transactions.html#match) to `Actions` with no `valid-time`. Matches the current state of an entity, if the state doesn't match the provided document, the transaction will not continue.
* `append_match_doc_timed<T: Serialize>(id: CruxId, action: T, date: DateTime<FixedOffset>)` appends a [`Match`](https://opencrux.com/reference/transactions.html#match) to `Actions` with `valid-time`.
* `build` generates the `Vec<Action>` from `Actions`

```rust
use transistor::client::Crux;
use transistor::types::http::Actions;

fn main() -> Result<(), CruxError> {
    let crux = Database {
        // ...
    };

    let psql = Database {
        // ...
    };

    let mysql = Database {
        // ...
    };

    let cassandra = Database {
        // ...
    };

    let sqlserver = Database {
        // ...
    };

    let client = Crux::new("localhost", "3000").http_client();
    let timed = "2014-11-28T21:00:09-09:00"
        .parse::<DateTime<FixedOffset>>()
        .unwrap();

    let actions: Vec<Action> = Actions::new()
        .append_put(crux)
        .append_put(psql)
        .append_put(mysql)
        .append_put_timed(cassandra, timed)
        .append_put(sqlserver)
        .build();

    let _ = client.tx_log(actions)?;
}
```

[`Query`](https://docs.rs/transistor/2.0.0/transistor/types/query/struct.Query.html) is a struct responsible for creating the fields and serializing them into the correct `query` format. It has a function for each field and a `build` function to help check if it is correctyly formatted.
* `find` is a static builder function to define the elements inside the `:find` clause.
* `where_clause` is a builder function that defines the vector os elements inside the `:where []` array.
* `order_by` is a builder function to define the elements inside the `:order-by` clause.
* `args` is a builder function to define the elements inside the `:args` clause.
* `limit` is a builder function to define the elements inside the `:limit` clause.
* `offset` is a builder function to define the elements inside the `:offset` clause.
* `with_full_results` is a builder function to define the flag `full-results?` as true. This allows your `query` response to return the whole document instead of only the searched keys. The result of the Query `{:query {:find [?user ?a] :where [[?user :first-name ?a]] :full-results? true}}` will be a `BTreeSet<Vec<String>>` like `([{:crux.db/id :fafilda, :first-name "Jorge", :last-name "Klaus"} "Jorge"])`, so the document will need further EDN parsing to become the document's struct.

Errors are defined in the [`CruxError`](https://docs.rs/transistor/2.0.0/transistor/types/error/enum.CruxError.html) enum.
* `EdnError` is a wrapper over `edn_rs::EdnError`.
* `RequestError` is originated by `reqwest` crate. Failed to make HTTP request.
* `QueryFormatError` is originated when the provided Query struct did not match schema.
* `QueryError` is responsible for encapsulation the Stacktrace error from Crux response:

```rust
use transistor::client::Crux;
use transistor::types::{query::Query};

let _client = Crux::new("localhost", "3000").http_client();

// field `n` doesn't exist
let _query_error_response = Query::find(vec!["?p1", "?n"])
    .where_clause(vec!["?p1 :name ?g", "?p1 :is-sql true"])
    .build();

let error = client.query(query_error_response?)?;
println!("Stacktrace \n{:?}", error);

// Stacktrace
// QueryError("{:via
//      [{:type java.lang.IllegalArgumentException,
//        :message \"Find refers to unknown variable: n\",
//    :at [crux.query$q invokeStatic \"query.clj\" 1152]}],
//  :trace
//  [[crux.query$q invokeStatic \"query.clj\" 1152]
//   [crux.query$q invoke \"query.clj\" 1099]
//   [crux.query$q$fn__10850 invoke \"query.clj\" 1107]
//   [clojure.core$binding_conveyor_fn$fn__5754 invoke \"core.clj\" 2030]
//   [clojure.lang.AFn call \"AFn.java\" 18]
//   [java.util.concurrent.FutureTask run \"FutureTask.java\" 264]
//   [java.util.concurrent.ThreadPoolExecutor
//    runWorker
//    \"ThreadPoolExecutor.java\"
//    1128]
//   [java.util.concurrent.ThreadPoolExecutor$Worker
//    run
//    \"ThreadPoolExecutor.java\"
//    628]
//   [java.lang.Thread run \"Thread.java\" 834]],
//  :cause \"Find refers to unknown variable: n\"}
// ")
```

### Testing the Crux Client

For testing purpose there is a `feature` called `mock` that enables the `http_mock` function that is a replacement for the `http_client` function. To use it run your commands with the the flag `--features "mock"` as in `cargo test --test lib --no-fail-fast --features "mock"`. The mocking feature uses the crate `mockito = "0.26"` as a Cargo dependency. An example usage with this feature enabled:

```rust
use transistor::client::Crux;
use transistor::http::Action;
use edn_derive::Serialize;
use transistor::types::{CruxId};
use mockito::mock;

#[test]
#[cfg(feature = "mock")]
fn mock_client() {
    let _m = mock("POST", "/tx-log")
        .with_status(200)
        .match_body("[[:crux.tx/put { :crux.db/id :jorge-3, :first-name \"Michael\", :last-name \"Jorge\", }], [:crux.tx/put { :crux.db/id :manuel-1, :first-name \"Diego\", :last-name \"Manuel\", }]]")
        .with_header("content-type", "text/plain")
        .with_body("{:crux.tx/tx-id 8, :crux.tx/tx-time #inst \"2020-07-16T21:53:14.628-00:00\"}")
        .create();

    let person1 = Person {
        // ...
    };

    let person2 = Person {
        /// ...
    };

    let actions = vec![Action::put(person1), Action::put(person2)];
    
    let body = Crux::new("localhost", "3000")
        .http_mock()
        .tx_log(actions)
        .unwrap();

    assert_eq!(
        format!("{:?}", body),
        String::from("TxLogResponse { tx___tx_id: 8, tx___tx_time: 2020-07-16T21:53:14.628-00:00, tx__event___tx_events: None }")
    );
}

#[derive(Debug, Clone, Serialize)]
#[allow(non_snake_case)]
pub struct Person {
    crux__db___id: CruxId,
    // ...
}

```

Also, struct `Actions` can be tested with feature `mock` by using enum `ActionMock` due to the implementation of `impl PartialEq<Vec<ActionMock>> for Actions`. A demo example can be:

```rust
use transistor::types::http::{Actions, ActionMock};

fn test_actions_eq_actions_mock() {
    let actions = test_actions();
    let mock = test_action_mock();

    assert_eq!(actions, mock);
}

fn test_action_mock() -> Vec<ActionMock> {
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

    vec![
        ActionMock::Put(edn_rs::to_string(person1.clone()), None),
        ActionMock::Put(edn_rs::to_string(person2), None),
        ActionMock::Delete(edn_rs::to_string(person1.crux__db___id), None),
    ]
}

fn test_actions() -> Actions {
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
    Actions::new().append_put(person1.clone()).append_put(person2).append_delete(person1.crux__db___id)
}
```

### Async support

**Async feature is still in BETA** as it depends heavily on `unwraps`.

It is possible to use `async/await` http client, for that it is necessary to enable feature `async` in transistor, `transistor = { version = "2.0.0", features = ["async"] }`. With this feature enabled the `HttpClient` will use `reqwest::Client` instead of `reqwest::blocking::Client`. The default async runtime for `reqwest::Client` is `tokio`, so it is good to have `tokio` with feature `macros`, as well as `futures`, in your `Cargo.toml`:

```toml
futures = {version = "0.3.5" }
tokio = {version = "0.2.22", features = ["macros"] }
```

An async query example can be found below:

```rust
use tokio::prelude::*;
use transistor::client::Crux;
use edn_derive::Serialize;
use transistor::types::http::Action;
use transistor::types::{
    error::CruxError,
    {query::Query, CruxId},
};

#[tokio::main]
async fn main() {
    let crux = Database {
        crux__db___id: CruxId::new("crux"),
        name: "Crux Datalog".to_string(),
        is_sql: false,
    };

    let psql = Database {
        crux__db___id: CruxId::new("postgres"),
        name: "Postgres".to_string(),
        is_sql: true,
    };

    let mysql = Database {
        crux__db___id: CruxId::new("mysql"),
        name: "MySQL".to_string(),
        is_sql: true,
    };

    let client = Crux::new("localhost", "3000").http_client();
    let action1 = Action::put(crux, None);
    let action2 = Action::put(psql, None);
    let action3 = Action::put(mysql, None);

    let _ = client.tx_log(vec![action1, action2, action3]).await;

    let query_is_sql = Query::find(vec!["?p1", "?n"])
        .unwrap()
        .where_clause(vec!["?p1 :name ?n", "?p1 :is-sql true"])
        .unwrap()
        .build();

    let is_sql = client.query(query_is_sql.unwrap()).await;

    let query_is_no_sql = Query::find(vec!["?p1", "?n", "?s"])
        .unwrap()
        .where_clause(vec!["?p1 :name ?n", "?p1 :is-sql ?s", "?p1 :is-sql false"])
        .unwrap()
        .with_full_results()
        .build();

    let is_no_sql = client.query(query_is_no_sql.unwrap()).await;
}

#[derive(Debug, Clone, Serialize)]
#[allow(non_snake_case)]
pub struct Database {
    crux__db___id: CruxId,
    name: String,
    is_sql: bool
}

```

Note `use tokio::prelude::*;` and `#[tokio::main] \n async fn main()`.

## Enababling feature `time_as_str`
It is possible to use receive the responses (`TxLogResponse`, `EntityTxResponse`, `EntityHistoryElement`) time dates as Strings, to do so you have to enable feature `time_as_str`:

```toml
transistor = { version = "2.0.0", features = ["time_as_str"] }
```

## Possible Features
```
mock = ["mockito"] -> http_mock()
time_as_str = [] -> DataTime types become Strings
async = ["tokio", "futures"] -> async/await
```

## Dependencies
A strong dependency of this crate is the [edn-rs](https://crates.io/crates/edn-rs) crate, as many of the return types are in the [Edn format](https://github.com/edn-format/edn), also the [edn-derive](https://crates.io/crates/edn-derive). The sync http client is `reqwest` with `blocking` feature enabled. `Chrono` for time values that can be `DateTime<Utc>`, for inserts, and `DateTime<FixedOffset>`, for reads, and `mockito` for feature `mock`.

## Licensing
This project is licensed under LGPP-3.0 (GNU Lesser General Public License v3.0).
