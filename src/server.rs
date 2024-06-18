use std::{ io::Read, net::TcpListener, convert::{ TryFrom, TryInto } };

use crate::http::Request;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        // Server { addr }
        Self { addr }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();

        // same as while true...
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            // Next line we are converting the buffer (an array) to a request, but to do that we need to convert the array into a byte slice
                            match Request::try_from(&buffer as &[u8]) {
                                // or Request::try_from(&buffer[..])
                                // or let result: &Result<Request, _> = &buffer[..].try_into();
                                Ok(request) => {}
                                Err(e) => println!("Failed to parse a request: {}", e),
                            };
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                }
                Err(e) => println!("Failed to establish connection: {}", e),
            }
        }
    }
}

/*
When pasting code in a file, this is automatically transformed into a module.
The equivalent of doing this inside the same file as the main code would be to wrap the 
above code in the the following:
mod server {
    struct Server {
        bla
    }

    etc...
}
*/
