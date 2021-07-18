use crate::date_time::LocalDate;
use crate::event::Event;
use crate::event_handling::{EventHandler, onUnknown};
use crate::gui_element_provider::GuiElementProvider;
use crate::source::Source;

use chrono::Datelike as _;
use gtk::prelude::LabelExt as _;
use std::path::Path;


pub struct OutputPathLabel
{
    widget: gtk::Label
}

impl EventHandler for OutputPathLabel
{
    fn handle(&mut self, source: Source, event: &Event)
    {
        match event {
            Event::OutputPathChanged(path) => self.onPathChanged(path),
            Event::PartialOutputPathChanged(path) => self.onPartialPathChanged(path),
            _ => onUnknown(source, event)
        }
    }
}

impl OutputPathLabel
{
    pub fn new(date: LocalDate, guiElementProvider: &GuiElementProvider) -> Self
    {
        let widget = guiElementProvider.get::<gtk::Label>("outputPathLabel");
        widget.set_text(&format!("<path>/{}/{:02}", date.year(), date.month()));
        Self{widget}
    }


    // private

    fn onPathChanged(&self, path: &Path)
    {
        self.widget.set_text(&path.to_string_lossy());
    }

    fn onPartialPathChanged(&self, path: &Path)
    {
        self.widget.set_text(&format!("<path>/{}", path.to_string_lossy()));
    }
}
