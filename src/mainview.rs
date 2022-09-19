use crate::menubar::menu_bar;
use crate::{easy, OscTypeTag, RawOscMessage, TransportProtocol};
use fltk::app;
use fltk::button::ReturnButton;
use fltk::enums::FrameType;
use fltk::input::Input;
use fltk::menu::Choice;
use fltk::misc::InputChoice;
use fltk::window::Window;
use fltk::{prelude::*, *};
use std::cell::RefCell;
use std::rc::Rc;

struct UserInterface {
    host: Input,
    port: Input,
    protocol: Choice,
    addr: InputChoice,
    value: Input,
    osc_type: Choice,
    send: ReturnButton,
}

impl UserInterface {
    fn make_window() -> Self {
        let screen_dimensions = app::screen_xywh(0);
        let mut win = Window::new(
            (screen_dimensions.2 - 400) / 2,
            (screen_dimensions.3 - 205) / 2,
            400,
            205,
            "Ayaya",
        );
        menu_bar();
        win.end();
        win.show();

        let host = Input::new(45, 11, 115, 24, "Host:");
        win.add(&host);
        let port = Input::new(205, 11, 55, 24, "Port:");
        win.add(&port);
        let mut protocol = Choice::new(325, 10, 65, 25, "Protocol:");
        protocol.end();
        protocol.add_choice("TCP");
        protocol.add_choice("UDP");
        protocol.set_value(1);
        protocol.set_down_frame(FrameType::BorderBox);
        win.add(&protocol);
        let addr = InputChoice::new(100, 61, 290, 24, "OSC Address:");
        win.add(&addr);
        let value = Input::new(100, 91, 290, 24, "Value:");
        win.add(&value);
        let mut osc_type = Choice::new(100, 121, 290, 24, "Type:");
        osc_type.end();
        osc_type.add_choice("int32");
        osc_type.add_choice("float32");
        osc_type.add_choice("OSC-string");
        osc_type.set_value(0);
        osc_type.set_down_frame(FrameType::BorderBox);
        win.add(&osc_type);
        let send = ReturnButton::new(325, 170, 65, 25, "Send");
        win.add(&send);

        Self {
            host,
            port,
            protocol,
            addr,
            value,
            osc_type,
            send,
        }
    }
}

pub fn main_view() -> app::App {
    let ui_data = Rc::new(RefCell::new(RawOscMessage {
        host: "127.0.0.1".to_string(),
        port: 7321,
        protocol: TransportProtocol::Udp,
        osc_address: "/ayaya/slider1".to_string(),
        value: "".to_string(),
        osc_type: OscTypeTag::Int32,
    }));

    let app = app::App::default();
    let mut ui = UserInterface::make_window();

    {
        let ui_data = Rc::clone(&ui_data);
        ui.host
            .set_callback(move |host| (*ui_data.borrow_mut()).host = host.value());
    }
    ui.host.set_value(&ui_data.borrow().host);

    {
        let ui_data = Rc::clone(&ui_data);
        ui.port.set_callback(move |port| {
            (*ui_data.borrow_mut()).port = port.value().parse::<u16>().unwrap_or_else(|_| {
                dialog::alert_default("This port number is invalid.");
                port.set_value(&(*ui_data.borrow_mut()).port.to_string());
                (*ui_data.borrow_mut()).port
            })
        });
    }
    ui.port.set_value(&ui_data.borrow().port.to_string());

    {
        let ui_data = Rc::clone(&ui_data);
        ui.protocol.set_callback(move |protocol| {
            (*ui_data.borrow_mut()).protocol = TransportProtocol::from(&protocol.choice().unwrap())
        });
    }
    match &ui_data.borrow().protocol {
        TransportProtocol::Tcp => ui.protocol.set_value(0),
        TransportProtocol::Udp => ui.protocol.set_value(1),
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
                    (*ui_data.borrow_mut()).osc_address = addr.value().unwrap_or_default()
                }
            }
        });
    }
    ui.addr.set_value(&ui_data.borrow().osc_address);

    {
        let ui_data = Rc::clone(&ui_data);
        ui.value
            .set_callback(move |value| (*ui_data.borrow_mut()).value = value.value());
    }
    ui.value.set_value(&ui_data.borrow().value);

    {
        let ui_data = Rc::clone(&ui_data);
        ui.osc_type.set_callback(move |osc_type| {
            (*ui_data.borrow_mut()).osc_type = OscTypeTag::from(&osc_type.choice().unwrap())
        });
    }
    match &ui_data.borrow().osc_type {
        OscTypeTag::Int32 => ui.osc_type.set_value(0),
        OscTypeTag::Float32 => ui.osc_type.set_value(1),
        OscTypeTag::OscString => ui.osc_type.set_value(2),
    };

    {
        let ui_data = Rc::clone(&ui_data);
        ui.send
            .set_callback(move |_| easy::send(&*ui_data.borrow()));
    }

    app
}
