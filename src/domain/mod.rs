mod frame;
mod lacrosse_v3_protocol;
pub mod raw_frame;
pub mod sensor;
mod sensor_identifier;

use crate::errors::Result;
use frame::Frame;
use raw_frame::RawFrame;
use sensor::{SensorRepository, SensorValue};

pub fn listen(data: &str, repo: std::sync::Arc<SensorRepository>) -> Result<()> {
    let raw = RawFrame::new(data);
    let frame = Frame::decrypt_raw(&raw)?;
    let sensors = frame.obtain_sensor_values();

    sensors.into_iter().map(|sv| repo.add_value(sv));

    Ok(())
}
