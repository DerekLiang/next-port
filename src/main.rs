use std::net::TcpListener;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    port: u16,
    host: Option<String>,
}

fn main() {
    let args = Cli::parse();

    if let Some(available_port) = get_available_port(
        args.port,
        &(args.host.map_or("127.0.0.1".to_owned(), |v| v)),
    ) {
        println!("{}", available_port);
    }
}

fn get_available_port(start_at: u16, host: &str) -> Option<u16> {
    (start_at..).find(|port| port_is_available(*port, host))
}

fn port_is_available(port: u16, host: &str) -> bool {
    match TcpListener::bind((host, port)) {
        Ok(_) => true,
        Err(_) => false,
    }
}
