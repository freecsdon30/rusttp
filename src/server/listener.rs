use super::parse;
use std::net::TcpListener;

#[allow(dead_code)]
pub fn listen(host: &str, port: &u16) {
    let listener = match TcpListener::bind(format!("{host}:{port}")) {
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
                parse::handle_connections(s);
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
}
