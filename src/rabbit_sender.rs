use futures_executor::LocalPool;
use crate::domain::external_message::{ExternalMessage,MessageSender};
use crate::domain::errors::Result;
use lapin::{
    options::*, BasicProperties, Connection,
    ConnectionProperties};
use snafu::ResultExt;

pub struct RabbitSender {
    connection_uri: String,
    exchange_key: String
}
impl RabbitSender {
    pub fn new(uri: String, exchange_key: String) -> RabbitSender {
        RabbitSender{connection_uri:uri,exchange_key:exchange_key}
    }
}

impl MessageSender for RabbitSender {
    fn send(&self, msg: ExternalMessage) -> Result<()> {
        send(self, msg)    
    }
}

async fn inner_send(sender: &RabbitSender, msg: ExternalMessage) -> Result<()> {
   
        let conn = Connection::connect(
            &sender.connection_uri,
            ConnectionProperties::default().with_default_executor(8),
        )
        .await;

        match conn {
            Err(e) => Err(e),
            Ok(c) => {
                let channel = c.create_channel().await;
                match channel {
                    Err(e) => Err(e),
                    Ok(ca) => {
                        ca
                            .basic_publish(
                            &sender.exchange_key,
                            &msg.routing_key,
                            BasicPublishOptions::default(),
                            msg.inner_message.as_bytes().to_vec(),
                            BasicProperties::default(),
                            )
                            .await.and_then(|_| Ok(()))
                    }
                }
            }
        }.context(crate::domain::errors::ExternalMessageError)
}
fn send(sender: &RabbitSender, msg: ExternalMessage) -> Result<()> {
    LocalPool::new().run_until(inner_send(sender,msg))
}