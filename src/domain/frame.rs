use crate::domain::lacrosse_v3_protocol::is_valid_raw;
use crate::domain::lacrosse_v3_protocol::LaCrosseData;
use crate::domain::raw_frame::RawFrame;
use crate::domain::sensor::SensorValue;
use crate::errors::Result;

#[derive(Debug, PartialEq)]
pub enum Frame {
    LaCrosseV3(LaCrosseData),
    Unknow,
}

impl Frame {
    pub fn decrypt_raw(raw: &RawFrame) -> Result<Frame> {
        match raw {
            r if is_valid_raw(&r) => LaCrosseData::from_raw(&r).and_then(|r| Ok(Frame::LaCrosseV3(r))),
            _ => Ok(Frame::Unknow),
        }
    }

    pub fn obtain_sensor_values(&self) -> Vec<SensorValue> {
        match self {
            Frame::Unknow => vec![],
            Frame::LaCrosseV3(f) => f.to_sensors_values()
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
                Frame::Unknow => assert!(true),
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
                Frame::Unknow => assert!(true),
                _ =>  assert!(false,"frame should be unknow")
            }
        }
    }
}
