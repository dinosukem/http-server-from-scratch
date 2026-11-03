mod request;
mod response;

use std::io::Write;
#[allow(unused_imports)]
use std::net::TcpListener;
pub use {
    request::*,
    response::*,
};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible
    // when running tests.
    println!("Logs from your program will appear here!");

    // TODO: Uncomment the code below to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");
                if let Err(e) = handle_connection(stream) {
                    println!("error: {}", e);
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: std::net::TcpStream) -> std::io::Result<()> {
    let request = Request::from_stream(stream.try_clone()?);

    println!("received request: {:?}", request);

    let protocol = request.protocol.clone();
    let response = match request.path.segments().as_slice() {
        [] => Response::new(&protocol, &Status::OK, ""),

        ["echo", str] => {
            let mut response = Response::new(&protocol, &Status::OK, str);
            response.add_header("Content-Type", "text/plain");
            response.add_header("Content-Length", &response.content.len().to_string());

            response
        }

        ["user-agent"] => {
            let mut response =
                Response::new(&protocol, &Status::OK, &request.headers.get("User-Agent"));
            response.add_header("Content-Type", "text/plain");
            response.add_header("Content-Length", &response.content.len().to_string());

            response
        }

        _ => Response::new(&protocol, &Status::NotFound, ""),
    };

    stream.write_all(response.raw().as_bytes())
}
