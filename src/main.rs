use clap::Parser;
use std::net::TcpListener;
mod server;

#[derive(Parser)]
struct Cli {
    #[arg(short = 'H', long)]
    host: String,

    #[arg(short, long)]
    port: u16,
}
fn main() {
    let args = Cli::parse();
    let host = args.host;
    let port = args.port;

    let listener = match TcpListener::bind(format!("{}:{}", host, port.to_string())) {
        Ok(l) => {
            println!("Server started successfully for host: {}:{}", host, port);
            l
        }
        Err(err) => {
            println!("Error starting server: {}", err);
            return;
        }
    };

    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                server::parse::handle_connections(s);
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
}
