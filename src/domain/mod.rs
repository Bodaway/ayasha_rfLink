mod frame;
mod lacrosse_v3_protocol;
mod oregon_temp_protocol;

pub mod command_event;
pub mod errors;
pub mod raw_frame;
pub mod sensor;
pub mod external_message;
mod sensor_identifier;
mod sensor_value_type;

use snafu::ResultExt;

use command_event::{Command, Event};
use errors::*;
use frame::Frame;
use raw_frame::RawFrame;
use sensor::SensorRepository;
use external_message::MessageSender;

pub fn dispatch(command: Command, repo: &SensorRepository) -> Result<Vec<Event>> {
    match command {
        Command::Rejeu(events) => Ok(events),
        Command::IncomingData(input) => dispatch_input(&input, &repo),
        Command::GetData(getter) => getter(&repo),
    }
}

fn dispatch_input(data: &str, repo: &SensorRepository) -> Result<Vec<Event>> {
    let raw = RawFrame::new(data);
    let frame = Frame::decrypt_raw(&raw)?;
    match frame {
        Frame::Unknow(raw) => Ok(vec![Event::UnknowDataReceived(raw)]),
        _ => {
let sensors = frame.obtain_sensor_values();

    sensors
        .into_iter()
        .map(|s| match repo.extract_sensor(&s.id) {
            None => Some(Ok(Event::ValueChanged(s))),
            Some(sensor) => sensor.get_last().and_then(|last| {
                let to_be_insert = last
                    .value
                    .is_signifiant_variation(s.value.clone())
                    .context(InvalidSensorValueError);
                match to_be_insert {
                    Err(e) => Some(Err(e)),
                    Ok(b) => match b {
                        false => None,
                        true => Some(Ok(Event::ValueChanged(s))),
                    },
                }
            }),
        })
        .filter_map(|e| e)
        .collect()
        }
    }
    
}

pub fn apply(events: Vec<Event>, repo: &mut SensorRepository) {
    for ev in events {
        match ev {
            Event::ValueChanged(value) => repo.add_value(value),
            Event::UnknowDataReceived(_) => (),
        };
    }
}

pub fn send_external_message(events: Vec<Event>, sender: &dyn MessageSender) -> Result<Vec<Event>> {
 events.into_iter().map(|ev|{
        match &ev {
            Event::ValueChanged(value) => sender.send(external_message::get_external_message("SensorValueChanged".to_string(),&value)?)?,
            Event::UnknowDataReceived(raw) => sender.send(external_message::get_external_message("SensorUnknowDataReceived".to_string(),&raw)?)?,
        };
        Ok(ev)
    }).collect::<Result<Vec<Event>>>()
}
