use clap::Parser;
use ctrlc;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::str;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

mod server;
mod client_handler;
mod topic;
mod message;
mod error;

use crate::server::Server;

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    address: String,
    #[arg(long)]
    port: String,
}

fn main() {
    let args = Cli::parse();

    let server_address = format!("{}:{}", args.address, args.port);

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    match Server::new(&server_address, running) {
        Ok(server) => {
            println!("Start kafka server on address {}", server_address);
            if let Err(e) = server.run() {
                eprintln!("Server error: {}", e);
            }
        }
        Err(e) => {
            if e.kind() == std::io::ErrorKind::AddrInUse {
                println!(
                    "Not able to start server on {} -- port is busy",
                    server_address
                );
            } else {
                println!("Not able to start server on {} -- {}", server_address, e);
            }
        }
    }
}
