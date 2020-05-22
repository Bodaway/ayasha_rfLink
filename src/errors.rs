use serial::Error as serial_error;
use snafu::Snafu;
use std::io::Error as io_error;
use std::string::FromUtf8Error;

#[derive(Debug, Snafu)]
#[snafu(visibility = "pub(crate)")]
pub enum RfError {
    #[snafu(display("Not implement error"))]
    NotImplementedError,

    #[snafu(display("error during serial configuration : {}", source.to_string()))]
    ConfigurationError { source: serial_error },

    #[snafu(display("error during serial read : {}", source.to_string()))]
    ReadError { source: io_error },
    #[snafu(display("error during utf8 convertion : {}", source.to_string()))]
    Utf8RawConvertError { source: FromUtf8Error },
    #[snafu(display("No valid frame in data"))]
    NoValidFrame,
    #[snafu(display("parsing failure for value {}", value))]
    ParsingFrameError {
        value: String,
        source: std::num::ParseIntError,
    },    
    #[snafu(display("error during reading env : {}", source.to_string()))]
    ReadEnvError { source:std::env::VarError},
    #[snafu(display("error during parsing env : {}", value))]
    ParsingEnvError { value : String},
    #[snafu(display("error during db migration : {}", source.to_string()))]
    MigrationDbError { source: refinery::Error},
    #[snafu(display("error cannot access connection db : {}", value))]
    DbAccessError { value : String},
    #[snafu(display("error db : {}", source.to_string()))]
    DbError { source : rusqlite::Error},

}pub type Result<T, E = RfError> = std::result::Result<T, E>;
