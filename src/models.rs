use crate::errors::*;
use chrono::NaiveDateTime;
use downcast_rs::Downcast;
use snafu::ResultExt;

pub struct RawFrame {
    pub data: String,
    pub timestamp: NaiveDateTime,
}

impl RawFrame {
    pub fn new(data: &str) -> RawFrame {
        RawFrame {
            data: String::from(data),
            timestamp: chrono::Local::now().naive_local(),
        }
    }
    pub fn from_string(data: String) -> RawFrame {
        RawFrame::new(data.as_str())
    }

    pub fn to_vec(&self) -> Vec<&str> {
        self.data.split(';').collect::<Vec<&str>>()
    }

    pub fn is_debug(&self) -> bool {
        let vec = self.to_vec();
        vec.len() > 2 && vec[2] == "RFDEBUG=ON"
    }

    pub fn from_utf8(data: Vec<u8>) -> Result<RawFrame> {
        Ok(RawFrame::from_string(
            String::from_utf8(data).context(Utf8RawConvertError)?,
        ))
    }
    pub fn get_debug_data(&self) -> String {
        let raw_vec = self.to_vec();
        (&raw_vec[4][13..]).to_string()
    }
}

pub struct RfDataDao {
    pub id: String,
    pub protocol: String,
    pub dt_start: chrono::NaiveDateTime,
    pub dt_end: Option<chrono::NaiveDateTime>,
    pub temperature: Option<f64>,
    pub humidity: Option<f64>,
}

pub trait RfData: Downcast {
    fn from_raw(raw: &RawFrame) -> Result<Self>
    where
        Self: Sized;
    //fn is_valid_raw(raw: &RawFrame) -> bool;
    fn get_date(&self) -> NaiveDateTime;
    fn get_id(&self) -> String;
    fn values_is_diff(&self, other: &Self) -> bool
    where
        Self: Sized;
    fn to_dao(&self) -> RfDataDao;
}
impl_downcast!(RfData);
