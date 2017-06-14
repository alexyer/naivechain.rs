extern crate futures;
extern crate websocket;
extern crate hyper;
extern crate naivechain;

use futures::future::FutureResult;

use hyper::server::{Http, Service, Request, Response};
use hyper::header::ContentLength;
use hyper::{Get, Post, StatusCode};

struct Echo;

impl Service for Echo {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = FutureResult<Response, hyper::Error>;

    fn call(&self, req: Request) -> Self::Future {
        futures::future::ok(match (req.method(), req.path()) {
            _ => {
                Response::new()
                    .with_status(StatusCode::NotFound)
            }
        })
    }
}

fn main() {
    let addr = "0.0.0.0:8080".parse().unwrap();

    let server = Http::new().bind(&addr, || Ok(Echo)).unwrap();
    print!("Listening on http://{} with 1 thread", server.local_addr().unwrap());
    server.run().unwrap();
}
