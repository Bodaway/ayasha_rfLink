#![feature(vec_remove_item)]

extern crate serial;
extern crate snafu;
#[macro_use]
extern crate downcast_rs;

mod db;
mod db_job;
mod env;
mod errors;
mod lacrosse_v3_protocol;
mod models;
mod store;

use snafu::ResultExt;
use std::path::Path;
use std::time::Duration;

use crate::db_job::Conn;
use crate::db_job::DbQueryExecutor;
use crate::env::*;
use crate::errors::*;
use crate::lacrosse_v3_protocol::is_valid_raw;
use crate::lacrosse_v3_protocol::LaCrosseData;
use crate::models::*;
use crate::store::Store;
use serial::prelude::*;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("./migrations");
}

fn main() {
    let path = match env::get_env::<String>(DATABASE) {
        Ok(path) => path,
        Err(e) => panic!("error database access : {}", e.to_string()),
    };
    let db_path = Path::new(&path);
    let (_, db_exec) = db_job::start_thread(db_path);

    db_exec.spawn(|conn: &mut Option<Conn>| {
        if let Some(conn) = conn.as_mut() {
            let r = embedded::migrations::runner().run(conn);
            match r {
                Ok(_) => Ok(()),
                Err(e) => Err(RfError::MigrationDbError { source: e }),
            }
        } else {
            Err(RfError::DbAccessError {
                value: "access error during migration".into(),
            })
        }
    });

    let db_ex = db_exec.clone();
    std::thread::spawn(move || loop {
        let result = listen(&db_ex);
        match result {
            Ok(()) => (),
            Err(ex) => println!("{}", ex.to_string()),
        };
    });

    loop {}
}

fn listen(db_ex: &DbQueryExecutor) -> Result<()> {
    let mut port = serial::open("/dev/ttyACM0").context(ConfigurationError)?; //SERIAL_PORT
    serial_config(&mut port)?;

    set_debug_mode(&mut port)?;

    let mut store = Store::init(Vec::new());
    loop {
        let raw = read_line(&mut port)?;
        println!("{}", raw.data);

        let data = match raw {
            r if is_valid_raw(&r) => LaCrosseData::from_raw(&r),
            _ => Err(RfError::NotImplementedError),
        };
        let data = data?;
        println!(
            "id:{}, temperature:{}, humidity:{}",
            data.sensor_id,
            data.temperature.to_string(),
            data.humidity.to_string()
        );

        let dao = data.to_dao();
        if store.is_new_value(&data) {
            store.insert(Box::new(data));
            db_ex.spawn(move |conn: &mut Option<Conn>| db::insert_sensor_data(conn, dao))
        }
    }
}

fn serial_config<T: SerialPort>(port: &mut T) -> Result<()> {
    port.reconfigure(&|settings| {
        settings.set_baud_rate(serial::Baud57600)?;
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_stop_bits(serial::Stop1);
        settings.set_flow_control(serial::FlowNone);
        Ok(())
    })
    .context(ConfigurationError)?;

    port.set_timeout(Duration::from_millis(100000))
        .context(ConfigurationError)?;

    read_line(port)?;
    Ok(())
}

fn read_line<T: SerialPort>(port: &mut T) -> Result<RawFrame> {
    // 10 = line feed
    let mut input: Vec<u8> = Vec::with_capacity(500);
    let mut buf = [0 as u8];
    while buf[0] != 10 {
        port.read(&mut buf).context(ReadError)?;
        input.push(buf[0]);
    }
    let result = RawFrame::from_utf8(input).expect("Found invalid UTF-8");
    Ok(result)
}

fn set_debug_mode<T: SerialPort>(port: &mut T) -> Result<bool> {
    port.write("10;rfdebug=on;\r\n".as_bytes())
        .context(ReadError)?;
    let response = read_line(port)?;
    println!("{}", response.data);
    match response {
        res if res.is_debug() => Ok(true), //"20;01;RFDEBUG=ON;"
        _ => Ok(set_debug_mode(port).unwrap()),
    }
}
