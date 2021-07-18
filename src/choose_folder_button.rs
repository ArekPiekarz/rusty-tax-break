use crate::event::Event;
use crate::event_handling::{EventHandler, onUnknown, Sender};
use crate::gui_element_provider::GuiElementProvider;
use crate::source::Source;

use gtk::prelude::ButtonExt as _;
use gtk::prelude::DialogExt as _;
use gtk::prelude::FileChooserExt as _;
use gtk::prelude::GtkWindowExt as _;
use gtk::prelude::WidgetExt as _;


pub struct ChooseFolderButton
{
    dialogTitle: &'static str,
    parentWindow: gtk::ApplicationWindow,
    source: Source,
    dialogSource: Source,
    sender: Sender,
    dialog: Option<gtk::FileChooserDialog>
}

impl EventHandler for ChooseFolderButton
{
    fn handle(&mut self, source: Source, event: &Event)
    {
        match event {
            Event::Clicked => self.onClicked(),
            Event::DialogResponded(response) => self.onDialogResponded(*response),
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
        dialogSource: Source,
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
            dialogSource,
            sender,
            dialog: None
        }
    }


    // private

    fn onClicked(&mut self)
    {
        let dialog = gtk::FileChooserDialog::with_buttons(
            Some(self.dialogTitle),
            Some(&self.parentWindow),
            gtk::FileChooserAction::SelectFolder,
            &[("Cancel", gtk::ResponseType::Cancel), ("Choose", gtk::ResponseType::Accept)]
        );
        let sender = self.sender.clone();
        let source = self.dialogSource;
        dialog.connect_response(move |_dialog, response| {
            sender.send((source, Event::DialogResponded(response))).unwrap();
        });
        dialog.set_modal(true);
        dialog.show();
        self.dialog = Some(dialog);
    }

    fn onDialogResponded(&mut self, response: gtk::ResponseType)
    {
        match response {
            gtk::ResponseType::Accept      => self.onDialogAccepted(),
            gtk::ResponseType::Cancel      => self.onDialogCancelled(),
            gtk::ResponseType::DeleteEvent => self.onDialogDeleted(),
            _ => self.onUnknownDialogResponse(response)
        }
    }

    fn onDialogAccepted(&mut self)
    {
        let dialog = match &self.dialog {
            None => {
                eprintln!("Expected dialog to exist, but it didn't");
                return;
            },
            Some(dialog) => dialog
        };

        if let Some(folder) = dialog.filename() {
            self.sender.send((self.source, Event::FolderChosen(folder))).unwrap();
        }
        dialog.close();
    }

    fn onDialogCancelled(&mut self)
    {
        match &self.dialog {
            Some(dialog) => {
                dialog.close();
                self.dialog = None;
            },
            None => eprintln!("Expected dialog to exist, but it didn't")
        }
    }

    fn onDialogDeleted(&mut self)
    {
        self.dialog = None;
    }

    fn onUnknownDialogResponse(&mut self, response: gtk::ResponseType)
    {
        eprintln!("Received unknown dialog response: {:?}", response);
        if let Some(dialog) = &self.dialog {
            dialog.close();
            self.dialog = None;
        }
    }
}

fn connectWidget(source: Source, widget: &gtk::Button, sender: Sender)
{
    widget.connect_clicked(move |_button| sender.send((source, Event::Clicked)).unwrap());
}
