extern crate tcp_test;
use tcp_test::act::serv;
fn main() {
    let ip = "0.0.0.0:12345";
    serv::Server::start(ip);
}
