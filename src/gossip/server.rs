use log::{info, error};
use std::io::{Read, Write};
use std::net::{SocketAddrV4, TcpListener, TcpStream, Shutdown};
use std::str::from_utf8;
use std::thread;

#[derive(Debug)]
pub struct GossipServer {
    socket: SocketAddrV4
}
impl GossipServer {
    // struct methods
    pub fn new(connection: String) -> GossipServer {
        GossipServer { socket: membership::get_socket_address(connection) }
    }

    pub fn start_server(&self) {
        let listener = TcpListener::bind(self.socket).unwrap();
        info!("listening on {:?}", self.socket);
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    info!("incoming peer {}", stream.peer_addr().unwrap());
                    thread::spawn(move || {
                        GossipServer::handle_request(stream)
                    });
                },
                Err(e) => error!("msg={}", e)
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
                    info!("request={:?}", from_utf8(&data[0..size]).unwrap());
                    stream.write(&data[0..size]).unwrap();
                }
                true
            },
            Err(_) => {
                error!("peer={}", stream.peer_addr().unwrap());
                stream.shutdown(Shutdown::Both).unwrap();
                false
            }
        } {}
    }
}