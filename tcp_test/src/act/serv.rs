use tokio_proto::TcpServer;
use super::setting::service::DBserver;
use super::setting::proto::LineProto;

pub struct Server;
impl Server {
    pub fn start(addr: &str) {
        let server = TcpServer::new(LineProto, addr.parse().unwrap());

        server.serve(|| Ok(DBserver));
    }
}
