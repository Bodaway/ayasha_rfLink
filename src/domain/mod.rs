mod frame;
mod lacrosse_v3_protocol;
pub mod raw_frame;
pub mod sensor;
mod sensor_identifier;

use crate::errors::Result;
use frame::Frame;
use raw_frame::RawFrame;
use sensor::{SensorRepository, SensorValue};

pub fn listen(data: &str, repo: &mut SensorRepository) -> Result<()> {
    let raw = RawFrame::new(data);
    let frame = Frame::decrypt_raw(&raw)?;
    let sensors = frame.obtain_sensor_values();

    for sv in sensors {
        repo.add_value(sv);
    }

    Ok(())
}
