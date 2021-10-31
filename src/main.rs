use std::{env, net::SocketAddr};

use log::{info, error};
use tiny_http::{Server, Response};


fn main() -> std::io::Result<()> {
    let port = match env::var("PORT") {
        Ok(p) => p.parse::<u16>().unwrap(),
        Err(..) => 8000,
    };

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let server = Server::http(addr).unwrap();
    info!("Server listening on port {}", port);

    for req in server.incoming_requests() {
        let response = {
            match req.url().trim_start_matches("/").parse::<u64>() {
                Ok(i) => { 
                    match i % 15 {
                        0 => Response::from_string("FizzBuzz").with_status_code(200),
                        3 | 6 | 9 | 12 => Response::from_string("Fizz").with_status_code(200),
                        5 | 10 => Response::from_string("Buzz").with_status_code(200),
                        _ => Response::from_string(i.to_string()).with_status_code(200),
                    }
                },
                Err(e) => Response::from_string(e.to_string()).with_status_code(400),
            }
        };
        match req.respond(response) {
            Err(e) => error!("{}", e),
            Ok(_) => ()
        };
    }

    Ok(())
}
