mod domain;
mod errors;
mod listener;
mod state_actor;

extern crate lazy_static;
extern crate serde;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Error, Method, Request, Response, Server, StatusCode};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::task;

use crate::domain::sensor::SensorRepository;
use state_actor::{Message, MessageSender};
use crate::errors::RfError;

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
                    let mess = Message::GetData(Box::new(move |state:&SensorRepository| {
                        let json = state.get_all_state()?;
                        match sender.send(Box::new(json)) {
                            Ok(_) => Ok(()),
                            Err(e) => Err(RfError::ComError{value: e.to_string()})
                        }?;
                        Ok(())

                    }));
                    sender_read.Send(mess);
                    match receiver.recv() {
                        Ok(data) => Ok::<_,Error>(Response::new(Body::from(*data))),
                        Err(e) => Ok::<_,Error>(Response::new(Body::from(e.to_string())))
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
