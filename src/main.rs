mod domain;
mod errors;
mod listener;

extern crate lazy_static;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Error, Method, Request, Response, Server, StatusCode};
use std::net::SocketAddr;
use std::sync::{Arc,Mutex};
use tokio::task;
use lazy_static::*;

use crate::domain::sensor::SensorRepository;

lazy_static! {
    pub static ref state: Arc<SensorRepository> = Arc::new(SensorRepository::new());
}

fn transfert_to_domain(data: String) {
        println!("{}", data);
        domain::listen(&data, state.clone());
}

#[tokio::main]
async fn main() {
    
    listener::start_listening(&transfert_to_domain);
    let addr = SocketAddr::from(([127, 0, 0, 1], 7000));

    let make_service = make_service_fn(move |_| {

        async move {
            Ok::<_, Error>(service_fn(move |req| async move {
                match (req.method(), req.uri().path()) {
                (&Method::GET, "/") => Ok(Response::new(Body::from("Try POSTing data to /echo such as: `curl localhost:3000/echo -XPOST -d 'hello world'`",))),
                (&Method::GET, "/alive") => Ok::<_,Error>(Response::new(Body::from("yes"))),
                _ => {
                        let mut not_found = Response::default();
                        *not_found.status_mut() = StatusCode::NOT_FOUND;
                        Ok::<_,Error>(not_found)
        }
    }
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_service);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }

    println!("end");
}
