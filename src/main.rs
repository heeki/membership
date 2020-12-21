mod gossip;

use gossip::{client, server};
use std::env;

fn usage() {
    println!("usage: cargo run -- [--server|--client [message]]")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    match args.len() {
        1 => usage(),
        2 => {
            let cmd = args[1].as_str();
            match cmd {
                "--server" => {
                    let s = server::GossipServer::new(String::from("0.0.0.0:8080"));
                    s.start_server();
                },
                "--client" => {
                    let c = client::GossipClient::new(String::from("0.0.0.0:8080"));
                    c.send("hello world".as_bytes());
                },
                _ => usage()
            }
        },
        3 => {
            let cmd = args[1].as_str();
            let msg = args[2].as_str();
            match cmd {
                "--client" => {
                    let c = client::GossipClient::new(String::from("0.0.0.0:8080"));
                    c.send(msg.as_bytes());
                },
                _ => usage()
            }
        }
        _ => usage()
    }
}