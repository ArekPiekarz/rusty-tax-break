use crate::event::Event;
use crate::event_handling::{EventHandler, onUnknown};
use crate::gui_element_provider::GuiElementProvider;
use crate::repository::Repository;
use crate::source::Source;

use gtk::prelude::LabelExt as _;
use std::rc::Rc;


pub struct RepositoryPathLabel
{
    widget: gtk::Label
}

impl EventHandler for RepositoryPathLabel
{
    fn handle(&mut self, source: Source, event: &Event)
    {
        match event {
            Event::RepositoryChanged(repo) => self.handleRepositoryChanged(repo),
            _ => onUnknown(source, event)
        }
    }
}

impl RepositoryPathLabel
{
    pub fn new(guiElementProvider: &GuiElementProvider) -> Self
    {
        Self{widget: guiElementProvider.get::<gtk::Label>("repositoryPathLabel")}
    }


    // private

    fn handleRepositoryChanged(&self, repo: &Rc<Repository>)
    {
        self.widget.set_text(&repo.getPath().to_string_lossy());
    }
}
