use snafu::Snafu;
use crate::domain::sensor_value_type::ValueTypeError;
use crate::domain::lacrosse_v3_protocol::LacrosseError;
use crate::domain::oregon_temp_protocol::OregonError;

#[derive(Debug, Snafu)]
#[snafu(visibility = "pub(crate)")]
pub enum DomainError {
    #[snafu(display("Invalid Sensor Value : {}", source.to_string()))]
    InvalidSensorValueError { source: ValueTypeError },

    #[snafu(display("lacrosse : {}", source.to_string()))]
    InternalLacrosseError { source: LacrosseError},

    #[snafu(display("oregon : {}", source.to_string()))]
    InternalOregonError { source: OregonError},

    #[snafu(display("error during data extraction: {}", value))]
    DataExtractionError { value: String },

    #[snafu(display("error during data formating: {}", source.to_string()))]
    DataFormatingError { source: serde_json::Error },
}

pub type Result<T, E = DomainError> = std::result::Result<T, E>;
