/// Generic Request/Response Types for Crux. 
/// Availables types are:
/// * `CruxId` is the field that receives a String and serielizes it to a EDN Keyword.
/// * `StateResponse` response for Crux RESTapi at endpoint `/state`.
/// * `TxLogResponse` response for Crux RESTapi at endpoint `/tx-log`. For `POSTs`, `tx__event___tx_events (:crux-tx.event/tx_events)` comes with `None`.
/// * `TxLogsResponse` response is the wrapper for a `GET` at endpoint `/tx-logs`, it is a `Vector` of type `TxLogResponse`.
pub mod types;

/// Docker Client  module. It has the basic struct `Crux` which contains the [`CruxClient`](../docker/struct.CruxClient.html#impl) for Docker.
/// Contains the following functions:
/// * `state` queries endpoint `/` with a `GET`. No args.
/// * `tx_log` requests endpoint `/tx-log` via `POST`. A Vector of `Action` is expected as argument.
/// * `tx_logs` resquests endpoint `/tx-log` via `GET`. No args.
/// * `entity` requests endpoint `/entity` via `POST`. A serialized `CruxId`, serialized `Edn::Key` or a String containing a [`keyword`](https://github.com/edn-format/edn#keywords) must be passed as argument.
/// * `document_by_id` resquests endpoint `/document/{:content-hash}` via `GET`. `{:content-hash}` can be obtained with function `entity_tx`.
/// * `documents` resquests endpoint `/documents` via `POST`. The argument of this reuqest is a vector of `content-hashes` that converts to an edn set as a body.
/// 
/// Enum [`Action`](../docker/enum.Action.html) is available in this module
pub mod docker;
// pub mod http;
// pub mod kafka;

pub use edn_rs;
