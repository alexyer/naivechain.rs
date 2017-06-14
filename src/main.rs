extern crate futures;
extern crate websocket;
extern crate hyper;
extern crate serde_json;

extern crate naivechain;

use futures::future::FutureResult;

use hyper::server::{Http, Service, Request, Response};
use hyper::header::{ContentLength, ContentType};
use hyper::{Get, Post, StatusCode};

use naivechain::blockchain::Blockchain;

struct Echo;

impl Service for Echo {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = FutureResult<Response, hyper::Error>;

    fn call(&self, req: Request) -> Self::Future {
        let chain = Blockchain::new();
        futures::future::ok(match (req.method(), req.path()) {
            (&Get, "/blocks") => {
                let res = serde_json::to_string(&chain).unwrap();
                Response::new()
                    .with_header(ContentLength(res.len() as u64))
                    .with_header(ContentType::json())
                    .with_body(res)
            },
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
