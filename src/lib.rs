use serde_json;
use serde::{Deserialize, Serialize};
use std::fs;
use std::net::{Ipv4Addr, SocketAddrV4};

pub fn get_socket_address(connection: String) -> SocketAddrV4 {
    let elements: Vec<_> = connection.split(":").collect();
    if elements.len() == 2 {
        let ipv4 = elements[0].parse::<Ipv4Addr>().unwrap();
        let port = elements[1].parse::<u16>().unwrap();
        SocketAddrV4::new(ipv4, port)
    } else {
        SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    socket: String
}
impl Config {
    pub fn new(file: String) -> Config {
        let data = fs::read_to_string(file).expect("[error] unable to read file");
        let config: Config = serde_json::from_str(data.as_str()).expect("[error] unable to parse config");
        config
    }

    pub fn get_socket(&self) -> &String {
        &self.socket
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

    #[test]
    fn test_config() {
        let json = "{ \"socket\": \"0.0.0.0:8080\" }";
        let config: Config = serde_json::from_str(json).unwrap();
        assert_eq!(config.get_socket(), "0.0.0.0:8080");
    }
}