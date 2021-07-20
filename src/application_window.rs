use crate::config_store::Config;
use crate::event::Event;
use crate::event_handling::{FORWARD_EVENT, Sender};
use crate::gui_element_provider::GuiElementProvider;
use crate::source::Source;

use gtk::prelude::GtkWindowExt as _;
use gtk::prelude::WidgetExt as _;


pub struct ApplicationWindow
{
    window: gtk::ApplicationWindow
}

impl ApplicationWindow
{
    pub fn new(config: &Config, guiElementProvider: &GuiElementProvider, sender: Sender) -> Self
    {
        let window = guiElementProvider.get::<gtk::ApplicationWindow>("mainWindow");
        if config.isWindowMaximized {
            window.maximize();
        }
        connectToWindowMaximized(&window, sender);
        connectToWindowDeletion(&window);
        Self{window}
    }

    pub fn show(&self)
    {
        self.window.show_all();
    }
}

fn connectToWindowMaximized(window: &gtk::ApplicationWindow, sender: Sender)
{
    window.connect_is_maximized_notify(move |window| {
        sender.send((Source::ApplicationWindow, Event::WindowMaximized(window.is_maximized()))).unwrap();
    });
}

fn connectToWindowDeletion(window: &gtk::ApplicationWindow)
{
    window.connect_delete_event(move |_window, _event| {
        if gtk::main_level() > 0 {
            gtk::main_quit();
        }
        FORWARD_EVENT
    });
}
