use crate::date_time::LocalDate;
use crate::event::Event;
use crate::event_handling::Sender;
use crate::gui_element_provider::GuiElementProvider;
use crate::source::Source;

use chrono::Datelike as _;
use gtk::ComboBoxExt as _;
use num_traits::cast::FromPrimitive as _;


pub fn setupMonthFilterComboBox(date: &LocalDate, guiElementProvider: &GuiElementProvider, sender: Sender)
{
    let monthFilterComboBox = guiElementProvider.get::<gtk::ComboBox>("monthFilterComboBox");
    monthFilterComboBox.connect_changed(move |widget| {
        let month = chrono::Month::from_u32(widget.get_active_id().unwrap().as_str().parse().unwrap()).unwrap();
        sender.send((Source::MonthComboBox, Event::MonthFilterChanged(month))).unwrap();
    });
    monthFilterComboBox.set_active_id(Some(&date.month().to_string()));
}
