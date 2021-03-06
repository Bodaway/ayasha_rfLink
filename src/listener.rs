use crate::errors::*;
use crate::domain::command_event::Command;
use crate::state_actor::MessageSender;

use bytes::{BufMut, BytesMut};
use futures::{sink::SinkExt, stream::StreamExt};
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

pub fn start_listening(messager: MessageSender) {
    println!("Start listening");
    let result = listen(messager.clone());
    match result {
        Ok(_) => (),
        Err(e) => {
            println!("error during read: {}", e);
            start_listening(messager);
        }
    };
}

fn listen(messager: MessageSender) -> Result<()> {

    let mut settings = tokio_serial::SerialPortSettings::default();
    settings.baud_rate = 57600;
    settings.data_bits = tokio_serial::DataBits::Eight;
    settings.flow_control = tokio_serial::FlowControl::None;
    settings.parity = tokio_serial::Parity::None;
    settings.stop_bits = tokio_serial::StopBits::One;
    let mut port = tokio_serial::Serial::from_path(DEFAULT_TTY, &settings).unwrap();

    #[cfg(unix)]
    port.set_exclusive(false)
        .expect("Unable to set serial port exclusive to false");

    let (sender, receiver) = std::sync::mpsc::channel::<bool>();

    tokio::spawn(async move {
        let mut io = LineCodec.framed(port);
        let a = io.next().await;
        let data = a.unwrap().expect("Failed to read line");
        println!("{}", data);

        io.send("10;rfdebug=on;\r\n".to_string())
            .await
            .expect("rf link comm error");
        let response_result = io.next().await;
        let is_debug = match response_result {
            None => {
                println!("debug is None");
                false
            }
            Some(result) => match result {
                Ok(data) => data.contains("RFDEBUG=ON"),
                Err(e) => {
                    println!("debug engage error: {}", e);
                    false
                }
            },
        };

        if is_debug {
            sender.send(true).expect("inter task communication error");
            while let Some(line_result) = io.next().await {
                let line = line_result.expect("Failed to read line");
                println!("{}", line);
                messager.send(Command::IncomingData(line));
            }
        }
        sender.send(false).expect("inter task communication error");
    });

    match receiver.recv() {
        Ok(engage) => match engage {
            true => Ok(()),
            false => Err(RfError::DebugNotEngage),
        },
        Err(_) => Err(RfError::DebugNotEngage),
    }
}
