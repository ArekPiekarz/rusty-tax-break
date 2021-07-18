#![allow(non_snake_case)]

use crate::common::gui_access::findFileChooserDialog;
use crate::common::gui_assertions::assertOutputPathLabelTextIsPlaceholder;
use crate::common::gui_interactions::{closeDialog, clickChooseOutputFolderButton};
use crate::common::test_setup::{getCurrentDate, makeGui, setupTestWithoutRepo};

use gtk::glib;
use rusty_fork::rusty_fork_test;


rusty_fork_test! {
#[test]
fn closeChoosingOutputFolder()
{
    let context = glib::MainContext::default();
    let _guard = context.acquire().unwrap();
    setupTestWithoutRepo();
    let gui = makeGui();
    let currentDate = getCurrentDate();
    assertOutputPathLabelTextIsPlaceholder(&currentDate, &gui);

    clickChooseOutputFolderButton(&gui);
    let dialog = findFileChooserDialog();
    closeDialog(&dialog);

    assertOutputPathLabelTextIsPlaceholder(&currentDate, &gui);
}
}
