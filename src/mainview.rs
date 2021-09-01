use fltk::enums::FrameType;
use fltk::window::Window;
use fltk::input::Input;
use fltk::misc::InputChoice;
use fltk::menu::Choice;
use fltk::button::ReturnButton;
use fltk::prelude::*;
use fltk::image::PngImage;
use std::path::Path;

pub struct UserInterface {
    pub win: Window,
    pub host: Input,
    pub port: Input,
    pub protocol: Choice,
    pub addr: InputChoice,
    pub value: Input,
    pub osc_type: Choice,
    pub send: ReturnButton,
}


impl UserInterface {
    pub fn make_window() -> Self {
        let mut win = Window::new(2996, 871, 400, 300, "Main View");
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
        osc_type.add_choice("OSC-String");
        osc_type.set_value(0);
        osc_type.set_down_frame(FrameType::BorderBox);
        win.add(&osc_type);
        let send = ReturnButton::new(325, 265, 65, 25, "Send");
        win.add(&send);

        Self { win, host, port, protocol, addr, value, osc_type, send }
    }
}
