use transistor::http::Crux;
use transistor::edn_rs::serialize::Serialize;

fn main() {
    let body = Crux::new("localhost", "3000").client().state().unwrap();

    println!("StateResponse = {:?}", body);
    println!("edn body = {:?}", body.serialize());
}
