use crate::domain::raw_frame::RawFrame;
use crate::domain::sensor::SensorValue;
use crate::domain::sensor_identifier::SensorIdentifier;
use crate::domain::sensor_value_type::{SensorValueType, Temperature, ValueType};
use chrono::NaiveDateTime;
use snafu::ResultExt;

use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility = "pub(crate)")]
pub enum OregonError {
    #[snafu(display("Invalid Frame"))]
    InvalidFrameError,

    #[snafu(display("parsing failure for value {}", value))]
    ParsingFrameError {
        value: String,
        source: std::num::ParseIntError,
    },
}

pub type Result<T, E = OregonError> = std::result::Result<T, E>;
#[derive(Debug, PartialEq)]
pub struct OregonTempData {
    pub sensor_id: String,
    pub temperature: f64,
    pub battery_state: String,
    pub timestamp: NaiveDateTime,
}

pub fn is_valid_raw(raw: &RawFrame) -> bool {
    let splitted = raw.data.split(';').collect::<Vec<&str>>();
    match splitted[2] {
        x if x == "Oregon Temp" => true,
        _ => false
    }
}

impl OregonTempData {
    pub fn from_raw(raw: &RawFrame) -> Result<OregonTempData> {
        //"20;03;Oregon Temp;ID=0410;TEMP=0153;BAT=OK;"
        let splitted = raw.data.split(';').collect::<Vec<&str>>();
        let extract_value =
         isize::from_str_radix(&splitted[4][5..], 16)
         .context(ParsingFrameError{value:&splitted[4][5..]})?;

        Ok(OregonTempData {
            sensor_id: splitted[3][3..].to_string(),
            temperature: ( extract_value as f64 / 10.0),
            battery_state: splitted[5][4..].to_string(),
            timestamp: chrono::Local::now().naive_local(),
        })
    }
    fn get_protocol() -> String {
        "oregon_temp".to_string()
    }
    pub fn to_sensors_values(&self) -> Vec<SensorValue> {
        let temp_id = SensorIdentifier::new(
            &self.sensor_id,
            &OregonTempData::get_protocol(),
            "temperature",
        );
        let typed_value = Temperature::create(self.temperature);
        match typed_value {
            Ok(t) => {
                let temp_value = SensorValue {
                    id: temp_id,
                    timestamp: self.timestamp,
                    value: SensorValueType::Temperature(t),
                };

                vec![temp_value]
            }
            Err(e) => {
                print!("error during typing value oregon");
                vec![]
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn from_raw_ok() {
        let raw = RawFrame::new("20;03;Oregon Temp;ID=0410;TEMP=0153;BAT=OK;");
        let r_data = OregonTempData::from_raw(&raw);

        assert_eq!(r_data.is_ok(), true);
        let data = r_data.unwrap();
        assert_eq!(data.sensor_id, "0410");
        assert_eq!(data.battery_state, "OK");
        assert_eq!(data.temperature, 33.9);
    }
}