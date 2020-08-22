use crate::errors::*;

use bytes::{BufMut, BytesMut};
use futures::future::{BoxFuture, FutureExt};
use futures::{sink::SinkExt, stream::StreamExt};
use snafu::ResultExt;
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

pub fn start_listening(handle: &'static (dyn Fn(String) + Send + Sync)) {
    println!("Start listening");
    let result = listen(handle);
    match result {
        Ok(_) => (),
        Err(e) => {
            println!("error during read: {}", e);
            start_listening(handle);}
    };
    println!("end listening");
}

fn listen(handle: &'static (dyn Fn(String) + Send + Sync)) -> Result<()> {
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

    let (sender, receiver) = std::sync::mpsc::channel::<bool>();

    tokio::spawn(async move {
        let mut io = LineCodec.framed(port);
        let a = io.next().await;
        let data = a.unwrap().expect("Failed to read line");
        println!("{}", data);

        io.send("10;rfdebug=on;\r\n".to_string()).await;
        let responseResult = io.next().await;
        let isDebug = match responseResult {
            None => false,
            Some(result) => result.map_or_else(
                |s| {
                    println!("debug engage {}", &s);
                    true
                },
                |_| false,
            ),
        };

        if isDebug {
            sender.send(true);
            while let Some(line_result) = io.next().await {
                let line = line_result.expect("Failed to read line");
                handle(line);
            }
        }
        sender.send(false);
    });

    match receiver.recv() {
        Ok(engage) => match engage {
            true => Ok(()),
            false => Err(RfError::DebugNotEngage)
        },
        Err(e) => Err(RfError::DebugNotEngage)
    }
}
