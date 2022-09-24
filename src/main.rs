use easy::{OscTypeTag, RawOscMessage};
use fltk::menu::mac_set_about;
use fltk::*;
use util::TransportProtocol;

mod dumpview;
mod easy;
mod mainview;
mod menubar;
mod replayview;
mod util;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn copyright_header() -> String {
    format!(
        "Ayaya {}
Copyright (c) 2021-2022, Hannes Braun
Ayaya is licensed under the Boost Software License 1.0",
        VERSION
    )
}

fn main() {
    println!("{}", copyright_header());

    let app = mainview::main_view();

    mac_set_about(move || {
        dialog::message_title("About Ayaya");
        dialog::message_default(&copyright_header());
    });

    app.run().unwrap();
}
