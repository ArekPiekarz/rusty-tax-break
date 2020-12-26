use crate::event::Event;
use crate::event_handling::{EventHandler, onUnknown, Sender};
use crate::gui_element_provider::GuiElementProvider;
use crate::source::Source;

use gtk::ButtonExt as _;
use gtk::FileChooserExt as _;
use gtk::NativeDialogExt as _;


pub struct ChooseFolderButton
{
    dialogTitle: &'static str,
    parentWindow: gtk::ApplicationWindow,
    source: Source,
    sender: Sender
}

impl EventHandler for ChooseFolderButton
{
    fn handle(&mut self, source: Source, event: &Event)
    {
        match event {
            Event::Clicked => self.onClicked(),
            _ => onUnknown(source, event)
        }
    }
}

impl ChooseFolderButton
{
    pub fn new(
        name: &str,
        dialogTitle: &'static str,
        source: Source,
        widgetSource: Source,
        guiElementProvider: &GuiElementProvider,
        sender: Sender)
        -> Self
    {
        let widget = guiElementProvider.get::<gtk::Button>(name);
        connectWidget(widgetSource, &widget, sender.clone());
        Self{
            parentWindow: guiElementProvider.get::<gtk::ApplicationWindow>("mainWindow"),
            dialogTitle,
            source,
            sender
        }
    }


    // private

    fn onClicked(&self)
    {
        let dialog = gtk::FileChooserNative::new(
            Some(self.dialogTitle),
            Some(&self.parentWindow),
            gtk::FileChooserAction::SelectFolder,
            Some("Choose"),
            Some("Cancel")
        );

        if dialog.run() == gtk::ResponseType::Accept {
            self.handleSelectedFolder(dialog.get_uri());
        }
    }

    fn handleSelectedFolder(&self, folderUriOpt: Option<glib::GString>)
    {
        if let Some(folderUri) = folderUriOpt {
            self.sender.send((self.source, Event::FolderChosen(folderUri.into()))).unwrap();
        }
    }
}

fn connectWidget(source: Source, widget: &gtk::Button, sender: Sender)
{
    widget.connect_clicked(move |_button| sender.send((source, Event::Clicked)).unwrap());
}
