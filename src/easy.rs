use fltk::dialog;
use rosc::{encoder, OscMessage, OscPacket, OscType};
use std::io::Write;
use std::net::{TcpStream, UdpSocket};
use crate::util::TransportProtocol;

pub fn send(msg: &RawOscMessage) {
    let packet = msg.to_packet();
    let packet = if let Some(packet) = packet {
        packet
    } else {
        return;
    };

    let data = match encoder::encode(&packet) {
        Ok(data) => data,
        Err(_) => {
            dialog::alert_default("Unable to encode the OSC message.");
            return;
        }
    };

    let addr = format!("{}:{}", msg.host, msg.port);
    match msg.protocol {
        TransportProtocol::Tcp => {
            let socket = TcpStream::connect(addr);
            let mut socket = if let Ok(socket) = socket {
                socket
            } else {
                dialog::alert_default(&format!("Unable to connect to {}:{}.", msg.host, msg.port));
                return;
            };
            let res = socket.write(&data);
            if res.is_err() {
                dialog::alert_default(&res.err().unwrap().to_string());
            }
        }
        TransportProtocol::Udp => {
            let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
            let res = socket.send_to(&data, addr);
            if res.is_err() {
                dialog::alert_default(&res.err().unwrap().to_string());
            }
        }
    }
}

pub enum OscTypeTag {
    Int32,
    Float32,
    OscString,
}

impl OscTypeTag {
    pub fn from(val: &str) -> OscTypeTag {
        match val {
            "int32" => OscTypeTag::Int32,
            "float32" => OscTypeTag::Float32,
            "OSC-string" => OscTypeTag::OscString,
            _ => OscTypeTag::Int32,
        }
    }
}

pub struct RawOscMessage {
    pub host: String,
    pub port: u16,
    pub protocol: TransportProtocol,
    pub osc_address: String,
    pub value: String,
    pub osc_type: OscTypeTag,
}

impl RawOscMessage {
    fn to_packet(&self) -> Option<OscPacket> {
        if !self.value.is_empty() {
            let value = match self.osc_type {
                OscTypeTag::Int32 => match self.value.parse() {
                    Ok(res) => OscType::Int(res),
                    Err(_) => {
                        dialog::alert_default("This value is not a 32-bit signed integer.");
                        return None;
                    }
                },
                OscTypeTag::Float32 => match self.value.parse() {
                    Ok(res) => OscType::Float(res),
                    Err(_) => {
                        dialog::alert_default("This value is not a 32-bit floating point number.");
                        return None;
                    }
                },
                OscTypeTag::OscString => OscType::String(self.value.clone()),
            };

            Some(OscPacket::Message(OscMessage {
                addr: self.osc_address.clone(),
                args: vec![value],
            }))
        } else {
            // No value present
            Some(OscPacket::Message(OscMessage {
                addr: self.osc_address.clone(),
                args: Vec::new(),
            }))
        }
    }
}
