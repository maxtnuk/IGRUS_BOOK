extern crate tcp_test;
use tcp_test::act::client;
fn main() {
    let ip = "127.0.0.1:12345";
    client::Client::connect(ip);
}
