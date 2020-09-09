use crate::domain::command_event::Command;
use std::sync::mpsc::channel;
use tokio::task;

use crate::domain::{dispatch, apply};
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
        let mut repo = SensorRepository::new();
        loop {
            match receiver.recv() {
                Ok(command) => {
                    let r_events = dispatch(command, &repo);
                    match r_events {
                        Ok(events) => apply(events,&mut repo),
                        Err(e) => println!("error during dispatch: {}", e)
                    }
                    
                },
                Err(e) => println!("inter task comm error {}", e),
            }
        }
    });
    MessageSender { inner: sender }
}
