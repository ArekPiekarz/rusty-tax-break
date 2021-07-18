#![allow(non_snake_case)]

use crate::common::gui_access::findFileChooserDialog;
use crate::common::gui_assertions::{assertCommitLogViewIsEmpty, assertRepositoryPathLabelTextIsPlaceholder};
use crate::common::gui_interactions::{clickChooseRepositoryFolderButton, closeDialog};
use crate::common::test_setup::{makeGui, setupTestWithoutRepo};

use gtk::glib;
use rusty_fork::rusty_fork_test;


rusty_fork_test! {
#[test]
fn closeChoosingRepository()
{
    let context = glib::MainContext::default();
    let _guard = context.acquire().unwrap();
    setupTestWithoutRepo();
    let gui = makeGui();
    assertRepositoryPathLabelTextIsPlaceholder(&gui);
    assertCommitLogViewIsEmpty(&gui);

    clickChooseRepositoryFolderButton(&gui);
    let dialog = findFileChooserDialog();
    closeDialog(&dialog);

    assertRepositoryPathLabelTextIsPlaceholder(&gui);
    assertCommitLogViewIsEmpty(&gui);
}
}
