extern crate futures;
extern crate websocket;
extern crate hyper;

extern crate serde_json;
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate lazy_static;

extern crate naivechain;

use futures::Future;
use futures::Stream;

use hyper::server::{Http, Service, Request, Response};
use hyper::header::{ContentLength, ContentType};
use hyper::{Get, Post, StatusCode};

use naivechain::blockchain::{Blockchain};

lazy_static! {
    static ref CHAIN: Blockchain = Blockchain::new();
}

#[derive(Debug, Deserialize)]
struct MineBlockRequest {
    data: String
}

struct NaivechainWeb;

impl Service for NaivechainWeb {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<futures::Future<Item = Response, Error = Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        match (req.method(), req.path()) {
            (&Get, "/blocks") => {
                let chain_str = serde_json::to_string(&*CHAIN).unwrap();
                let res = NaivechainWeb::create_response(&chain_str, StatusCode::Ok);
                futures::future::ok::<_, Self::Error>(res).boxed()
            },
            (&Post, "/mineBlock") => {
                req.body().fold(Vec::new(), move |mut acc, chunk| {
                    acc.extend_from_slice(&*chunk);
                    futures::future::ok::<_, Self::Error>(acc)
                }).map(move |b| {
                    let r = String::from_utf8(b).unwrap();
                    match serde_json::from_str::<MineBlockRequest>(&r) {
                        Ok(request) => {
                            let new_block = CHAIN.generate_new_block(request.data);
                            CHAIN.add_block(&new_block).unwrap();

                            let res = serde_json::to_string(&new_block).unwrap();
                            NaivechainWeb::create_response(&res, StatusCode::Created)
                        },
                        Err(err) => NaivechainWeb::create_response(&err.to_string(), StatusCode::BadRequest)
                    }
                }).boxed()
            }
            _ => {
                panic!("Not Found")
            }
        }
    }

}

impl NaivechainWeb {
    fn create_response(body: &String, status_code: StatusCode) -> Response {
        Response::new()
            .with_header(ContentLength(body.len() as u64))
            .with_header(ContentType::json())
            .with_status(status_code)
            .with_body(body.to_string())
    }
}

fn main() {
    let addr = "0.0.0.0:8080".parse().unwrap();

    let server = Http::new().bind(&addr, || Ok(NaivechainWeb)).unwrap();
    println!("Listening on http://{} with 1 thread", server.local_addr().unwrap());
    server.run().unwrap();
}
