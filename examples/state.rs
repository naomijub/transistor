use transistor::client::Crux;

fn main() {
    let body = Crux::new("localhost", "3000").docker_client().state().unwrap();

    println!("body = {:?}", body);
    // body = StateResponse { 
    //          index___index_version: 5, 
    //          doc_log___consumer_state: None, 
    //          tx_log___consumer_state: None, 
    //          kv___kv_store: "crux.kv.rocksdb.RocksKv", 
    //          kv___estimate_num_keys: 56, 
    //          kv___size: 2271042 
    //        }
}
