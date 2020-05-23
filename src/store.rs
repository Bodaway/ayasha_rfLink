use crate::models::*;
use std::collections::HashMap;

pub struct Store {
    local: HashMap<String, Vec<Box<dyn RfData>>>,
}

impl Store {
    pub fn init(base_data: Vec<Box<dyn RfData>>) -> Store {
        let mut loc: HashMap<String, Vec<Box<dyn RfData>>> =
            HashMap::<String, Vec<Box<dyn RfData>>>::new();

        for data in base_data {
            let id = &data.get_id();
            if !loc.contains_key(id) {
                loc.insert(data.get_id(), vec![data]);
            } else {
                loc.get_mut(id).unwrap().push(data);
            }
        }
        Store { local: loc }
    }

    pub fn clean(&mut self) {
        for data in self.local.values_mut() {
            data.retain(|d| d.get_date().date() == chrono::Local::today().naive_local());
        }
    }

    pub fn is_new_value<T: RfData>(&self, n_data: &T) -> bool {
        let id = n_data.get_id();
        let last = self
            .local
            .get(&id)
            .map(|v| v.last())
            .flatten()
            .map(|l| l.downcast_ref::<T>())
            .flatten();
        match last {
            None => true,
            Some(l) => n_data.values_is_diff(&l),
        }
    }
    pub fn insert(&mut self, data: Box<dyn RfData>) {
        let id = data.get_id();
        if !self.local.contains_key(&id) {
            self.local.insert(id, vec![data]);
        } else {
            self.local.get_mut(&id).unwrap().push(data);
        }
    }
}

#[test]
fn init_a_store() {
    use crate::lacrosse_v3_protocol::LaCrosseData;
    use chrono::Local;

    let data: Vec<Box<dyn RfData>> = vec![
        Box::new(LaCrosseData {
            sensor_id: "0".into(),
            temperature: 10.1,
            humidity: 50,
            timestamp: Local::now().naive_local(),
        }),
        Box::new(LaCrosseData {
            sensor_id: "0".into(),
            temperature: 10.1,
            humidity: 70,
            timestamp: Local::now().naive_local(),
        }),
        Box::new(LaCrosseData {
            sensor_id: "1".into(),
            temperature: 10.1,
            humidity: 90,
            timestamp: Local::now().naive_local(),
        }),
    ];

    let mut st = Store::init(data);
    let vector = st.local.get_mut("0").unwrap();
    let data = vector.last().unwrap();
    let last = data.downcast_ref::<LaCrosseData>();

    assert_eq!(2, vector.len());
    assert_eq!(70, last.unwrap().humidity);
}
#[test]
fn clean_a_store() {
    use crate::lacrosse_v3_protocol::LaCrosseData;
    use chrono::Duration;
    use chrono::Local;

    let data: Vec<Box<dyn RfData>> = vec![
        Box::new(LaCrosseData {
            sensor_id: "0".into(),
            temperature: 10.1,
            humidity: 50,
            timestamp: Local::now().naive_local(),
        }),
        Box::new(LaCrosseData {
            sensor_id: "0".into(),
            temperature: 10.1,
            humidity: 70,
            timestamp: Local::now().naive_local() - Duration::days(1),
        }),
        Box::new(LaCrosseData {
            sensor_id: "1".into(),
            temperature: 10.1,
            humidity: 70,
            timestamp: Local::now().naive_local(),
        }),
    ];

    let mut st = Store::init(data);
    st.clean();
    let vector = st.local.get_mut("0").unwrap();
    let data = vector.last().unwrap();
    let last = data.downcast_ref::<LaCrosseData>();

    assert_eq!(1, vector.len());
    assert_eq!(50, last.unwrap().humidity);
}
#[test]
fn is_new_in_a_store() {
    use crate::lacrosse_v3_protocol::LaCrosseData;
    use chrono::Local;

    let data: Vec<Box<dyn RfData>> = vec![Box::new(LaCrosseData {
        sensor_id: "0".into(),
        temperature: 10.1,
        humidity: 50,
        timestamp: Local::now().naive_local(),
    })];
    let data1 = LaCrosseData {
        sensor_id: "0".into(),
        temperature: 10.1,
        humidity: 50,
        timestamp: Local::now().naive_local(),
    };
    let data2 = LaCrosseData {
        sensor_id: "1".into(),
        temperature: 10.2,
        humidity: 50,
        timestamp: Local::now().naive_local(),
    };
    let data3 = LaCrosseData {
        sensor_id: "0".into(),
        temperature: 10.1,
        humidity: 52,
        timestamp: Local::now().naive_local(),
    };

    let st = Store::init(data);

    assert_eq!(st.is_new_value(&data1), false);
    assert_eq!(st.is_new_value(&data2), true);
    assert_eq!(st.is_new_value(&data3), true);
}
