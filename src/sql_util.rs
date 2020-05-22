



/// ISO 8601 combined date and time without timezone =>
/// "YYYY-MM-DDTHH:MM:SS.SSS"
impl ToSql for NaiveDateTime {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        let date_str = self.format("%Y-%m-%dT%H:%M:%S%.f").to_string();
        Ok(ToSqlOutput::from(date_str))
    }
}

/// "YYYY-MM-DD HH:MM:SS"/"YYYY-MM-DD HH:MM:SS.SSS" => ISO 8601 combined date
/// and time without timezone. ("YYYY-MM-DDTHH:MM:SS"/"YYYY-MM-DDTHH:MM:SS.SSS"
/// also supported)
impl FromSql for NaiveDateTime {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        value.as_str().and_then(|s| {
            let fmt = if s.len() >= 11 && s.as_bytes()[10] == b'T' {
                "%Y-%m-%dT%H:%M:%S%.f"
            } else {
                "%Y-%m-%d %H:%M:%S%.f"
            };

            match NaiveDateTime::parse_from_str(s, fmt) {
                Ok(dt) => Ok(dt),
                Err(err) => Err(FromSqlError::Other(Box::new(err))),
            }
        })
    }
}
