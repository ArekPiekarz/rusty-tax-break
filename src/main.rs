#![allow(non_snake_case)]

use gtk::glib;
use rusty_tax_break::gui::Gui;


fn main()
{
    let context = glib::MainContext::default();
    let _guard = context.acquire().unwrap();
    let gui = Gui::new();
    gui.show();
    gui.run();
}
