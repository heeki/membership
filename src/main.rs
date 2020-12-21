mod gossip;

use gossip::{client, server};
use std::env;

fn usage() {
    println!("usage: cargo run -- [--server|--client [message]]")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let c = membership::Config::new(String::from("etc/config.json"));
    println!("{:?}", c);

    match args.len() {
        1 => usage(),
        2 => {
            let cmd = args[1].as_str();
            match cmd {
                "--server" => {
                    let s = server::GossipServer::new(String::from(c.get_socket()));
                    s.start_server();
                },
                "--client" => {
                    let c = client::GossipClient::new(String::from(c.get_socket()));
                    c.send("hello world".as_bytes());
                },
                "--config" => {
                },
                _ => usage()
            }
        },
        3 => {
            let cmd = args[1].as_str();
            let msg = args[2].as_str();
            match cmd {
                "--client" => {
                    let c = client::GossipClient::new(String::from(c.get_socket()));
                    c.send(msg.as_bytes());
                },
                _ => usage()
            }
        }
        _ => usage()
    }
}