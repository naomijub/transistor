/// Generic Request/Response Types. Availables types are:
/// * `CruxId` is the field that receives a String and serielizes it to a EDN Keyword.
/// * `StateResponse` response for Crux RESTapi at endpoint `/state`.
/// * `TxLogResponse` response for Crux RESTapi at endpoint `/tx-log`. For `POSTs`, `tx__event___tx_events (:crux-tx.event/tx_events)` comes with `None`.
/// * `TxLogsResponse` response is the wrapper for a `GET` at endpoint `/tx-logs`, it is a `Vector` of type `TxLogResponse`.
pub mod types;

/// Docker Client  module. It has the basic struct `Crux` which contains the [`CruxClient`](../transistor/docker/struct.CruxClient.html#impl) for Docker with the following functions:
/// * `state` queries endpoint `/` with a `GET`. No args.
/// * `tx_log` requests endpoint `/tx-log` via `POST`. A Vector of `Action` is expected as argument.
/// * `tx_logs` resquests endpoint `/tx-log` via `GET`. No args.
/// * `entity` requests endpoint `/entity` via `POST`. A serialized `CruxId`, serialized `Edn::Key` or a String containing a [`keyword`](https://github.com/edn-format/edn#keywords) must be passed as argument.
/// 
/// Enum [`Action`](../transistor/docker/enum.Action.html) is available in this module
pub mod docker;
// pub mod http;
// pub mod kafka;

pub use edn_rs;
