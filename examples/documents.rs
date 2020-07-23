use transistor::docker::{Action};
use transistor::client::Crux;
use transistor::types::{CruxId};
use transistor::edn_rs::{ser_struct, Serialize};

fn main() {
    let person1 = Person {
        crux__db___id: CruxId::new("hello-entity"), 
        first_name: "Hello".to_string(), 
        last_name: "World".to_string()
    };

    let person2 = Person {
        crux__db___id: CruxId::new("hello-documents"), 
        first_name: "Hello".to_string(), 
        last_name: "Documents".to_string()
    };

    let client = Crux::new("localhost", "3000").docker_client();
    let put_person1 = Action::Put(person1.clone().serialize());
    let put_person2 = Action::Put(person2.clone().serialize());

    let _ = client.tx_log(vec![put_person1]).unwrap();
    let _ = client.tx_log(vec![put_person2]).unwrap();



    let tx_body1 = client.entity_tx(person1.crux__db___id.serialize()).unwrap();
    let tx_body2 = client.entity_tx(person2.crux__db___id.serialize()).unwrap();
    println!("\n Tx Body = {:#?}", tx_body1.clone());
    // Tx Body = EntityTxResponse {
    //     db___id: "d72ccae848ce3a371bd313865cedc3d20b1478ca",
    //     db___content_hash: "1828ebf4466f98ea3f5252a58734208cd0414376",
    //     db___valid_time: "2020-07-21T23:33:44.339-00:00",
    //     tx___tx_id: 44,
    //     tx___tx_time: "2020-07-21T23:33:44.339-00:00",
    // }

    let contesnt_hashes = vec![tx_body1.db___content_hash, tx_body2.db___content_hash];

    let documents = client.documents(contesnt_hashes).unwrap();
    println!("\n Documents Body = {:#?}", documents);
    // Documents Body = {
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