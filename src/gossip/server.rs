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
            GossipServer { socket: membership::get_socket_address(connection) }
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