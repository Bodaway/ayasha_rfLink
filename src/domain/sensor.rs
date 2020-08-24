use crate::domain::sensor_identifier::SensorIdentifier;
use std::cell::RefCell;
use serde::Serialize;
use crate::errors::*;
use snafu::ResultExt;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum SensorValueType {
    Number(f64),
}

#[derive(Clone, Serialize)]
pub struct Sensor {
    id: SensorIdentifier,
    values: Vec<SensorValue>,
}

impl Sensor {
    pub fn new(id: &SensorIdentifier) -> Sensor {
        Sensor {
            id: SensorIdentifier::new(
                id.probe_id.as_ref(),
                id.protocol.as_ref(),
                id.probe_value_name.as_ref(),
            ),
            values: vec![],
        }
    }
    pub fn add_value(&mut self, value: SensorValue) {
        match value.value {
            SensorValueType::Number(x) => println!("ajout de la valeur {}", x),
            _ => ()
        }
        
        self.values.push(value);
    }
    fn get_last(&self) -> Option<SensorValue> {
        self.values.last().and_then(|s| Some(s.clone()))
    }
}

#[derive(Clone, Serialize)]
pub struct SensorValue {
    pub id: SensorIdentifier,
    pub timestamp: chrono::NaiveDateTime,
    pub value: SensorValueType,
}

pub struct SensorRepository {
    sensors: Vec<Sensor>,
}
unsafe impl Send for SensorRepository {}
unsafe impl Sync for SensorRepository {}


impl SensorRepository {
    pub fn new() -> SensorRepository {
        SensorRepository {
            sensors: vec![],
        }
    }
    pub fn add_value(&mut self, value: SensorValue) {
        let sensor = self.sensors.iter_mut().find(|s| s.id == value.id);
        match sensor {
            Some(s) => {
                s.add_value(value);
            }
            None => {
                let mut nsensor = Sensor::new(&value.id);
                nsensor.add_value(value);
                self.sensors.push(nsensor)
            }
        }
    }
    fn extract_sensor(&self, id: &SensorIdentifier) -> Option<Sensor> {
        let sensor = self.sensors.iter().find(|s| &s.id == id);
        sensor.and_then(|s| Some(s.clone()))
    }

    pub fn get_all_state(&self) -> Result<String> {
        serde_json::to_string(&self.sensors).context(SerialisationError)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn new_sensor() {
        let id = SensorIdentifier::new("probeid", "protocol", "name");
        let sensor = Sensor::new(&id);
        assert_eq!(sensor.id.probe_id, "probeid");
        assert_eq!(sensor.id.protocol, "protocol");
        assert_eq!(sensor.id.probe_value_name, "name");
    }

    #[test]
    fn add_first_in_repo() {
        let id = SensorIdentifier::new("probeid", "protocol", "name");

        let value = SensorValue {
            id: id.clone(),
            timestamp: chrono::Local::now().naive_local(),
            value: SensorValueType::Number(10.0),
        };
        let mut repo = SensorRepository::new();

        {
            assert_eq!(repo.sensors.len(), 0);
            repo.add_value(value);
        }
        assert_eq!(repo.sensors.len(), 1);

        let finded = repo.extract_sensor(&id);
        assert_eq!(
            finded.unwrap().get_last().unwrap().value,
            SensorValueType::Number(10.0)
        )
    }
    #[test]
    fn add_second_in_repo() {
        let id = SensorIdentifier::new("probeid", "protocol", "name");
        let id2 = SensorIdentifier::new("probeid2", "protocol", "name");

        let value = SensorValue {
            id: id.clone(),
            timestamp: chrono::Local::now().naive_local(),
            value: SensorValueType::Number(10.0),
        };
        let value2 = SensorValue {
            id: id2.clone(),
            timestamp: chrono::Local::now().naive_local(),
            value: SensorValueType::Number(11.0),
        };

        let mut repo = SensorRepository::new();

        assert_eq!(repo.sensors.len(), 0);
        repo.add_value(value);
        repo.add_value(value2);

        assert_eq!(repo.sensors.len(), 2);

        let finded = repo.extract_sensor(&id);
        assert_eq!(
            finded.unwrap().get_last().unwrap().value,
            SensorValueType::Number(10.0)
        )
    }
}
