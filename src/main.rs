use fltk::{prelude::*, *};
use rosc::{OscPacket, OscMessage, OscType};
use rosc::encoder;
use crate::mainview::UserInterface;
use std::rc::Rc;
use std::cell::RefCell;
use std::any::Any;

mod mainview;

fn main() {
    let packet = OscPacket::Message(
        OscMessage {
            addr: String::from("/ayaya/slider1"),
            args: vec![OscType::Float(0.5)],
        }
    );
    encoder::encode(&packet);

    let mut ui_data = Rc::new(RefCell::new(
        RawOscMessage {
            host: "127.0.0.1".to_string(),
            port: 7321,
            protocol: TransportProtocol::UDP,
            osc_address: "/ayaya/slider1".to_string(),
            value: "".to_string(),
            osc_type: OscTypeTag::Int32,
        }
    ));

    let app = app::App::default();
    let mut ui = mainview::UserInterface::make_window();

    {
        let ui_data = Rc::clone(&ui_data);
        ui.host.set_callback(move |host| (*ui_data.borrow_mut()).host = host.value());
    }
    ui.host.set_value(&ui_data.borrow().host);

    {
        let ui_data = Rc::clone(&ui_data);
        ui.port.set_callback(move |port| (*ui_data.borrow_mut()).port = port.value().parse::<u16>().unwrap_or_else(|_| {
            dialog::alert_default("This port number is invalid.");
            port.set_value(&(*ui_data.borrow_mut()).port.to_string());
            (*ui_data.borrow_mut()).port
        }));
    }
    ui.port.set_value(&ui_data.borrow().port.to_string());

    {
        let ui_data = Rc::clone(&ui_data);
        ui.protocol.set_callback(move |protocol| (*ui_data.borrow_mut()).protocol = TransportProtocol::from(&protocol.choice().unwrap()));
    }
    match &ui_data.borrow().protocol {
        TransportProtocol::TCP => ui.protocol.set_value(0),
        TransportProtocol::UDP => ui.protocol.set_value(1),
    };
    
    {
        let ui_data = Rc::clone(&ui_data);
        ui.addr.set_callback(move |addr| {
            let val = addr.value();
            match val {
                Some(addr_val) => {
                    addr.add(&addr_val);
                    (*ui_data.borrow_mut()).osc_address = addr_val;
                }
                None => {
                    addr.set_value_index(0);
                    (*ui_data.borrow_mut()).osc_address = addr.value().unwrap_or("".to_string())
                }
            }
        });
    }
    ui.addr.set_value(&ui_data.borrow().osc_address);

    {
        let ui_data = Rc::clone(&ui_data);
        ui.value.set_callback(move |value| (*ui_data.borrow_mut()).value = value.value());
    }
    ui.value.set_value(&ui_data.borrow().value);

    {
        let ui_data = Rc::clone(&ui_data);
        ui.protocol.set_callback(move |osc_type| (*ui_data.borrow_mut()).osc_type = OscTypeTag::from(&osc_type.choice().unwrap()));
    }
    match &ui_data.borrow().osc_type {
        OscTypeTag::Int32 => ui.osc_type.set_value(0),
        OscTypeTag::Float32 => ui.osc_type.set_value(1),
        OscTypeTag::OscString => ui.osc_type.set_value(2),
    };

    {
        let ui_data = Rc::clone(&ui_data);
        ui.send.set_callback(move |_| send(&*ui_data.borrow()));
    }
    
    app.run().unwrap();
}

pub fn send(ui: &RawOscMessage) {

}

pub enum TransportProtocol {
    TCP = 0,
    UDP = 1,
}

impl TransportProtocol {
    pub fn from(val: &str) -> TransportProtocol {
        match val {
            "TCP" => TransportProtocol::TCP,
            "UDP" => TransportProtocol::UDP,
            _ => TransportProtocol::UDP
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
            _ => OscTypeTag::Int32
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
