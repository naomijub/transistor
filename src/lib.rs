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

/// Docker Client  module. It contains the [`DockerClient`](../docker/struct.DockerClient.html#impl) for Docker.
///
/// `DockerClient` Contains the following functions:
/// * `state` queries endpoint `/` with a `GET`. No args.
/// * `tx_log` requests endpoint `/tx-log` via `POST`. A Vector of `Action` is expected as argument.
/// * `tx_logs` requests endpoint `/tx-log` via `GET`. No args.
/// * `entity` requests endpoint `/entity` via `POST`. A serialized `CruxId`, serialized `Edn::Key` or a String containing a [`keyword`](https://github.com/edn-format/edn#keywords) must be passed as argument.
/// * `entity_tx` requests endpoint `/entity-tx` via `POST`. A serialized `CruxId`, serialized `Edn::Key` or a String containing a [`keyword`](https://github.com/edn-format/edn#keywords) must be passed as argument.
/// * `query` requests endpoint `/query` via `POST`. Argument is a `query` of the type `Query`. Retrives a Set containing a vector of the values defined by the function `Query::find`.
///
/// Enum [`Action`](../docker/enum.Action.html) is available in this module.
///
/// Examples can be found in the [examples directory](https://github.com/naomijub/transistor/tree/master/examples).
pub mod docker;
// pub mod http;
// pub mod kafka;

/// This module contains the basic client, struct `Crux`, which configures `host:port` and `authorization`, and returns the needed `client`.
pub mod client;
