use tokio_proto::TcpClient;
use tokio_core::reactor::Core;
use super::setting::proto::LineProto;
use super::setting::service::{Message, State};
use std::net::SocketAddr;
use futures::Future;
use serde_json;
pub struct Client;
impl Client {
    pub fn connect(addr: &str) {
        let clint = TcpClient::new(LineProto);
        let mut core = Core::new().unwrap();
        let handle = core.handle();
        let address = addr.parse::<SocketAddr>().unwrap();
        let meg = Message::new(0, State::ReqSearch { content: String::from("hello") });
        println!("{}", serde_json::to_string(&meg).unwrap());
        let client_future = clint.connect(&address, &handle).and_then(|_| Ok(()));
        core.run(client_future).unwrap();
    }
}
