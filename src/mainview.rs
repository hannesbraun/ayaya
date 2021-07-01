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

        let mut host = Input::new(45, 11, 115, 24, "Host:");
        win.add(&host);
        let mut port = Input::new(205, 11, 55, 24, "Port:");
        win.add(&port);
        let mut protocol = Choice::new(325, 10, 65, 25, "Protocol:");
        protocol.end();
        protocol.add_choice("TCP");
        protocol.add_choice("UDP");
        protocol.set_item(&protocol.at(1).unwrap());
        protocol.set_down_frame(FrameType::BorderBox);
        win.add(&protocol);
        let mut addr = InputChoice::new(100, 61, 290, 24, "OSC Address:");
        win.add(&addr);
        let mut value = Input::new(100, 91, 290, 24, "Value:");
        win.add(&value);
        let mut osc_type = Choice::new(100, 121, 290, 24, "Type:");
        win.add(&osc_type);
        let mut send = ReturnButton::new(325, 265, 65, 25, "Send");
        win.add(&send);

        Self { win, host, port, protocol, addr, value, osc_type, send }
    }
}
