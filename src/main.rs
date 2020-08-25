mod domain;
mod errors;
mod listener;
mod state_actor;

extern crate lazy_static;
extern crate serde;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Error, Method, Response, Server, StatusCode};
use std::net::SocketAddr;

use crate::domain::sensor::SensorRepository;
use crate::domain::errors::DomainError;
use crate::domain::command_event::Command;

#[tokio::main]
async fn main() {
    let message_sender = state_actor::init_actor();

    listener::start_listening(message_sender.clone());
    let addr = SocketAddr::from(([127, 0, 0, 1], 7000));

    let make_service = make_service_fn(move |_| {
        let sender_read = message_sender.clone();

        async move {
            let sender_read = sender_read.clone();
            Ok::<_, Error>(service_fn(move |req| {
                let sender_read = sender_read.clone();
                async move {
                    let sender_read = sender_read.clone();
                    match (req.method(), req.uri().path()) {
                (&Method::GET, "/") => Ok(Response::new(Body::from("Try POSTing data to /echo such as: `curl localhost:3000/echo -XPOST -d 'hello world'`",))),
                (&Method::GET, "/alive") => Ok::<_,Error>(Response::new(Body::from("yes"))),
                (&Method::GET, "/all_sensors") => {
                    let (sender, receiver) = std::sync::mpsc::channel::<Box<String>>();
                    let mess = Command::GetData(Box::new(move |state:&SensorRepository| {
                        let json = state.get_all_state()?;
                        match sender.send(Box::new(json)) {
                            Ok(_) => Ok(()),
                            Err(e) => Err(DomainError::DataExtractionError{value: e.to_string()})
                        }?;
                        Ok(vec![])

                    }));
                    sender_read.send(mess);
                    match receiver.recv_timeout(std::time::Duration::from_secs(1)) {
                        Ok(data) => {
                            Ok::<_,Error>(Response::builder().header("content-type", "application/json").header("charset", "UTF-8").body(Body::from(*data)).unwrap())
                        },
                        Err(e) => {
                            println!("error in hyper: {}", e);
                            Ok::<_,Error>(Response::new(Body::from(e.to_string())))
                        }
                    }
                },
                _ => {
                        let mut not_found = Response::default();
                        *not_found.status_mut() = StatusCode::NOT_FOUND;
                        Ok::<_,Error>(not_found)
        }
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
