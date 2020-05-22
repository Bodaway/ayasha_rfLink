use crate::db_job::Conn;
use crate::models::rf_data_dao;
use rusqlite::params;
use crate::errors::{Result, RfError};

pub fn insert_sensor_data(conn: &mut Option<Conn>, dao: &'static rf_data_dao) -> Result<()> {
    if let Some(c) = conn.as_mut() {
        let r = c.execute(
            "insert into sensors_data (id, dt_start,dt_end,temperature, humidity)
                    values (?1,?2,?3,?4,?5);",
            params![
                dao.id,
                dao.protocol,
                dao.dt_start,
                dao.dt_end,
                dao.temperature,
                dao.humidity
            ],
        );
        match r {
            Ok(_) => Ok(()),
            Err(e) => Err(RfError::DbError { source: e }),
        }
    } else {
        Err(RfError::DbAccessError {
            value: "access error during migration".into(),
        })
    }
}
