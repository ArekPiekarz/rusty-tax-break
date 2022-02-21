use crate::config_store::Config;
use crate::event::{Event, OutputPathInfo};
use crate::event_handling::{EventHandler, onUnknown};
use crate::gui_element_provider::GuiElementProvider;
use crate::source::Source;

use gtk::prelude::LabelExt as _;
use std::path::Path;
use time::Date;


pub struct OutputPathLabel
{
    widget: gtk::Label
}

impl EventHandler for OutputPathLabel
{
    fn handle(&mut self, source: Source, event: &Event)
    {
        match event {
            Event::OutputPathChanged(pathInfo)    => self.onPathChanged(pathInfo),
            Event::PartialOutputPathChanged(path) => self.onPartialPathChanged(path),
            _ => onUnknown(source, event)
        }
    }
}

impl OutputPathLabel
{
    pub fn new(config: &Config, date: Date, guiElementProvider: &GuiElementProvider) -> Self
    {
        let widget = guiElementProvider.get::<gtk::Label>("outputPathLabel");
        let prefixPath = match &config.outputPathPrefix {
            Some(path) => path.to_string_lossy(),
            None => "<path>".into()
        };
        widget.set_text(&format!("{}/{}/{:02}", prefixPath, date.year(), date.month()));
        Self{widget}
    }


    // private

    fn onPathChanged(&self, pathInfo: &OutputPathInfo)
    {
        self.widget.set_text(&pathInfo.full.to_string_lossy());
    }

    fn onPartialPathChanged(&self, path: &Path)
    {
        self.widget.set_text(&format!("<path>/{}", path.to_string_lossy()));
    }
}
