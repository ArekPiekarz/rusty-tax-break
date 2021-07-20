#![allow(non_snake_case)]

use rusty_tax_break::config_path::ConfigPath;
use rusty_tax_break::gui::Gui;

use gtk::glib;

fn main()
{
    let context = glib::MainContext::default();
    let _guard = context.acquire().unwrap();
    let gui = Gui::new(&ConfigPath::default());
    gui.show();
    gui.run();
}
