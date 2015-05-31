extern crate tiny_http;

use std::env;

use tiny_http::{Response, ServerBuilder};


fn main() {
    let port = match env::var("PORT") {
        Ok(p) => p.parse::<u16>().unwrap(),
        Err(..) => 8000,
    };

    let server = ServerBuilder::new().with_port(port).build().unwrap();

    for req in server.incoming_requests() {
        let response = Response::from_string("Hello world!");
        req.respond(response);
    }
}
