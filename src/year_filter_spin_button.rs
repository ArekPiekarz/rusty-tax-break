use crate::date_time::LocalDate;
use crate::event::Event;
use crate::event_handling::Sender;
use crate::gui_element_provider::GuiElementProvider;
use crate::source::Source;

use chrono::Datelike as _;
use gtk::EditableSignals as _;
use gtk::EntryExt as _;
use gtk::SpinButtonExt as _;


pub fn setupYearFilterSpinButton(date: &LocalDate, guiElementProvider: &GuiElementProvider, sender: Sender)
{
    let yearFilterSpinButton = guiElementProvider.get::<gtk::SpinButton>("yearFilterSpinButton");
    let sender2 = sender.clone();

    yearFilterSpinButton.connect_changed(move |widget| {
        sender.send((Source::YearSpinButton, Event::YearFilterChanged(widget.get_value_as_int()))).unwrap();
    });
    yearFilterSpinButton.connect_activate(move |widget| {
        sender2.send((Source::YearSpinButton, Event::YearFilterChanged(widget.get_value_as_int()))).unwrap();
    });

    yearFilterSpinButton.set_value(date.year().into());
}
