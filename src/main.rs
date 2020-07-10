mod http;

use http::Crux;

fn main() {
    let body = Crux::new("localhost", "3000").client().state().unwrap();

    println!("body = {:?}", body);
}