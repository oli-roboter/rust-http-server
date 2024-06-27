#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use server::Server;
use std::env;
use website_handler::WebsiteHandler;

mod server;
mod http;
mod website_handler;

fn main() {
    /*
    The below code will give the path to "/Users/oliverwagner/Code/udemy/rust/rust-http-server";
    let default_path = env!("CARGO_MANIFEST_DIR");
    This can be checked by typing 'cargo expand | code -' in the terminal -> will generate the compiled version
    of the code. the path can be found in the main() function section
     */
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    println!("public_path: {}", public_path);
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(WebsiteHandler::new(public_path));
}
