use fltk::{prelude::*, *};
use rosc::{OscPacket,OscMessage,OscType};
use rosc::encoder;

mod mainview;

fn main() {
    let packet = OscPacket::Message(
        OscMessage {
            addr: String::from("/ayaya/slider1"),
            args: vec![OscType::Float(0.5)],
        }
    );
    encoder::encode(&packet);

    let app = app::App::default();
    let mut ui = mainview::UserInterface::make_window();
    ui.send.set_callback(|_| println!("hi"));
    app.run().unwrap();
}
