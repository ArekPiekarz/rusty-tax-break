use crate::event_handling::FORWARD_EVENT;
use crate::gui_element_provider::GuiElementProvider;

use gtk::prelude::WidgetExt as _;


pub struct ApplicationWindow
{
    window: gtk::ApplicationWindow
}

impl ApplicationWindow
{
    pub fn new(guiElementProvider: &GuiElementProvider) -> Self
    {
        let window = guiElementProvider.get::<gtk::ApplicationWindow>("mainWindow");
        connectToWindowDeletion(&window);
        Self{window}
    }

    pub fn show(&self)
    {
        self.window.show_all();
    }
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
