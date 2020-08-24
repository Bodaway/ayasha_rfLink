use crate::errors::Result;
use std::sync::mpsc::channel;
use tokio::task;

use crate::domain::listen;
use crate::domain::sensor::SensorRepository;

type GetDataFunction = Box<dyn FnOnce(&SensorRepository) -> Result<()> + Send>;

pub enum Message {
    IncomingData(String),
    GetData(GetDataFunction),
}

#[derive(Clone)]
pub struct MessageSender {
    inner: std::sync::mpsc::Sender<Message>,
}

impl MessageSender {
    pub fn send(&self, mess: Message) {
        self.inner.send(mess).expect("comm error with state actor");
    }
}

pub fn init_actor() -> MessageSender {
    let (sender, receiver) = channel::<Message>();
    task::spawn(async move {
        let mut repo = SensorRepository::new();
        loop {
            match receiver.recv() {
                Ok(message) => {
                    let result = match message {
                        Message::IncomingData(input) => listen(&input, &mut repo),
                        Message::GetData(getter) => getter(&repo),
                    };
                    match result {
                        Ok(_) => (),
                        Err(e) => println!("error in domain actor: {}", e),
                    };
                }
                Err(e) => println!("inter task comm error {}", e),
            }
        }
    });
    MessageSender { inner: sender }
}
