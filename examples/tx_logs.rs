use transistor::client::Crux;
use transistor::types::response::TxLogsResponse;

fn tx_logs() -> TxLogsResponse {
    let body = Crux::new("localhost", "3000")
        .http_client()
        .tx_logs()
        .unwrap();

    return body;
}

#[test]
fn test_tx_logs() {
    let logs = tx_logs();
    assert!(logs.tx_events.len() > 0);
}

fn main() {
    let body = tx_logs();
    println!("Body = {:#?}", body);
    // Body = TxLogsResponse {
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
}
