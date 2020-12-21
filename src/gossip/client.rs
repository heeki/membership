use std::io::{Read, Write};
use std::net::{Ipv4Addr, SocketAddrV4, TcpStream};
use std::str::from_utf8;

#[derive(Debug)]
pub struct GossipClient {
    socket: SocketAddrV4
}
impl GossipClient {
    // struct methods
    pub fn new(connection: String) -> GossipClient {
        let elements: Vec<_> = connection.split(":").collect();
        if elements.len() == 2 {
            GossipClient { socket: membership::get_socket_address(connection) }
        } else {
            GossipClient { socket: SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080) }
        }
    }

    pub fn send(&self, msg: &[u8]) {
        match TcpStream::connect(self.socket) {
            Ok(mut stream) => {
                println!("[ok] connected_to_server={}", self.socket.to_string());
                stream.write(msg).unwrap();
                GossipClient::handle_response(stream, msg);
            },
            Err(e) => println!("[error] failed_to_connect={}", e)
        }
        println!("[ok] connection_terminated");
    }

    // static methods
    fn handle_response(mut stream: TcpStream, msg: &[u8]) {
        let mut data = [0 as u8; 50];
        match stream.read(&mut data) {
            Ok(_) => {
                let original = from_utf8(msg).unwrap();
                let response = from_utf8(&data).unwrap().replace("\u{0}", "");
                if original == response { println!("[ok] response={:?}", response); }
                else { println!("[error] original={:?} response={:?}", original, response); }
            },
            Err(e) => println!("[error] stream_error={}", e)
        }
    }
}