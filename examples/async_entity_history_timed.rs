use chrono::prelude::*;
use transistor::client::Crux;
use transistor::edn_rs::{ser_struct, Serialize};
use transistor::types::http::{Action, Order, TimeHistory};
use transistor::types::CruxId;

#[tokio::main]
async fn main() {
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

    let client = Crux::new("localhost", "3000").http_client();
    let timed = "2014-11-28T21:00:09-09:00"
        .parse::<DateTime<FixedOffset>>()
        .unwrap();

    let start_timed = "2013-11-28T21:00:09-09:00"
        .parse::<DateTime<Utc>>()
        .unwrap();
    let end_timed = "2015-11-28T21:00:09-09:00"
        .parse::<DateTime<Utc>>()
        .unwrap();
    let time_history = TimeHistory::ValidTime(Some(start_timed), Some(end_timed));

    let action1 = Action::put(person1.clone(), Some(timed));
    let action2 = Action::put(person2, Some(timed));

    let _ = Crux::new("localhost", "3000")
        .http_client()
        .tx_log(vec![action1, action2])
        .await
        .unwrap();

    let tx_body = client.entity_tx(person1.crux__db___id).await.unwrap();

    let entity_history = client
        .entity_history_timed(
            tx_body.db___id.clone(),
            Order::Asc,
            true,
            vec![time_history],
        )
        .await
        .unwrap();

    println!("{:#?}", entity_history);
    // EntityHistoryResponse {
    //     history: [
    //         EntityHistoryElement {
    //             db___valid_time: 2014-11-28T12:00:09+00:00,
    //             tx___tx_id: 6,
    //             tx___tx_time: 2020-08-17T15:21:53.682+00:00,
    //             db___content_hash: "9d2c7102d6408d465f85b0b35dfb209b34daadd1",
    //             db__doc: Some(
    //                 Map(
    //                     Map(
    //                         {
    //                             ":crux.db/id": Key(
    //                                 ":jorge-3",
    //                             ),
    //                             ":first-name": Str(
    //                                 "Michael",
    //                             ),
    //                             ":last-name": Str(
    //                                 "Jorge",
    //                             ),
    //                         },
    //                     ),
    //                 ),
    //             ),
    //         },
    //         EntityHistoryElement {
    //             db___valid_time: 2014-11-29T06:00:09+00:00,
    //             tx___tx_id: 12,
    //             tx___tx_time: 2020-08-17T18:57:00.044+00:00,
    //             db___content_hash: "9d2c7102d6408d465f85b0b35dfb209b34daadd1",
    //             db__doc: Some(
    //                 Map(
    //                     Map(
    //                         {
    //                             ":crux.db/id": Key(
    //                                 ":jorge-3",
    //                             ),
    //                             ":first-name": Str(
    //                                 "Michael",
    //                             ),
    //                             ":last-name": Str(
    //                                 "Jorge",
    //                             ),
    //                         },
    //                     ),
    //                 ),
    //             ),
    //         },
    //     ],
    // }
}

ser_struct! {
    #[derive(Debug, Clone)]
    #[allow(non_snake_case)]
    pub struct Person {
        crux__db___id: CruxId,
        first_name: String,
        last_name: String
    }
}
