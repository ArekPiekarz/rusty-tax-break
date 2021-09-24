#![allow(non_snake_case)]

use crate::common::gui_access::findFileChooserDialog;
use crate::common::gui_assertions::assertOutputPathLabelTextIsPlaceholder;
use crate::common::gui_interactions::{cancelDialog, clickChooseOutputFolderButton};
use crate::common::test_setup::{getCurrentDate, makeGui, setupTestWithoutRepo};

use rusty_fork::rusty_fork_test;


rusty_fork_test! {
#[test]
fn cancelChoosingOutputFolder()
{
    let testResources = setupTestWithoutRepo();
    let gui = makeGui(testResources.getConfigFilePath());
    let currentDate = getCurrentDate();
    assertOutputPathLabelTextIsPlaceholder(&currentDate, &gui);

    clickChooseOutputFolderButton(&gui);
    let dialog = findFileChooserDialog();
    cancelDialog(&dialog);

    assertOutputPathLabelTextIsPlaceholder(&currentDate, &gui);
}
}
