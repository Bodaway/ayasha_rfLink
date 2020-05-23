use crate::db_job::Conn;
use crate::errors::{Result, RfError};
use crate::models::RfDataDao;
use rusqlite::params;

pub fn insert_sensor_data(conn: &mut Option<Conn>, dao: RfDataDao) -> Result<()> {
    if let Some(c) = conn.as_mut() {

        let r1 = c.execute(
            "update sensors_data
            set dt_end = ?3
            where id = ?1 and protocol = ?2 and dt_end is null;",
            params![dao.id, dao.protocol, dao.dt_start,],
        );

        let r2 = c.execute(
            "insert into sensors_data (id,protocol, dt_start,dt_end,temperature, humidity)
                    values (?1,?2,?3,?4,?5,?6);",
            params![
                dao.id,
                dao.protocol,
                dao.dt_start,
                dao.dt_end,
                dao.temperature,
                dao.humidity
            ],
        );
        match r1.or(r2) {
            Ok(_) => {
                Ok(())
            },
            Err(e) => Err(RfError::DbError { source: e }),
        }
    } else {
        Err(RfError::DbAccessError {
            value: "access error during migration".into(),
        })
    }
}
