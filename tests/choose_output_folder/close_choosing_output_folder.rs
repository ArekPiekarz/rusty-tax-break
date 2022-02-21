#![allow(non_snake_case)]

use crate::common::gui_access::findFileChooserDialog;
use crate::common::gui_assertions::assertOutputPathLabelTextIsPlaceholder;
use crate::common::gui_interactions::{closeDialog, clickChooseOutputFolderButton};
use crate::common::test_setup::{makeGui, setupTestWithoutRepo};

use rusty_tax_break::date_time::getCurrentDate;

use rusty_fork::rusty_fork_test;


rusty_fork_test! {
#[test]
fn closeChoosingOutputFolder()
{
    let testResources = setupTestWithoutRepo();
    let gui = makeGui(testResources.getConfigFilePath());
    let currentDate = getCurrentDate();
    assertOutputPathLabelTextIsPlaceholder(&currentDate, &gui);

    clickChooseOutputFolderButton(&gui);
    let dialog = findFileChooserDialog();
    closeDialog(&dialog);

    assertOutputPathLabelTextIsPlaceholder(&currentDate, &gui);
}
}
