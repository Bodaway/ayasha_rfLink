use crate::domain::sensor_identifier::SensorIdentifier;
use std::cell::RefCell;

#[derive(Debug, PartialEq, Clone)]
pub enum SensorValueType {
    Number(f64),
}

#[derive(Clone)]
pub struct Sensor {
    id: SensorIdentifier,
    values: RefCell<Vec<SensorValue>>,
}

impl Sensor {
    pub fn new(id: &SensorIdentifier) -> Sensor {
        Sensor {
            id: SensorIdentifier::new(
                id.probe_id.as_ref(),
                id.protocol.as_ref(),
                id.probe_value_name.as_ref(),
            ),
            values: RefCell::new(vec![]),
        }
    }
    pub fn add_value(&self, value: SensorValue) {
        self.values.borrow_mut().push(value);
    }
    fn get_last(&self) -> Option<SensorValue> {
        self.values.borrow().last().and_then(|s| Some(s.clone()))
    }
}

#[derive(Clone)]
pub struct SensorValue {
    pub id: SensorIdentifier,
    pub timestamp: chrono::NaiveDateTime,
    pub value: SensorValueType,
}

pub struct SensorRepository {
    sensors: RefCell<Vec<Sensor>>,
}

impl SensorRepository {
    pub fn new() -> SensorRepository {
        SensorRepository {
            sensors: RefCell::new(vec![]),
        }
    }
    pub fn add_value(&self, id: &SensorIdentifier, value: SensorValue) {
        let mut sensors = self.sensors.borrow_mut();
        let sensor = sensors.iter().find(|s| &s.id == id);
        match sensor {
            Some(s) => {
                s.add_value(value);
            }
            None => {
                let nsensor = Sensor::new(id);
                nsensor.add_value(value);
                sensors.push(nsensor)
            }
        }
    }
    fn extract_sensor(&self, id: &SensorIdentifier) -> Option<Sensor> {
        let sensors = self.sensors.borrow();
        let sensor = sensors.iter().find(|s| &s.id == id);
        sensor.and_then(|s| Some(s.clone()))
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
        let repo = SensorRepository::new();

        {
            assert_eq!(repo.sensors.borrow().len(), 0);
            repo.add_value(&id, value);
        }
        assert_eq!(repo.sensors.borrow().len(), 1);

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

        let repo = SensorRepository::new();

        assert_eq!(repo.sensors.borrow().len(), 0);
        repo.add_value(&id, value);
        repo.add_value(&id2, value2);

        assert_eq!(repo.sensors.borrow().len(), 2);

        let finded = repo.extract_sensor(&id);
        assert_eq!(
            finded.unwrap().get_last().unwrap().value,
            SensorValueType::Number(10.0)
        )
    }
}
