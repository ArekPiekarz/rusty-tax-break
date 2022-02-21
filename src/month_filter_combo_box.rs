use crate::event::Event;
use crate::event_handling::Sender;
use crate::gui_element_provider::GuiElementProvider;
use crate::source::Source;

use gtk::prelude::ComboBoxExt as _;
use time::{Date, Month};
use to_trait::To as _;


pub fn setupMonthFilterComboBox(date: &Date, guiElementProvider: &GuiElementProvider, sender: Sender)
{
    let monthFilterComboBox = guiElementProvider.get::<gtk::ComboBox>("monthFilterComboBox");
    monthFilterComboBox.connect_changed(move |widget| {
        let month = Month::try_from(widget.active_id().unwrap().as_str().parse::<u8>().unwrap()).unwrap();
        sender.send((Source::MonthComboBox, Event::MonthFilterChanged(month))).unwrap();
    });
    monthFilterComboBox.set_active_id(Some(&date.month().to::<u8>().to_string()));
}
