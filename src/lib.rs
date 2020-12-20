use std::io::{Read, Write};
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream, Shutdown};
use std::str::from_utf8;
use std::thread;

#[derive(Debug)]
pub struct GossipServer {
    socket: SocketAddrV4
}
impl GossipServer {
    // struct methods
    pub fn new(connection: String) -> GossipServer {
        let elements: Vec<_> = connection.split(":").collect();
        if elements.len() == 2 {
            GossipServer { socket: get_socket_address(connection) }
        } else {
            GossipServer { socket: SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080) }
        }
    }

    pub fn start_server(&self) {
        let listener = TcpListener::bind(self.socket).unwrap();
        println!("listening on {:?}", self.socket);
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("[ok] incoming_peer={}", stream.peer_addr().unwrap());
                    thread::spawn(move || {
                        GossipServer::handle_request(stream)
                    });
                },
                Err(e) => println!("[error] msg={}", e)
            }
        }
        drop(listener);
    }

    // static methods
    fn handle_request(mut stream: TcpStream) {
        let mut data = [0 as u8; 50];
        while match stream.read(&mut data) {
            Ok(size) => {
                if size > 0 {
                    println!("[ok] request={:?}", from_utf8(&data[0..size]).unwrap());
                    stream.write(&data[0..size]).unwrap();
                }
                true
            },
            Err(_) => {
                println!("[error] peer={}", stream.peer_addr().unwrap());
                stream.shutdown(Shutdown::Both).unwrap();
                false
            }
        } {}
    }
}

#[derive(Debug)]
pub struct GossipClient {
    socket: SocketAddrV4
}
impl GossipClient {
    // struct methods
    pub fn new(connection: String) -> GossipClient {
        let elements: Vec<_> = connection.split(":").collect();
        if elements.len() == 2 {
            GossipClient { socket: get_socket_address(connection) }
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

fn get_socket_address(connection: String) -> SocketAddrV4 {
    let elements: Vec<_> = connection.split(":").collect();
    if elements.len() == 2 {
        let ipv4 = elements[0].parse::<Ipv4Addr>().unwrap();
        let port = elements[1].parse::<u16>().unwrap();
        SocketAddrV4::new(ipv4, port)
    } else {
        SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_socket_address() {
        assert_eq!(get_socket_address(String::from("10.10.0.100:443")), SocketAddrV4::new(Ipv4Addr::new(10, 10, 0, 100), 443));
        assert_ne!(get_socket_address(String::from("10.10.0.100;443")), SocketAddrV4::new(Ipv4Addr::new(10, 10, 0, 100), 443));
        assert_ne!(get_socket_address(String::from("10.10.0.100")), SocketAddrV4::new(Ipv4Addr::new(10, 10, 0, 100), 443));
    }
}