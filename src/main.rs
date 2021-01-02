#![allow(non_snake_case)]

use rusty_tax_break::gui::Gui;


fn main()
{
    let gui = Gui::new();
    gui.show();
    gui.run();
}
