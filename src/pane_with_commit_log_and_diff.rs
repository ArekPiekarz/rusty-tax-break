use crate::config_store::Config;
use crate::event::Event;
use crate::event_handling::Sender;
use crate::gui_element_provider::GuiElementProvider;
use crate::source::Source;

use gtk::prelude::PanedExt as _;


pub(crate) type PanePosition = i32;

pub(crate) fn setupPaneWithCommitLogAndCommitDiff(config: &Config, guiElementProvider: &GuiElementProvider, sender: Sender)
{
    let pane = guiElementProvider.get::<gtk::Paned>("paneWithCommitLogAndDiff");
    pane.set_position(config.positionOfPaneWithCommitLogAndDiff);
    pane.connect_position_notify(move |widget| {
        sender.send((Source::PaneWithCommitLogAndDiff, Event::PanePositionChanged(widget.position()))).unwrap();
    });
}
