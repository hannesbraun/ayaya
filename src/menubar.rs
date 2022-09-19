use crate::{dumpview, mainview, replayview};
use fltk::enums::Shortcut;
use fltk::menu::{MenuFlag, SysMenuBar};
use fltk::prelude::*;

pub fn menu_bar() {
    let mut menu_bar = SysMenuBar::default();
    menu_bar.add(
        "&Tools/Ayaya...\t",
        Shortcut::empty(),
        MenuFlag::Normal,
        move |_| {
            mainview::main_view();
        },
    );
    menu_bar.add(
        "&Tools/Dump...\t",
        Shortcut::Command | 'd',
        MenuFlag::Normal,
        move |_| {
            dumpview::dump_view();
        },
    );
    menu_bar.add(
        "&Tools/Replay...\t",
        Shortcut::Command | 'r',
        MenuFlag::Normal,
        move |_| {
            replayview::replay_view();
        },
    );
}
