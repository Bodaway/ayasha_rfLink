use serde::Serialize;

#[derive(PartialEq, Debug, Clone, Serialize)]
pub struct SensorIdentifier {
    pub probe_id: String,
    pub protocol: String,
    pub probe_value_name: String
}

impl SensorIdentifier {
    pub fn new(_probe_id: &str, _protocol: &str, _probe_value_name: &str) -> SensorIdentifier {
        SensorIdentifier {
            probe_id: _probe_id.to_string(),
            protocol: _protocol.to_string(),
            probe_value_name: _probe_value_name.to_string()
        }
    }
}