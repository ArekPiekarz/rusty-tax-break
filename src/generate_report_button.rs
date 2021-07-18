use crate::event::Event;
use crate::event_handling::Sender;
use crate::gui_element_provider::GuiElementProvider;
use crate::source::Source;

use gtk::prelude::ButtonExt as _;

pub fn setupGenerateReportButton(guiElementProvider: &GuiElementProvider, sender: Sender)
{
    let button = guiElementProvider.get::<gtk::Button>("generateReportButton");
    button.connect_clicked(move |_widget|
        sender.send((Source::GenerateReportButton, Event::GenerateReportRequested)).unwrap());
}
