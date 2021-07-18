use crate::event::Event;
use crate::event_handling::{EventHandler, onUnknown, Sender};
use crate::gui_element_provider::GuiElementProvider;
use crate::source::Source;

use gtk::prelude::ButtonExt as _;
use gtk::prelude::DialogExt as _;
use gtk::prelude::EntryExt as _;
use gtk::prelude::GtkWindowExt as _;
use gtk::prelude::WidgetExt as _;


pub struct OptionsDialog
{
    outputFileNamesPattern: String,
    widgets: Option<Widgets>,
    sender: Sender
}

impl EventHandler for OptionsDialog
{
    fn handle(&mut self, source: Source, event: &Event)
    {
        match event {
            Event::DialogResponded(response) => self.onDialogResponded(*response),
            Event::OpenOptionsRequested      => self.open(),
            _ => onUnknown(source, event)
        }
    }
}

impl OptionsDialog
{
    pub fn new(outputFileNamesPattern: &str, sender: Sender) -> Self
    {
        Self{outputFileNamesPattern: outputFileNamesPattern.into(), widgets: None, sender}
    }

    fn open(&mut self)
    {
        let guiElementProvider = GuiElementProvider::new(include_str!("options_dialog.glade"));
        let patternEntry = guiElementProvider.get::<gtk::Entry>("outputFileNamesPatternEntry");
        patternEntry.set_text(&self.outputFileNamesPattern);

        let dialog = guiElementProvider.get::<gtk::Dialog>("dialog");
        let sender = self.sender.clone();
        dialog.connect_response(move |_dialog, response| {
            sender.send((Source::OptionsDialogWidget, Event::DialogResponded(response))).unwrap();
        });

        let saveButton = guiElementProvider.get::<gtk::Button>("saveButton");
        let sender = self.sender.clone();
        saveButton.connect_clicked(move |_button| {
            sender.send((Source::OptionsDialogWidget, Event::DialogResponded(gtk::ResponseType::Apply))).unwrap();
        });

        let cancelButton = guiElementProvider.get::<gtk::Button>("cancelButton");
        let sender = self.sender.clone();
        cancelButton.connect_clicked(move |_button| {
            sender.send((Source::OptionsDialogWidget, Event::DialogResponded(gtk::ResponseType::Cancel))).unwrap();
        });

        dialog.set_modal(true);
        dialog.show();

        self.widgets = Some(Widgets{dialog, outputFileNamesPatternEntry: patternEntry});
    }

    fn onDialogResponded(&mut self, response: gtk::ResponseType)
    {
        match response {
            gtk::ResponseType::Apply       => self.onSaveDialog(),
            gtk::ResponseType::Cancel      => self.onCancelDialog(),
            gtk::ResponseType::DeleteEvent => self.onDialogDeleted(),
            _ => self.onUnknownDialogResponse(response)
        }
    }

    fn onSaveDialog(&mut self)
    {
        let widgets = match &self.widgets {
            Some(widgets) => widgets,
            None => {
                eprintln!("Expected OptionsDialog::widgets to be filled, but it was not");
                return;
            }
        };

        let newPattern = widgets.outputFileNamesPatternEntry.text().to_string();
        if self.outputFileNamesPattern != newPattern {
            self.outputFileNamesPattern = newPattern.clone();
            self.sender.send((Source::OptionsDialog, Event::OutputFileNamesPatternChanged(newPattern))).unwrap();
        }
        self.close();
    }

    fn onCancelDialog(&mut self)
    {
        self.close();
    }

    fn onDialogDeleted(&mut self)
    {
        self.widgets = None;
    }

    fn onUnknownDialogResponse(&mut self, response: gtk::ResponseType)
    {
        eprintln!("Received unknown dialog response: {:?}", response);
        self.close();
    }

    fn close(&mut self)
    {
        if let Some(widgets) = &self.widgets {
            widgets.dialog.close();
            self.widgets = None;
        }
    }
}

struct Widgets
{
    dialog: gtk::Dialog,
    outputFileNamesPatternEntry: gtk::Entry
}
