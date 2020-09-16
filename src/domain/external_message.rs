use crate::domain::errors::*;
use serde::Serialize;
use snafu::ResultExt;

pub struct ExternalMessage {
    pub routing_key: String,
    pub inner_message: String
}

#[derive(Serialize)]
pub struct InnerMessage<T> where T: Serialize {
    node_name: String,
    timestamp: chrono::NaiveDateTime,
    title: String,
    inner: T
}

pub trait MessageSender {
    fn send(&self,msg: ExternalMessage) -> Result<()>;
}

pub fn get_external_message<T>(msg_title:String,value: &T) -> Result<ExternalMessage> where T: Serialize {
    let inner = InnerMessage{
        node_name: "ayasha_rflink".to_string(),
        timestamp: chrono::Local::now().naive_local(),
        title: msg_title,
        inner: value
    };

    let msg = ExternalMessage{
        routing_key: "atmosSensor".to_string(),
        inner_message: serde_json::to_string(&inner).context(DataFormatingError)?
    };

    Ok(msg)

}