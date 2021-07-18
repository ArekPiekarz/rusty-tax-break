use crate::event::Event;
use crate::event_handling::Sender;
use crate::gui_element_provider::GuiElementProvider;
use crate::source::Source;

use gtk::prelude::EntryExt as _;

pub fn setupCommitAuthorFilterEntry(guiElementProvider: &GuiElementProvider, sender: Sender)
{
    let widget = guiElementProvider.get::<gtk::Entry>("commitAuthorFilterEntry");
    widget.connect_activate(move |widget| {
        sender.send((Source::CommitAuthorFilterEntry, Event::CommitAuthorFilterChanged(widget.text().into()))).unwrap();
    });
}
