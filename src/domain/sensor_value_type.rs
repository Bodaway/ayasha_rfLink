use serde::Serialize;
use snafu::Snafu;
use std::fmt::Display;

#[derive(Debug, Snafu)]
#[snafu(visibility = "pub(crate)")]
pub enum ValueTypeError {
    #[snafu(display("temperature input invalid: {}", value))]
    InvalidTemperature { value: f64},

    #[snafu(display("humidity input invalid: {}", value))]
    InvalidHumidity { value:u32},
}

pub type Result<T, E = ValueTypeError> = std::result::Result<T, E>;

pub trait ValueType<T> {
    fn create(value: T) -> Result<Self> where Self: Sized;
    fn is_valid_value(value: T) -> bool;

}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum SensorValueType {
    Temperature(Temperature),
    Humidity(Humidity)
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Temperature(f64);

impl ValueType<f64> for Temperature {
    fn create(value: f64) -> Result<Temperature> {
        match Temperature::is_valid_value(value) {
            true => Ok(Temperature(value)),
            false => Err(ValueTypeError::InvalidTemperature{value: value})
        }
    }

    fn is_valid_value(value: f64) -> bool {
       match value {
           v if v > -50.0 && v < 70.0 => true,
           _ => false
       } 
    }
}
impl Display for Temperature {
    
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
    write!(f,"{}",self.0)
 }
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Humidity(u32);

impl ValueType<u32> for Humidity {
    fn create(value: u32) -> Result<Humidity> {
        match Humidity::is_valid_value(value) {
            true => Ok(Humidity(value)),
            false => Err(ValueTypeError::InvalidHumidity{value: value})
        }
    }

    fn is_valid_value(value: u32) -> bool {
       match value {
           v if v <= 100 => true,
           _ => false
       } 
    }
}
impl Display for Humidity {
    
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
    write!(f,"{}",self.0)
 }
}

