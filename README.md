# Transistor

A Rust Crux Client crate/lib. For now, this crate intends to support 2 ways to interact with Crux:

- [x] Via `Docker` with a [`crux-standalone`](https://opencrux.com/docs#config-docker) version [docker-hub](https://hub.docker.com/r/juxt/crux-standalone). Current Docker image `juxt/crux-standalone:20.07-1.10.0`.
- [x] Via [`HTTP`](https://opencrux.com/docs#config-http) using the [`REST API`](https://opencrux.com/docs#restapi).
- [ ] Via kafka. (To be evaluated.)

> Other solutions may be added after the first release.

* For information on Crux and how to use it, please follow the link to [opencrux](https://opencrux.com/docs#restapi). Note that the documentation for the REST API isn't completly up to date, so `document` endpoints don't exist since [changelog 20.06-1.9.0](https://github.com/juxt/crux/releases/tag/20.06-1.9.0).
* [**Crux FAQ**](https://opencrux.com/docs#faqs)
* For examples on usage, please refer to [examples directory](https://github.com/naomijub/transistor/tree/master/examples) or to the [`ATM Crux` (under development)](https://github.com/naomijub/atm-crux) for more complete and interactive example.

## Usage 

To add this crate to your project you should add one of the following line to your `dependencies` field in `Cargo.toml`:
>
> ```
> [dependencies]
> transistor = "1.0.0-beta.2"
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
* [`state`](https://docs.rs/transistor/1.0.0-beta.2/transistor/http/struct.HttpClient.html#method.state) queries endpoint [`/`](https://opencrux.com/docs#rest-home) with a `GET`. No args. Returns various details about the state of the database.
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

* [`tx_log`](https://docs.rs/transistor/1.0.0-beta.2/transistor/http/struct.HttpClient.html#method.tx_log) requests endpoint [`/tx-log`](https://opencrux.com/docs#rest-tx-log-post) via `POST`. A Vector of `Action` is expected as argument. The "write" endpoint, to post transactions.
```rust
use transistor::http::{Action};
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

let action1 = Action::Put(person1.serialize());
let action2 = Action::Put(person2.serialize());

let body = client.tx_log(vec![action1, action2]).unwrap();
// {:crux.tx/tx-id 7, :crux.tx/tx-time #inst \"2020-07-16T21:50:39.309-00:00\"}
```

* [`tx_logs`](https://docs.rs/transistor/1.0.0-beta.2/transistor/http/struct.HttpClient.html#method.tx_logs) requests endpoint [`/tx-log`](https://opencrux.com/docs#rest-tx-log) via `GET`. No args. Returns a list of all transactions.
```rust
use transistor::client::Crux;

let body = client.tx_logs().unwrap();

// TxLogsResponse {
//     tx_events: [
//         TxLogResponse {
//             tx___tx_id: 0,
//             tx___tx_time: "2020-07-09T23:38:06.465-00:00",
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
//             tx___tx_time: "2020-07-09T23:39:33.815-00:00",
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

* [`entity`](https://docs.rs/transistor/1.0.0-beta.2/transistor/http/struct.HttpClient.html#method.entity) requests endpoint [`/entity`](https://opencrux.com/docs#rest-entity) via `POST`. A serialized `CruxId`, serialized `Edn::Key` or a String containing a [`keyword`](https://github.com/edn-format/edn#keywords) must be passed as argument. Returns an entity for a given ID and optional valid-time/transaction-time co-ordinates.
```rust
let person = Person {
    crux__db___id: CruxId::new("hello-entity"), 
    ...
};

let client = Crux::new("localhost", "3000").http_client();

let edn_body = client.entity(person.crux__db___id.serialize()).unwrap();
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

* [`entity_tx`](https://docs.rs/transistor/1.0.0-beta.2/transistor/http/struct.HttpClient.html#method.entity_tx) requests endpoint [`/entity-tx`](https://opencrux.com/docs#rest-entity-tx) via `POST`. A serialized `CruxId`, serialized `Edn::Key` or a String containing a [`keyword`](https://github.com/edn-format/edn#keywords) must be passed as argument. Returns the transaction that most recently set a key.
```rust
use transistor::http::{Action};
use transistor::client::Crux;
use transistor::types::{CruxId};

let person = Person {
    crux__db___id: CruxId::new("hello-entity"), 
    ...
};

let client = Crux::new("localhost", "3000").http_client();

let tx_body = client.entity_tx(person.crux__db___id.serialize()).unwrap();
// EntityTxResponse {
//     db___id: "d72ccae848ce3a371bd313865cedc3d20b1478ca",
//     db___content_hash: "1828ebf4466f98ea3f5252a58734208cd0414376",
//     db___valid_time: "2020-07-20T20:38:27.515-00:00",
//     tx___tx_id: 31,
//     tx___tx_time: "2020-07-20T20:38:27.515-00:00",
// }
```

* [`entity_history`](https://docs.rs/transistor/1.0.0-beta.2/transistor/http/struct.HttpClient.html#method.entity_history) requests endpoint [`/entity-history`](https://opencrux.com/docs#rest-entity) via `GET`. Arguments are the `crux.db/id` as a `String`, an ordering argument defined by the enum `http::Order` (`Asc` or `Desc`) and a boolean for the `with-docs?` flag. The response is a Vector containing `EntityHistoryElement`. If `with-docs?` is `true`, thank the field `db__doc`, `:crux.db/doc`, witll return an `Option<Edn>` containing the inserted struct.
```rust
use transistor::client::Crux;
use transistor::http::Order;
use transistor::types::CruxId;

let person = Person {
    crux__db___id: CruxId::new("hello-history"),
    ...

let client = Crux::new("localhost", "3000").http_client();

let tx_body = client.entity_tx(person.crux__db___id.serialize()).unwrap();

let entity_history = client.entity_history(tx_body.db___id.clone(), Order::Asc, true);
// EntityHistoryResponse { history: [
//     EntityHistoryElement { 
//         db___valid_time: "2020-08-05T03:00:06.476-00:00", 
//         tx___tx_id: 37, tx___tx_time: "2020-08-05T03:00:06.476-00:00", 
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
//              db___valid_time: "2020-08-05T03:00:06.476-00:00", 
//              tx___tx_id: 37, 
//              tx___tx_time: "2020-08-05T03:00:06.476-00:00", 
//              db___content_hash: "2da097a2dffbb9828cd4377f1461a59e8454674b", 
//              db__doc: None 
//             }
//         }
//     ]}
```


* [`query`](https://docs.rs/transistor/1.0.0-beta.2/transistor/http/struct.HttpClient.html#method.query) requests endpoint [`/query`](https://opencrux.com/docs#rest-query) via `POST`. Argument is a `query` of the type `Query`. Retrives a Set containing a vector of the values defined by the function `Query::find`.
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

[`Action`](https://docs.rs/transistor/1.0.0-beta.2/transistor/http/enum.Action.html) is an enum with a set of options to use in association with the function `tx_log`:
* [`PUT`](https://opencrux.com/docs#transactions-put) - Write a version of a document
* [`Delete`](https://opencrux.com/docs#transactions-delete) - Deletes the specific document at a given valid time
* [`Evict`](https://opencrux.com/docs#transactions-evict) - Evicts a document entirely, including all historical versions (receives only the ID to evict)

[`Query`](https://docs.rs/transistor/1.0.0-beta.2/transistor/types/query/struct.Query.html) is a struct responsible for creating the fields and serializing them into the correct `query` format. It has a function for each field and a `build` function to help check if it is correctyly formatted.
* `find` is a static builder function to define the elements inside the `:find` clause.
* `where_clause` is a builder function that defines the vector os elements inside the `:where []` array.
* `order_by` is a builder function to define the elements inside the `:order-by` clause.
* `args` is a builder function to define the elements inside the `:args` clause.
* `limit` is a builder function to define the elements inside the `:limit` clause.
* `offset` is a builder function to define the elements inside the `:offset` clause.

Errors are defined in the [`CruxError`](https://docs.rs/transistor/1.0.0-beta.2/transistor/types/error/enum.CruxError.html) enum.
* `ParseEdnError` is originated by `edn_rs` crate. The provided EDN did not match schema.
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
use transistor::edn_rs::{ser_struct, Serialize};
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

    let actions = vec![Action::Put(person1.serialize()), Action::Put(person2.serialize())];
    
    let body = Crux::new("localhost", "3000")
        .http_mock()
        .tx_log(actions)
        .unwrap();

    assert_eq!(
        format!("{:?}", body),
        String::from("TxLogResponse { tx___tx_id: 8, tx___tx_time: \"2020-07-16T21:53:14.628-00:00\", tx__event___tx_events: None }")
    );
}

ser_struct! {
    #[derive(Debug, Clone)]
    #[allow(non_snake_case)]
    pub struct Person {
        crux__db___id: CruxId,
        // ...
    }
}

```

## Using `Chrono`
It is possible to use `chrono`  for time related responses (`TxLogResponse`, `EntityTxResponse`, `EntityHistoryElement`). to use it you need to enable feature `"time"`:

```toml
transistor = { version = "1.0.0-beta.2", features = ["time"] }
```

By doing this, all field named `time` will contain a `DateTime<Utc>` value.

## Dependencies
A strong dependency of this crate is the [edn-rs](https://crates.io/crates/edn-rs) crate, as many of the return types are in the [Edn format](https://github.com/edn-format/edn). The sync http client is `reqwest` with `blocking` feature enabled. `Chrono` for feature `time` so that time values are a `DateTime<Utc>` and `mockito` for feature `mock`.

## Licensing
This project is licensed under LGPP-3.0 (GNU Lesser General Public License v3.0).