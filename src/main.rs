use clap::Parser;
pub mod enums;
pub mod request;
pub mod response;
pub mod server;

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

    server::listener::listen(&host, &port);
}
