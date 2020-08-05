pub use edn_rs;

/// Generic Request/Response Types for Crux.
/// Availables types are:
/// * `CruxId` is the field that receives a String and serielizes it to a EDN Keyword.
///
/// Availables types for `response` are:
/// * `StateResponse` response for Crux RESTapi at endpoint `/state`.
/// * `TxLogResponse` response for Crux RESTapi at endpoint `/tx-log`. For `POSTs`, `tx__event___tx_events (:crux-tx.event/tx_events)` comes with `None`.
/// * `TxLogsResponse` response is the wrapper for a `GET` at endpoint `/tx-logs`, it is a `Vector` of type `TxLogResponse`.
pub mod types;

/// Http Client  module. It contains the [`HttpClient`](../http/struct.HttpClient.html#impl) for Docker and Standalone HTTP Server.
///
/// `HttpClient` Contains the following functions:
/// * `state` queries endpoint `/` with a `GET`. No args.
/// * `tx_log` requests endpoint `/tx-log` via `POST`. A Vector of `Action` is expected as argument.
/// * `tx_logs` requests endpoint `/tx-log` via `GET`. No args.
/// * `entity` requests endpoint `/entity` via `POST`. A serialized `CruxId`, serialized `Edn::Key` or a String containing a [`keyword`](https://github.com/edn-format/edn#keywords) must be passed as argument.
/// * `entity_tx` requests endpoint `/entity-tx` via `POST`. A serialized `CruxId`, serialized `Edn::Key` or a String containing a [`keyword`](https://github.com/edn-format/edn#keywords) must be passed as argument.
/// * `entity_history` requests endpoint `/entity-history` via `GET`. Arguments are the `crux.db/id` as a `String`, an ordering argument defined by the enum `http::Order` (`Asc` or `Desc`) and a boolean for the `with-docs?` flag (this returns values for the field `:crux.db/doc`).
/// * `query` requests endpoint `/query` via `POST`. Argument is a `query` of the type `Query`. Retrives a Set containing a vector of the values defined by the function `Query::find`.
///
/// Enum [`Action`](../http/enum.Action.html) is available in this module.
/// Enum `Order` is available in this module to be used with `entity_history`.
///
/// Examples can be found in the [examples directory](https://github.com/naomijub/transistor/tree/master/examples).
pub mod http;
// pub mod kafka;

/// This module contains the basic client, struct `Crux`, which configures `host:port` and `authorization`, and returns the needed `client`.
pub mod client;
