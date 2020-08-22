pub mod raw_frame;
mod frame;
mod lacrosse_v3_protocol;
mod sensor_identifier;
pub mod sensor;

use raw_frame::RawFrame;
use frame::Frame;
use sensor::{SensorValue,SensorRepository};
use crate::errors::Result;

pub fn listen(data: &str, repo: &mut SensorRepository) -> Result<()> {
    let raw = RawFrame::new(data);
    let frame = Frame::decrypt_raw(&raw)?;
    let sensors = frame.obtain_sensor_values();

    Ok(())
}