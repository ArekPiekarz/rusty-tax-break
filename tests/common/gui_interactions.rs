use crate::common::event_processing::processEvents;
use crate::common::gui_access::{findChooseOutputFolderButton, findChooseRepositoryFolderButton};
use crate::common::test_gui::TestGui;

use gtk::ButtonExt as _;
use gtk::DialogExt as _;
use gtk::FileChooserExt as _;
use gtk::WidgetExt as _;
use std::path::Path;


pub fn clickChooseRepositoryFolderButton(gui: &TestGui)
{
    clickButton(&findChooseRepositoryFolderButton(gui));
}

pub fn clickChooseOutputFolderButton(gui: &TestGui)
{
    clickButton(&findChooseOutputFolderButton(gui));
}

pub fn setCurrentFolderInDialog(path: &Path, dialog: &gtk::FileChooserDialog)
{
    // Changing the current folder in a file chooser dialog often doesn't seem to work on the first try.
    // Perhaps the reason for it is that it is done through a different mechanism than modifying normal widgets
    // and needs time to propagate the change.
    dialog.set_current_folder(path);
    processEvents();
    while dialog.get_current_folder() != Some(path.into()) {
        processEvents();
    }
}

pub fn acceptDialog(dialog: &gtk::FileChooserDialog)
{
    dialog.response(gtk::ResponseType::Accept);
    processEvents();
}

pub fn cancelDialog(dialog: &gtk::FileChooserDialog)
{
    dialog.response(gtk::ResponseType::Cancel);
    processEvents();
}

pub fn closeDialog(dialog: &gtk::FileChooserDialog)
{
    dialog.response(gtk::ResponseType::DeleteEvent);
    processEvents();
}


// private

fn clickButton(button: &gtk::Button)
{
    assert!(button.is_sensitive());
    button.clicked();
    processEvents();
}
