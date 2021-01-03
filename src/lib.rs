use serde_json;
use serde::{Deserialize, Serialize};
use std::fs;
use std::net::{Ipv4Addr, SocketAddrV4};

fn get_socket_address(connection: &String) -> SocketAddrV4 {
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
    tserver: String,
    userver: String,
    uclient: String
}
impl Config {
    pub fn new(file: String) -> Config {
        let data = fs::read_to_string(file).expect("[error] unable to read file");
        let config: Config = serde_json::from_str(data.as_str()).expect("[error] unable to parse config");
        config
    }

    pub fn get_tserver(&self) -> SocketAddrV4 {
        get_socket_address(&self.tserver)
    }

    pub fn get_userver(&self) -> SocketAddrV4 {
        get_socket_address(&self.userver)
    }

    pub fn get_uclient(&self) -> SocketAddrV4 {
        get_socket_address(&self.uclient)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_socket_address() {
        let test1 = String::from("10.10.0.100:443");
        let test2 = String::from("10.10.0.100;443");
        let test3 = String::from("10.10.0.100");
        assert_eq!(get_socket_address(&test1), SocketAddrV4::new(Ipv4Addr::new(10, 10, 0, 100), 443));
        assert_ne!(get_socket_address(&test2), SocketAddrV4::new(Ipv4Addr::new(10, 10, 0, 100), 443));
        assert_ne!(get_socket_address(&test3), SocketAddrV4::new(Ipv4Addr::new(10, 10, 0, 100), 443));
    }

    #[test]
    fn test_config() {
        let json = "{ \"tserver\": \"0.0.0.0:8080\", \"userver\": \"0.0.0.0:9000\", \"uclient\": \"0.0.0.0:9001\" }";
        let config: Config = serde_json::from_str(json).unwrap();
        assert_eq!(config.get_tserver(), SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 8080));
    }
}