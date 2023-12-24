use clap::Parser;

mod client_handler;
mod error;
mod message;
mod server;
mod topic;

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

    match Server::new(&server_address) {
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
