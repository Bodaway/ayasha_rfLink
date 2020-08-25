mod frame;
mod lacrosse_v3_protocol;
mod oregon_temp_protocol;

pub mod command_event;
pub mod raw_frame;
pub mod sensor;
mod sensor_value_type;
mod sensor_identifier;
pub mod errors;

use errors::Result;
use command_event::{Command, Event};
use frame::Frame;
use raw_frame::RawFrame;
use sensor::SensorRepository;

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
    let sensors = frame.obtain_sensor_values();

    for sensor in sensors {
        unimplemented!();
    }
    Ok(vec![])
}

pub fn apply(events: Vec<Event>, repo: &mut SensorRepository) {
    for ev in events {
        match ev {
            Event::ValueChanged(value) => repo.add_value(value),
        };
    }
}
