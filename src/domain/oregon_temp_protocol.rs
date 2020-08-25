use crate::domain::raw_frame::RawFrame;
use crate::domain::sensor::SensorValue;
use crate::domain::sensor_identifier::SensorIdentifier;
use crate::domain::sensor_value_type::{SensorValueType, Temperature, ValueType};
use crate::errors::Result;
use chrono::NaiveDateTime;

#[derive(Debug, PartialEq)]
pub struct OregonTempData {
    pub sensor_id: String,
    pub temperature: f64,
    pub battery_state: String,
    pub timestamp: NaiveDateTime,
}

impl OregonTempData {
    pub fn from_raw(raw: &RawFrame) -> Result<OregonTempData> {
        Ok(OregonTempData {
            sensor_id: "".to_string(),
            temperature: 0.0,
            battery_state: "".to_string(),
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
    }
}
