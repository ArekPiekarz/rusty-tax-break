#![allow(non_snake_case)]

use crate::common::gui_access::findFileChooserDialog;
use crate::common::gui_assertions::{assertCommitLogViewIsEmpty, assertRepositoryPathLabelTextIs};
use crate::common::gui_interactions::{acceptDialog, clickChooseRepositoryFolderButton, setCurrentFolderInDialog};
use crate::common::test_setup::{makeGui, setupTest};

use gtk::glib;
use rusty_fork::rusty_fork_test;


rusty_fork_test! {
#[test]
fn chooseEmptyRepository()
{
    let context = glib::MainContext::default();
    let _guard = context.acquire().unwrap();
    let testResources = setupTest();
    let repoDir = testResources.getRepoDir();
    let repoDirStr = repoDir.to_str().unwrap();
    let gui = makeGui(testResources.getConfigFilePath());
    assertRepositoryPathLabelTextIs("none", &gui);
    assertCommitLogViewIsEmpty(&gui);

    clickChooseRepositoryFolderButton(&gui);
    let dialog = findFileChooserDialog();
    setCurrentFolderInDialog(&repoDir, &dialog);
    acceptDialog(&dialog);

    assertRepositoryPathLabelTextIs(repoDirStr, &gui);
    assertCommitLogViewIsEmpty(&gui);
}
}
