use std::{ convert::{ TryFrom, TryInto }, fmt::Debug, io::{ Read, Write }, net::TcpListener };

use crate::http::{ ParseError, Request, Response, StatusCode };

pub trait Handler {
    fn hande_request(&mut self, request: &Request) -> Response;

    // default implementation of the function
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse a request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        // Server { addr }
        Self { addr }
    }

    pub fn run(self, mut handler: impl Handler) {
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
                            let response = match Request::try_from(&buffer as &[u8]) {
                                // or Request::try_from(&buffer[..])
                                // or let result: &Result<Request, _> = &buffer[..].try_into();
                                Ok(request) => {
                                    dbg!(&request);
                                    handler.hande_request(&request)
                                    /*
                                    after creating and implementing the handler trait, we use the above code, instead of the original: 
                                    Response::new(
                                        StatusCode::Ok,
                                        Some("<h1>Fooooquin works</h1>".to_string())
                                    )
                                    */
                                }
                                Err(e) => {
                                    handler.handle_bad_request(&e)
                                    /*
                                    After creating and implementing the handler trait, the above code is going to use the default implementation of the Handler
                                    trait to deal with the bad request, instead of using the below code.
                                    println!("Failed to parse a request: {}", e);
                                    Response::new(StatusCode::BadRequest, None)
                                    */
                                }
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
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
