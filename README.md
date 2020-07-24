# Transistor

A Rust Crux Client crate/lib. For now, this crate intends to support 2 ways to interact with Crux:

- [x] Via `Docker` with a [`crux-standalone`](https://opencrux.com/docs#config-docker) version [docker-hub](https://hub.docker.com/r/juxt/crux-standalone).
- [ ] Via [`HTTP`](https://opencrux.com/docs#config-http) using the [`REST API`](https://opencrux.com/docs#restapi).
- [ ] Via kafka. Maybe?

> Other solutions may be added after the first release.

To add this crate to your project you should add the following line to `dependencies` in `Cargo.toml`:
>
> ```
[dependencies]
> transistor = "0.3.1"
> ```

To use `query` function:
>
> ```
[dependencies]
> transistor = "0.4.0-BETA"
> ```


* For information on Crux and how to use it, please follow the link to [opencrux](https://opencrux.com/docs#restapi). Note that the current crate version (`Docker only`) uses a few modified endpoints due to its Docker implementation.

* For examples on usage, please refer to [examples directory](https://github.com/naomijub/transistor/tree/master/examples).

## Creating a Crux Client
All operations with Transistor start in the module `client` with `Crux::new("localhost", "3000")`.  The struct `Crux` is responsabile for defining request `HeadersMap` and the request `URL`. The `URL` definition is required and it is done by the static function `new`, which receives as argument a `host` and a `port` and returns a `Crux` instance. To change `HeadersMap` info so that you can add `AUTHORIZATION` you can use the function `with_authorization` that receives as argument the authorization token and mutates the `Crux` instance.
* `HeaderMap` already contains the header `Content-Type: application/edn`.

Finally, to create a Crux Client the function `<type>_client` should be called, for example `docker_client`. This function returns a struct that contains all possible implementarions to query Crux Docker.
```rust
use transistor::client::Crux;

// DockerClient with AUTHORIZATION
let auth_client = Crux::new("127.0.0.1","3000").with_authorization("my-auth-token").docker_client();

// DockerClient without AUTHORIZATION
let client = Crux::new("127.0.0.1","3000").docker_client();
```

## Docker Client
Once you have called `docker_client` you will have an instance of the `DockerClient` struct which has a bunch of functions to query Crux on Docker:
* `state` queries endpoint [`/`](https://opencrux.com/docs#rest-home) with a `GET`. No args. Returns various details about the state of the database.
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

* `tx_log` requests endpoint [`/tx-log`](https://opencrux.com/docs#rest-tx-log-post) via `POST`. A Vector of `Action` is expected as argument. The "write" endpoint, to post transactions.
```rust
use transistor::docker::{Action};
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

* `tx_logs` requests endpoint [`/tx-log`](https://opencrux.com/docs#rest-tx-log) via `GET`. No args. Returns a list of all transactions.
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

* `entity` requests endpoint [`/entity`](https://opencrux.com/docs#rest-entity) via `POST`. A serialized `CruxId`, serialized `Edn::Key` or a String containing a [`keyword`](https://github.com/edn-format/edn#keywords) must be passed as argument. Returns an entity for a given ID and optional valid-time/transaction-time co-ordinates.
```rust
let person = Person {
    crux__db___id: CruxId::new("hello-entity"), 
    ...
};

let client = Crux::new("localhost", "3000").docker_client();

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

* `entity_tx` requests endpoint [`/entity-tx`](https://opencrux.com/docs#rest-entity-tx) via `POST`. A serialized `CruxId`, serialized `Edn::Key` or a String containing a [`keyword`](https://github.com/edn-format/edn#keywords) must be passed as argument. Returns the transaction that most recently set a key.
```rust
use transistor::docker::{Action};
use transistor::client::Crux;
use transistor::types::{CruxId};

let person = Person {
    crux__db___id: CruxId::new("hello-entity"), 
    ...
};

let client = Crux::new("localhost", "3000").docker_client();

let tx_body = client.entity_tx(person.crux__db___id.serialize()).unwrap();
// EntityTxResponse {
//     db___id: "d72ccae848ce3a371bd313865cedc3d20b1478ca",
//     db___content_hash: "1828ebf4466f98ea3f5252a58734208cd0414376",
//     db___valid_time: "2020-07-20T20:38:27.515-00:00",
//     tx___tx_id: 31,
//     tx___tx_time: "2020-07-20T20:38:27.515-00:00",
// }
```

* `document_by_id` requests endpoint [`/document/{:content-hash}`](https://opencrux.com/docs#rest-document) via `GET`. `{:content-hash}` can be obtained with function `entity_tx`. Returns the document for a given content hash.
```rust
use transistor::docker::{Action};
use transistor::client::Crux;
use transistor::types::{CruxId};

let person = Person {
    crux__db___id: CruxId::new("hello-entity"),
    first_name: "Hello".to_string(),
    last_name: "World".to_string()
};

let client = Crux::new("localhost", "3000").docker_client();

let document = client.document_by_id(tx_body.db___content_hash).unwrap();
// Person {
//     crux__db___id: CruxId(
//         ":hello-entity",
//     ),
//     first_name: "Hello",
//     last_name: "World",
// }
```

* `documents` requests endpoint [`/documents`](https://opencrux.com/docs#rest-documents) via `POST`. The argument of this reuqest is a vector of `content-hashes` that converts to an edn set as a body. Returns a map of document ids and respective documents for a given set of content hashes submitted in the request body.
```rust
use transistor::docker::{Action};
use transistor::client::Crux;
use transistor::types::{CruxId};

let person1 = Person {
    crux__db___id: CruxId::new("hello-entity"),
    ...
};

let person2 = Person {
    crux__db___id: CruxId::new("hello-documents"),
    ...
};

let client = Crux::new("localhost", "3000").docker_client();

let contesnt_hashes = vec![tx_body1.db___content_hash, tx_body2.db___content_hash];

let documents = client.documents(contesnt_hashes).unwrap();
// {
//     "1828ebf4466f98ea3f5252a58734208cd0414376": Map(
//         Map(
//             {
//                 ":crux.db/id": Key(
//                     ":hello-entity",
//                 ),
//                 ":first-name": Str(
//                     "Hello",
//                 ),
//                 ":last-name": Str(
//                     "World",
//                 ),
//             },
//         ),
//     ),
//     "1aeb98a4e11f30827e0304a9c289aad673b6cf57": Map(
//         Map(
//             {
//                 ":crux.db/id": Key(
//                     ":hello-documents",
//                 ),
//                 ":first-name": Str(
//                     "Hello",
//                 ),
//                 ":last-name": Str(
//                     "Documents",
//                 ),
//             },
//         ),
//     ),
// }
```
* `query` requests endpoint [`/query`](https://opencrux.com/docs#rest-query) via `POST`. Argument is a `query` of the type `Query`. Retrives a Set containing a vector of the values defined by the function `Query::find`.
```rust
use transistor::client::Crux;
use transistor::types::{query::Query};

let client = Crux::new("localhost", "3000").docker_client();

let query_is_sql = Query::find(vec!["p1", "n"])
    .where_clause(vec!["p1 :name n", "p1 :is-sql true"])
    .build();
// Query:
// {:query
//     {:find [p1 n]
//      :where [[p1 :name n]
//              [p1 :is-sql true]]}}

let is_sql = client.query(query_is_sql.unwrap()).unwrap();
// {[":mysql", "MySQL"], [":postgres", "Postgres"]} BTreeSet
```

`Action` is an enum with a set of options to use in association with the function `tx_log`:
* [`PUT`](https://opencrux.com/docs#transactions-put) - Write a version of a document
* [`Delete`](https://opencrux.com/docs#transactions-delete) - Deletes the specific document at a given valid time
* [`Evict`](https://opencrux.com/docs#transactions-evict) - Evicts a document entirely, including all historical versions (receives only the ID to evict)

`Query` is a struct responsible for creating the fields and serializing them into the correct `query` format. It has a function for each field and a `build` function to help check if it is correctyly formatted.
* `find` is a static function to define the elements inside the `:find` clause.
* `where_clause` is a function that defines the vector os elements inside the `:where []` array.


## Dependencies
A strong dependency of this crate is the [edn-rs](https://crates.io/crates/edn-rs) crate, as many of the return types are in the [Edn format](https://github.com/edn-format/edn). The sync http client is `reqwest` with `blocking` feature enabled.

## Licensing
This project is licensed under LGPP-3.0 (GNU Lesser General Public License v3.0).