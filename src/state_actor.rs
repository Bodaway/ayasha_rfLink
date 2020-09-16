use crate::domain::command_event::Command;
use crate::rabbit_sender::RabbitSender;
use std::sync::mpsc::channel;

use crate::domain::{dispatch, apply, send_external_message};
use crate::domain::sensor::SensorRepository;


#[derive(Clone)]
pub struct MessageSender {
    inner: std::sync::mpsc::Sender<Command>,
}

impl MessageSender {
    pub fn send(&self, mess: Command) {
        self.inner.send(mess).expect("comm error with state actor");
    }
}

pub fn init_actor() -> MessageSender {
    let (sender, receiver) = channel::<Command>();
    std::thread::spawn(move || {
    let args: Vec<String> = std::env::args().collect();
    let rabbit_user = &args[1];
    let rabbit_address = &args[2];
    let uri = format!("amqp://{}@{}/%2f", rabbit_user,rabbit_address);
    let ex_message_sender = RabbitSender::new( uri,  "Ayasha".to_string());
        let mut repo = SensorRepository::new();
        loop {
            match receiver.recv() {
                Ok(command) => {
                    dispatch(command, &repo)
                    .and_then(|evs| send_external_message(evs, &ex_message_sender) )
                    .map_or_else(
                        |e| println!("error during dispatch: {}", e),
                         |ve|apply(ve, &mut repo));
                },
                Err(e) => println!("inter task comm error {}", e),
            }
        }
    });
    MessageSender { inner: sender }
}
