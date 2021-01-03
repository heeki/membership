use log::{info, error};
use std::io::{Read, Write};
use std::net::{SocketAddrV4, TcpStream, UdpSocket};
use std::str::from_utf8;

#[derive(Debug)]
pub struct GossipClient {
    src: Option<SocketAddrV4>,
    dst: SocketAddrV4
}
impl GossipClient {
    // struct methods
    pub fn new(src: Option<SocketAddrV4>, dst: SocketAddrV4) -> GossipClient {
        GossipClient { src, dst }
    }

    pub fn tsend(&self, msg: &[u8]) {
        match TcpStream::connect(self.dst) {
            Ok(mut stream) => {
                info!("status=connected server={}", self.dst.to_string());
                stream.write(msg).unwrap();
                GossipClient::handle_tresponse(stream, msg);
            },
            Err(e) => error!("status=connection_failed error={}", e)
        }
        info!("status=disconnected");
    }

    pub fn usend(&self, msg: &[u8]) {
        let socket = UdpSocket::bind(self.src.unwrap()).unwrap();
        let result = socket.send_to(msg, self.dst.to_string()).unwrap();
        info!("status=sent bytes={}", result);
    }

    // static methods
    fn handle_tresponse(mut stream: TcpStream, msg: &[u8]) {
        let mut data = [0 as u8; 64];
        match stream.read(&mut data) {
            Ok(_) => {
                let original = from_utf8(msg).unwrap();
                let response = from_utf8(&data).unwrap().replace("\u{0}", "");
                if original == response { info!("response={:?}", response); }
                else { error!("original={:?} response={:?}", original, response); }
            },
            Err(e) => error!("error={}", e)
        }
    }
}