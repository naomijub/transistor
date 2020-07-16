use transistor::docker::{Crux,Action};
use transistor::types::{CruxId};
use transistor::edn_rs::{ser_struct, Serialize};

fn main() {
    let person1 = Person {
        crux__db___id: CruxId::new("jorge-3"), 
        first_name: "Michael".to_string(), 
        last_name: "Jorge".to_string()
    };

    let person2 = Person {
        crux__db___id: CruxId::new("manuel-1"), 
        first_name: "Diego".to_string(), 
        last_name: "Manuel".to_string()
    };
    println!("{:?}", person1.clone().serialize());
    //"{ :crux.db/id :jorge-3, :first-name \"Michael\", :last-name \"Jorge\", }"
    println!("{:?}", person2.clone().serialize());
    //"{ :crux.db/id :manuel-1, :first-name \"Diego\", :last-name \"Manuel\", }"

    let action1 = Action::Put(person1.serialize());
    let action2 = Action::Put(person2.serialize());

    let body = Crux::new("localhost", "3000").client().tx_log(vec![action1, action2]).unwrap();
    // Request body for vec![action1, action2]
    // "[[:crux.tx/put { :crux.db/id :jorge-3, :first-name \"Michael\", :last-name \"Jorge\", }], 
    //   [:crux.tx/put { :crux.db/id :manuel-1, :first-name \"Diego\", :last-name \"Manuel\", }]]"

    println!("\n Body = {:?}", body);
    //  Body = "{:crux.tx/tx-id 7, :crux.tx/tx-time #inst \"2020-07-16T21:50:39.309-00:00\"}"
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