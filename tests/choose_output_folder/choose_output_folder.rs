#![allow(non_snake_case)]

use crate::common::gui_access::findFileChooserDialog;
use crate::common::gui_assertions::{
    assertOutputPathLabelTextIs, assertOutputPathLabelTextIsPlaceholder, makeOutputPathLabelText};
use crate::common::gui_interactions::{acceptDialog, clickChooseOutputFolderButton, setCurrentFolderInDialog};
use crate::common::test_setup::{getCurrentDate, makeGui, makeTemporaryDir, setupTestWithoutRepo};

use rusty_fork::rusty_fork_test;


rusty_fork_test! {
#[test]
fn chooseOutputFolder()
{
    setupTestWithoutRepo();
    let (_outputPathPrefixGuard, outputPathPrefix) = makeTemporaryDir();
    let gui = makeGui();
    let currentDate = getCurrentDate();
    assertOutputPathLabelTextIsPlaceholder(&currentDate, &gui);

    clickChooseOutputFolderButton(&gui);
    let dialog = findFileChooserDialog();
    setCurrentFolderInDialog(&outputPathPrefix, &dialog);
    acceptDialog(&dialog);

    assertOutputPathLabelTextIs(&makeOutputPathLabelText(&outputPathPrefix, &currentDate), &gui);
}
}
