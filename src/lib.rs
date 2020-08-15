pub use edn_rs;

/// Generic Request/Response Types for Crux.
/// Availables types are:
/// * `CruxId` is the field that receives a String and serielizes it to a EDN Keyword.
///
/// Availables types for responses in module `types::response` are:
/// * `StateResponse` response for Crux REST API at endpoint `/state`.
/// * `TxLogResponse` response for Crux REST API at endpoint `/tx-log`. For `POSTs`, `tx__event___tx_events (:crux-tx.event/tx_events)` comes with `None`.
/// * `TxLogsResponse` response is the wrapper for a `GET` at endpoint `/tx-logs`, it is a `Vector` of type `TxLogResponse`.
/// * `EntityTxResponse` response for Crux REST API at `/entity-tx` endpoint.
/// * `EntityHistoryResponse` response for Crux REST API at `/entity-history`.
///
/// Available auxiliary Enums for HTTP in module `types::http`
/// Enum [`Action`](../types/http/enum.Action.html) is available in this module.
/// Enum [`Order`](../types/http/enum.Order.html)  is available in this module to be used with `entity_history`.
/// Enum [`TimeHistory`](../types/http/enum.TimeHistory.html)  is available in this module to be used with `entity_history_timed`.
///
/// It is possible to use `chrono`  for time related responses (`TxLogResponse`, `EntityTxResponse`, `EntityHistoryElement`). to use it you need to enable feature `"time".
pub mod types;

/// Http Client  module. It contains the [`HttpClient`](../http/struct.HttpClient.html#impl) for Docker and Standalone HTTP Server.
///
/// `HttpClient` Contains the following functions:
/// * `tx_log` requests endpoint `/tx-log` via `POST`. A Vector of `types::http::Action` is expected as argument.
/// * `tx_logs` requests endpoint `/tx-log` via `GET`. No args.
/// * `entity` requests endpoint `/entity` via `POST`. A serialized `CruxId`, serialized `Edn::Key` or a String containing a [`keyword`](https://github.com/edn-format/edn#keywords) must be passed as argument.
/// * `entity_timed` similar to `entity`, but receives as arguments `transaction_time: Option<DateTime<FixedOffset>>` and `valid_time: Option<DateTime<FixedOffset>>,`.
/// * `entity_tx` requests endpoint `/entity-tx` via `POST`. A serialized `CruxId`, serialized `Edn::Key` or a String containing a [`keyword`](https://github.com/edn-format/edn#keywords) must be passed as argument.
/// * `entity_tx_timed` similar to `entity_tx`, but receives as arguments `transaction_time: Option<DateTime<FixedOffset>>` and `valid_time: Option<DateTime<FixedOffset>>,`.
/// * `entity_history` requests endpoint `/entity-history` via `GET`. Arguments are the `crux.db/id` as a `String`, an ordering argument defined by the enum `types::http::Order` (`Asc` or `Desc`) and a boolean for the `with-docs?` flag (this returns values for the field `:crux.db/doc`).
/// * `entity_history_timed` similar to `entity_history`, but receives one more argument that is a `Vec<TimeHistory>` to define `valid-time` and `transaction-time`
/// * `query` requests endpoint `/query` via `POST`. Argument is a `query` of the type `Query`. Retrives a Set containing a vector of the values defined by the function `Query::find`.
///
/// Examples can be found in the [examples directory](https://github.com/naomijub/transistor/tree/master/examples).
pub mod http;

/// This module contains the basic client, struct `Crux`, which configures `host:port` and `authorization`, and returns the needed `client`.
pub mod client;
