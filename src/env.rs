use dotenv::dotenv;
use std::env;
use crate::errors::Result;
use crate::errors::RfError;
use crate::errors::ReadEnvError;
use std::str::FromStr;
use snafu::ResultExt;

pub const DATABASE : &str = &"DATABASE";

pub fn get_env<T>(name : &str) -> Result<T> where T : FromStr {
    dotenv().ok();
    let r_parsed = env::var(name).context(ReadEnvError)?.parse::<T>();
    match r_parsed {
        Ok(p) => Ok(p),
        Err(_) => Err(RfError::ParsingEnvError{value: format!("error during parsing {}",name)})
    }
}
