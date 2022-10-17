use std::net::TcpListener;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    port: u16,
}

fn main() {
    let args = Cli::parse();
    if let Some(available_port) = get_available_port(args.port) {
        println!("{}", available_port);
    }
}

fn get_available_port(start_at: u16) -> Option<u16> {
    (start_at..).find(|port| port_is_available(*port))
}

fn port_is_available(port: u16) -> bool {
    match TcpListener::bind(("127.0.0.1", port)) {
        Ok(_) => true,
        Err(_) => false,
    }
}
