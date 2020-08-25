use crate::domain::sensor::{SensorRepository, SensorValue};
use crate::domain::errors::Result;

pub type GetDataFunction = Box<dyn FnOnce(&SensorRepository) -> Result<Vec<Event>> + Send>;

pub enum Command {
    Rejeu(Vec<Event>),
    IncomingData(String),
    GetData(GetDataFunction),
}

pub enum Event {
    ValueChanged(SensorValue)
}