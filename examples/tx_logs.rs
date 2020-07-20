use transistor::docker::{Crux};

fn main() {
    let body = Crux::new("localhost", "3000").client().tx_logs().unwrap();

    println!("Body = {:#?}", body);
    // Body = TxLogsResponse { tx_events: [
    // TxLogResponse { tx___tx_id: 0, 
    //                 tx___tx_time: "2020-07-09T23:38:06.465-00:00", 
    //                 tx__event___tx_events: Some("[Vector(Vector([Key(\":crux.tx/put\"), Str(\"a15f8b81a160b4eebe5c84e9e3b65c87b9b2f18e\"), Str(\"125d29eb3bed1bf51d64194601ad4ff93defe0e2\")])), ]") }, 
    // TxLogResponse { tx___tx_id: 1, 
    //                 tx___tx_time: "2020-07-09T23:39:33.815-00:00", 
    //                 tx__event___tx_events: Some("[Vector(Vector([Key(\":crux.tx/put\"), Str(\"a15f8b81a160b4eebe5c84e9e3b65c87b9b2f18e\"), Str(\"1b42e0d5137e3833423f7bb958622bee29f91eee\")])), ]") }, 
    // TxLogResponse { tx___tx_id: 2, 
    //                 tx___tx_time: "2020-07-09T23:39:45.526-00:00", 
    //                 tx__event___tx_events: Some("[Vector(Vector([Key(\":crux.tx/put\"), Str(\"a15f8b81a160b4eebe5c84e9e3b65c87b9b2f18e\"), Str(\"b422a66971103b0652e231f8d269695915b72113\")])), ]") }, 
    // TxLogResponse { tx___tx_id: 3, 
    //                 tx___tx_time: "2020-07-10T15:47:10.966-00:00", 
    //                 tx__event___tx_events: Some("[Vector(Vector([Key(\":crux.tx/put\"), Str(\"a15f8b81a160b4eebe5c84e9e3b65c87b9b2f18e\"), Str(\"b422a66971103b0652e231f8d269695915b72113\")])), ]") }, 
    // TxLogResponse { tx___tx_id: 4, 
    //                 tx___tx_time: "2020-07-10T15:47:14.528-00:00", 
    //                 tx__event___tx_events: Some("[Vector(Vector([Key(\":crux.tx/put\"), Str(\"a15f8b81a160b4eebe5c84e9e3b65c87b9b2f18e\"), Str(\"b422a66971103b0652e231f8d269695915b72113\")])), ]") }, 
    // TxLogResponse { tx___tx_id: 5, 
    //                 tx___tx_time: "2020-07-16T21:48:20.208-00:00", 
    //                 tx__event___tx_events: Some("[Vector(Vector([Key(\":crux.tx/put\"), Str(\"be21bd5ae7f3334b9b8abb185dfbeae1623088b1\"), Str(\"9d2c7102d6408d465f85b0b35dfb209b34daadd1\")])), ]") }, 
    // TxLogResponse { tx___tx_id: 6, 
    //                 tx___tx_time: "2020-07-16T21:49:56.668-00:00", 
    //                 tx__event___tx_events: Some("[Vector(Vector([Key(\":crux.tx/put\"), Str(\"be21bd5ae7f3334b9b8abb185dfbeae1623088b1\"), Str(\"9d2c7102d6408d465f85b0b35dfb209b34daadd1\")])), ]") }, 
    // TxLogResponse { tx___tx_id: 7, 
    //                 tx___tx_time: "2020-07-16T21:50:39.309-00:00", 
    //                 tx__event___tx_events: Some("[Vector(Vector([Key(\":crux.tx/put\"), Str(\"be21bd5ae7f3334b9b8abb185dfbeae1623088b1\"), Str(\"9d2c7102d6408d465f85b0b35dfb209b34daadd1\")])), Vector(Vector([Key(\":crux.tx/put\"), Str(\"31af6abc6258012408cf79543641c1b059a65b36\"), Str(\"0d68017b5a9ff805a9f488e19e0ba532d439a721\")])), ]") }, 
    // TxLogResponse { tx___tx_id: 8, 
    //                 tx___tx_time: "2020-07-16T21:53:14.628-00:00", 
    //                 tx__event___tx_events: Some("[Vector(Vector([Key(\":crux.tx/put\"), Str(\"be21bd5ae7f3334b9b8abb185dfbeae1623088b1\"), Str(\"9d2c7102d6408d465f85b0b35dfb209b34daadd1\")])), Vector(Vector([Key(\":crux.tx/put\"), Str(\"31af6abc6258012408cf79543641c1b059a65b36\"), Str(\"0d68017b5a9ff805a9f488e19e0ba532d439a721\")])), ]") }, 
    // TxLogResponse { tx___tx_id: 9, 
    //                 tx___tx_time: "2020-07-16T22:00:45.898-00:00", 
    //                 tx__event___tx_events: Some("[Vector(Vector([Key(\":crux.tx/put\"), Str(\"be21bd5ae7f3334b9b8abb185dfbeae1623088b1\"), Str(\"9d2c7102d6408d465f85b0b35dfb209b34daadd1\")])), Vector(Vector([Key(\":crux.tx/put\"), Str(\"31af6abc6258012408cf79543641c1b059a65b36\"), Str(\"0d68017b5a9ff805a9f488e19e0ba532d439a721\")])), ]") }, 
    // TxLogResponse { tx___tx_id: 10, 
    //                 tx___tx_time: "2020-07-16T22:05:03.424-00:00", 
    //                 tx__event___tx_events: Some("[Vector(Vector([Key(\":crux.tx/delete\"), Str(\"facab0078b9ec975787eed8b68c6a79047dca68a\")])), ]") }, 
    // TxLogResponse { tx___tx_id: 11, 
    //                 tx___tx_time: "2020-07-16T22:11:16.222-00:00", 
    //                 tx__event___tx_events: Some("[Vector(Vector([Key(\":crux.tx/put\"), Str(\"be21bd5ae7f3334b9b8abb185dfbeae1623088b1\"), Str(\"9d2c7102d6408d465f85b0b35dfb209b34daadd1\")])), Vector(Vector([Key(\":crux.tx/put\"), Str(\"31af6abc6258012408cf79543641c1b059a65b36\"), Str(\"0d68017b5a9ff805a9f488e19e0ba532d439a721\")])), ]")}]}
}
