use crate::event::Event;
use crate::event_handling::Sender;
use crate::gui_element_provider::GuiElementProvider;
use crate::source::Source;

use gtk::EditableSignals as _;
use gtk::prelude::EntryExt as _;
use gtk::prelude::SpinButtonExt as _;
use time::Date;


pub fn setupYearFilterSpinButton(date: &Date, guiElementProvider: &GuiElementProvider, sender: Sender)
{
    let yearFilterSpinButton = guiElementProvider.get::<gtk::SpinButton>("yearFilterSpinButton");
    let sender2 = sender.clone();

    yearFilterSpinButton.connect_changed(move |widget| {
        sender.send((Source::YearSpinButton, Event::YearFilterChanged(widget.value_as_int()))).unwrap();
    });
    yearFilterSpinButton.connect_activate(move |widget| {
        sender2.send((Source::YearSpinButton, Event::YearFilterChanged(widget.value_as_int()))).unwrap();
    });

    yearFilterSpinButton.set_value(date.year().into());
}
