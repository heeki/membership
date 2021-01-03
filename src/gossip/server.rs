use log::{info, error};
use std::io::{Read, Write};
use std::net::{Shutdown, SocketAddrV4, TcpListener, TcpStream, UdpSocket};
use std::str::from_utf8;
use std::thread;

#[derive(Debug)]
pub struct GossipServer {
    socket: SocketAddrV4
}
impl GossipServer {
    // struct methods
    pub fn new(socket: SocketAddrV4) -> GossipServer {
        GossipServer { socket }
    }

    pub fn start_tserver(&self) {
        let listener = TcpListener::bind(self.socket).expect("failed to bind");
        info!("status=listening binding={:?}", self.socket);
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    thread::spawn(move || {
                        GossipServer::handle_trequest(stream)
                    });
                },
                Err(e) => error!("error={}", e)
            }
        }
        drop(listener);
    }

    pub fn start_userver(&self) {
        let socket = UdpSocket::bind(self.socket).expect("failed to bind");
        info!("status=listening binding={:?}", self.socket);
        loop {
            GossipServer::handle_urequest(&socket);
        }
    }

    // static methods
    fn handle_trequest(mut stream: TcpStream) {
        let mut buffer = [0 as u8; 64];
        while match stream.read(&mut buffer) {
            Ok(recv_bytes) => {
                if recv_bytes > 0 {
                    info!("recv_src={} recv_bytes={} request={:?}", stream.peer_addr().unwrap(), recv_bytes, from_utf8(&buffer[0..recv_bytes]).unwrap());
                    stream.write(&buffer[0..recv_bytes]).unwrap();
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

    fn handle_urequest(socket: &UdpSocket) -> usize {
        let mut buffer = [0 as u8; 64];
        let (recv_bytes, recv_src) = socket.recv_from(&mut buffer).unwrap();
        info!("recv_src={} recv_bytes={} request={:?}", recv_src, recv_bytes, from_utf8(&buffer[0..recv_bytes]).unwrap());
        recv_bytes
    }
}