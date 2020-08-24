use chrono::NaiveDateTime;

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
    /*fn from_string(data: String) -> RawFrame {
        RawFrame::new(data.as_str())
    }
    pub fn is_debug(&self) -> bool {
        let vec = self.to_vec();
        vec.len() > 2 && vec[2] == "RFDEBUG=ON"
    }

    pub fn from_utf8(data: Vec<u8>) -> Result<RawFrame> {
        Ok(RawFrame::from_string(
            String::from_utf8(data).context(Utf8RawConvertError)?,
        ))
    }*/
    fn to_vec(&self) -> Vec<&str> {
        self.data.split(';').collect::<Vec<&str>>()
    }

    pub fn get_debug_data(&self) -> String {
        let raw_vec = self.to_vec();
        (&raw_vec[4][13..]).to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
}
