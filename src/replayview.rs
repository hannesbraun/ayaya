use crate::menubar::menu_bar;
use fltk::app;
use fltk::button::{Button, ReturnButton};
use fltk::enums::FrameType;
use fltk::input::Input;
use fltk::menu::Choice;
use fltk::output::MultilineOutput;
use fltk::prelude::*;
use fltk::window::Window;

pub struct UserInterface {
    win: Window,
    host: Input,
    port: Input,
    protocol: Choice,
    file_name: Input,
    choose_file: Button,
    output: MultilineOutput,
    send: ReturnButton,
}

impl UserInterface {
    fn make_window() -> Self {
        let screen_dimensions = app::screen_xywh(0);
        let width = 400;
        let height = 305;
        let mut win = Window::new(
            (screen_dimensions.2 - width) / 2,
            (screen_dimensions.3 - height) / 2,
            width,
            height,
            "OSC Replay",
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
        let file_name = Input::new(77, 41, 240, 24, "File name:");
        win.add(&file_name);
        let choose_file = Button::new(320, 41, 70, 25, "Browse...");
        win.add(&choose_file);
        let output = MultilineOutput::new(10, 71, 380, 190, "");
        win.add(&output);
        let send = ReturnButton::new(325, 270, 65, 25, "Send");
        win.add(&send);

        Self {
            win,
            host,
            port,
            protocol,
            file_name,
            choose_file,
            output,
            send,
        }
    }
}

pub fn replay_view() {
    let mut ui = UserInterface::make_window();

    std::thread::spawn(move || {
        for i in 1..20 {
            ui.output.append(format!("{}\n", i).as_str());
            app::awake();
            std::thread::sleep(std::time::Duration::from_millis(1000));
        }
    });
}
