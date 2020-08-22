use std::time::Duration;

use crate::errors::*;
use snafu::ResultExt;

use bytes::{BufMut, BytesMut};
use futures::{sink::SinkExt, stream::StreamExt};
use std::sync::{Arc, Mutex};
use std::{env, io, str};
use tokio_util::codec::{Decoder, Encoder};

#[cfg(unix)]
const DEFAULT_TTY: &str = "/dev/ttyACM0";
#[cfg(windows)]
const DEFAULT_TTY: &str = "COM1";

struct LineCodec;

impl Decoder for LineCodec {
    type Item = String;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let newline = src.as_ref().iter().position(|b| *b == b'\n');
        if let Some(n) = newline {
            let line = src.split_to(n + 1);
            return match str::from_utf8(line.as_ref()) {
                Ok(s) => Ok(Some(s.to_string())),
                Err(_) => Err(io::Error::new(io::ErrorKind::Other, "Invalid String")),
            };
        }
        Ok(None)
    }
}

impl Encoder<String> for LineCodec {
    type Error = io::Error;

    fn encode(&mut self, item: String, dst: &mut BytesMut) -> Result<(), Self::Error> {
        println!("In writer {:?}", &item);
        dst.reserve(item.len() + 1);
        dst.put(item.as_bytes());
        dst.put_u8(b'\n');
        Ok(())
    }
}

pub trait Observer {
    fn on_receive_data(&self, data: String);
}

pub struct RfReceiver {
    obs: Vec<Arc<Mutex<dyn Observer + Send + Sync>>>,
}
unsafe impl Send for RfReceiver {}
unsafe impl Sync for RfReceiver {}

impl RfReceiver {
    pub fn new(observers: Vec<Arc<Mutex<dyn Observer + Send + Sync>>>) -> RfReceiver {
        RfReceiver { obs: observers }
    }

    pub fn handle(&self, data: String) {
        for observer in &self.obs.clone() {
            observer.lock().unwrap().on_receive_data(data.clone());
        }
    }

    pub fn start_listening(handle: &'static (dyn Fn(String) +Send+Sync)) {
        println!("Start listening");
            let result = RfReceiver::listen(handle);
            match result {
                Ok(_) => (),
                Err(e) => unimplemented!(),
            };
            println!("end listening");
    }

    fn listen(handle: &'static (dyn Fn(String) +Send+Sync)) -> Result<()> {
        let mut args = env::args();
        let tty_path = args.nth(1).unwrap_or_else(|| DEFAULT_TTY.into());

        let mut settings = tokio_serial::SerialPortSettings::default();
        settings.baud_rate = 57600;
        settings.data_bits = tokio_serial::DataBits::Eight;
        settings.flow_control = tokio_serial::FlowControl::None;
        settings.parity = tokio_serial::Parity::None;
        settings.stop_bits = tokio_serial::StopBits::One;
        let mut port = tokio_serial::Serial::from_path(tty_path, &settings).unwrap();

        #[cfg(unix)]
        port.set_exclusive(false)
            .expect("Unable to set serial port exclusive to false");

        tokio::spawn(async move {
            let mut io = LineCodec.framed(port);
            let a = io.next().await;
            let data = a.unwrap().expect("Failed to read line");
            println!("{}", data);
            io.send("10;rfdebug=on;\r\n".to_string()).await;

            while let Some(line_result) = io.next().await {
                let line = line_result.expect("Failed to read line");
                handle(line);
            }
        });

        Ok(())
    }
}
