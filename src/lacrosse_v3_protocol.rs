use crate::errors::*;
use crate::models::*;
use chrono::NaiveDateTime;
use snafu::ResultExt;

pub struct LaCrosseData {
    pub sensor_id: String,
    pub temperature: f64,
    pub humidity: i32,
    pub timestamp: NaiveDateTime,
}

impl RfData for LaCrosseData {
    fn from_raw(raw: &RawFrame) -> Result<LaCrosseData> {
        decrypt(&raw)
    }
    fn get_date(&self) -> NaiveDateTime {
        self.timestamp
    }
    fn get_id(&self) -> String {
        self.sensor_id.clone()
    }
    fn values_is_diff(&self, other: &Self) -> bool
    where
        Self: Sized,
    {
        let diff_temp = ((self.temperature - other.temperature) * 100.0).round() / 100.0;
        let diff_hum = self.humidity - other.humidity;

        diff_temp.abs() >= 0.2 || diff_hum.abs() >= 1
    }
    fn to_dao(&self) -> RfDataDao {
        RfDataDao {
            id: self.sensor_id.clone(),
            protocol: "lacrosse_v3".into(),
            dt_start: self.timestamp,
            dt_end: None,
            temperature: Some(self.temperature),
            humidity: Some(self.humidity as f64),
        }
    }
}

pub fn is_valid_raw(raw: &RawFrame) -> bool {
    let signal = raw.data.split(';').collect::<Vec<&str>>();
    if signal[2] == "DEBUG" && signal[3] == "Pulses=511" {
        true
    } else {
        false
    }
}
fn decrypt(raw: &RawFrame) -> Result<LaCrosseData> {
    //if pulse_number != "511" {warn!("pulse number different du standart LaCrosse 511 : {}", pulse_number)};
    let debug_data = raw.get_debug_data();
    let signal = debug_data.split(',').collect::<Vec<&str>>();

    let tuple_pulse = to_tuple_pulse(&signal)?;
    let binary_signal = binarize(tuple_pulse);
    //debug!("signal : {}", binary_signal);
    let binary_frames = binary_signal
        .split("hhhh")
        .into_iter()
        .filter(|x| x.len() == 41)
        .collect::<Vec<&str>>();
    if binary_frames.len() == 0 {
        return Err(RfError::NoValidFrame);
    }
    if binary_frames.len() != 4 {
        println!(
            " {} frames trouver au lieu des 4 prévu",
            binary_frames.len()
        )
    }
    //if binary_frames[0].len() != binary_frames[1].len() { }

    let w_frame = binary_frames[0];

    let id_bin = isize::from_str_radix(&w_frame[..8], 2)
        .context(ParsingFrameError {
            value: String::from(&w_frame[..8]),
        })?
        .to_string();
    let temp_bin = &w_frame[12..24];
    let temp_val: f64 =
        (isize::from_str_radix(reverse_binary(temp_bin).as_str(), 2).unwrap() as f64) / 10.0 - 50.0;
    let hum_bin = &w_frame[25..32];
    let hum_val =
        isize::from_str_radix(reverse_binary(hum_bin).as_str(), 2).context(ParsingFrameError {
            value: hum_bin.to_string(),
        })? as i32;

    Ok(LaCrosseData {
        sensor_id: id_bin,
        temperature: temp_val,
        humidity: hum_val,
        timestamp: raw.timestamp,
    })
}

fn to_tuple_pulse(signal: &Vec<&str>) -> Result<Vec<(i32, i32)>> {
    let mut i = 0;
    let mut done = false;
    let mut tuple_pulse: Vec<(i32, i32)> = Vec::new();
    let ended_index = signal.len() - 3;

    while !done {
        let t1 = signal[i]
            .parse::<i32>()
            .context(ParsingFrameError { value: signal[i] })?;
        let t2 = signal[i + 1].parse::<i32>().context(ParsingFrameError {
            value: signal[i + 1],
        })?;
        tuple_pulse.push((t1, t2));

        if i >= ended_index {
            done = true;
        }
        i = i + 2;
    }
    Ok(tuple_pulse)
}

fn binarize(tuple_signal: Vec<(i32, i32)>) -> String {
    tuple_signal
        .into_iter()
        .map(|t| match t {
            (x, y) if x > 450 && y > 450 => "h",
            (x, y) if x > y => "0",
            _ => "1",
        })
        .collect::<Vec<&str>>()
        .concat()
}

fn reverse_binary(frame: &str) -> String {
    let mut new_frame = String::new();

    for bit in frame.chars() {
        match bit {
            '1' => new_frame.push('0'),
            _ => new_frame.push('1'),
        }
    }
    new_frame
}

#[test]
fn compare_lacrosse_data() {
    use chrono::Local;
    let data1 = LaCrosseData {
        sensor_id: "0".into(),
        temperature: 10.1,
        humidity: 50,
        timestamp: Local::now().naive_local(),
    };
    let data2 = LaCrosseData {
        sensor_id: "0".into(),
        temperature: 10.2,
        humidity: 50,
        timestamp: Local::now().naive_local(),
    };
    let data3 = LaCrosseData {
        sensor_id: "0".into(),
        temperature: 10.3,
        humidity: 50,
        timestamp: Local::now().naive_local(),
    };
    let data5 = LaCrosseData {
        sensor_id: "0".into(),
        temperature: 10.4,
        humidity: 50,
        timestamp: Local::now().naive_local(),
    };
    let data4 = LaCrosseData {
        sensor_id: "0".into(),
        temperature: 10.1,
        humidity: 51,
        timestamp: Local::now().naive_local(),
    };

    assert_eq!(data1.values_is_diff(&data2), false);
    assert_eq!(data1.values_is_diff(&data4), true);
    assert_eq!(data1.values_is_diff(&data5), true);
    assert_eq!(data1.values_is_diff(&data3), true);
}
