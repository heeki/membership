mod gossip;

use gossip::{client, server};
use log::{info, error};
use std::env;

fn usage() {
    error!("usage: cargo run -- [--tserver|--tclient [message]]");
    error!("usage: cargo run -- [--userver|--uclient [message]]");
}

fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    info!("{:?}", args);
    let c = membership::Config::new(String::from("etc/config.json"));
    info!("{:?}", c);

    match args.len() {
        1 => usage(),
        2 => {
            let cmd = args[1].as_str();
            let default = "hello world";
            match cmd {
                "--tserver" => {
                    let s = server::GossipServer::new(c.get_tserver());
                    s.start_tserver();
                },
                "--userver" => {
                    let s = server::GossipServer::new(c.get_userver());
                    s.start_userver();
                },
                "--tclient" => {
                    let c = client::GossipClient::new(None, c.get_tserver());
                    c.tsend(default.as_bytes());
                },
                "--uclient" => {
                    let c = client::GossipClient::new(Some(c.get_uclient()), c.get_userver());
                    c.usend(default.as_bytes());
                }
                "--config" => {
                },
                _ => usage()
            }
        },
        3 => {
            let cmd = args[1].as_str();
            let msg = args[2].as_str();
            match cmd {
                "--tclient" => {
                    let c = client::GossipClient::new(None, c.get_tserver());
                    c.tsend(msg.as_bytes());
                },
                "--uclient" => {
                    let c = client::GossipClient::new(Some(c.get_uclient()), c.get_userver());
                    c.usend(msg.as_bytes());
                },
                _ => usage()
            }
        }
        _ => usage()
    }
}