use crate::event::Event;
use crate::event_handling::Sender;
use crate::gui_element_provider::GuiElementProvider;
use crate::source::Source;

use gtk::ButtonExt as _;


pub fn setupOpenOptionsButton(guiElementProvider: &GuiElementProvider, sender: Sender)
{
    let button = guiElementProvider.get::<gtk::Button>("openOptionsButton");
    button.connect_clicked(move |_widget|
        sender.send((Source::OpenOptionsButton, Event::OpenOptionsRequested)).unwrap());
}
