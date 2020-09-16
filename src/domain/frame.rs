use crate::domain::lacrosse_v3_protocol::LaCrosseData;
use crate::domain::oregon_temp_protocol::OregonTempData;
use crate::domain::raw_frame::RawFrame;
use crate::domain::sensor::SensorValue;
use crate::domain::errors::*;

use snafu::ResultExt;

#[derive(Debug, PartialEq)]
pub enum Frame {
    LaCrosseV3(LaCrosseData),
    OregonSc(OregonTempData),
    Unknow(RawFrame),
}

impl Frame {
    pub fn decrypt_raw(raw: &RawFrame) -> Result<Frame> {
        match raw {
            r if crate::domain::lacrosse_v3_protocol::is_valid_raw(&r) => 
                LaCrosseData::from_raw(&r)
                .and_then(|r| Ok(Frame::LaCrosseV3(r)))
                .context(InternalLacrosseError),
            r if crate::domain::oregon_temp_protocol::is_valid_raw(&r) => 
                OregonTempData::from_raw(&r)
                .and_then(|r| Ok(Frame::OregonSc(r)))
                .context(InternalOregonError),
            _ => Ok(Frame::Unknow(raw.clone())),
        }
    }

    pub fn obtain_sensor_values(&self) -> Vec<SensorValue> {
        match self {
            Frame::Unknow(_) => vec![],
            Frame::LaCrosseV3(f) => f.to_sensors_values(),
            Frame::OregonSc(f) => f.to_sensors_values() 
        }
    } 
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_frame_normal() {
        let data = RawFrame::new("0;10;40");
        let frame = Frame::decrypt_raw(&data);

        match frame {
            Err(_) => assert!(false,"frame should be unknow"),
            Ok(f) => match f {
                Frame::Unknow(_) => assert!(true),
                _ =>  assert!(false,"frame should be unknow")
            }
        }
    }
    #[test]
    fn create_frame_lacrosse() {
        let data = RawFrame::new("test;test;DEBUG;Pulses=511");
        let frame = Frame::decrypt_raw(&data);

        match frame {
            Err(_) => assert!(false,"frame should be unknow"),
            Ok(f) => match f {
                Frame::Unknow(_) => assert!(true),
                _ =>  assert!(false,"frame should be unknow")
            }
        }
    }
}
