mod domain;
mod errors;
mod listener;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Error, Method, Request, Response, Server, StatusCode};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::task;


use crate::domain::sensor::SensorRepository;
use crate::listener::RfReceiver;

#[tokio::main]
async fn main() {
    let state = Arc::new(SensorRepository::new());

    let listener = RfReceiver::new(vec![]);

    tokio::spawn(async {
        RfReceiver::start_listening(& move |s| println!("{}", s))
    });
    let addr = SocketAddr::from(([127, 0, 0, 1], 7000));

    let make_service = make_service_fn(move |_| {
        let state = state.clone();

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
