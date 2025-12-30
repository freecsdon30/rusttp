use clap::Parser;
use std::net::TcpListener;

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

    let listener = TcpListener::bind(format!("{}:{}", host, port.to_string())).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("connection established..!!: {:?}", stream);
    }
}
