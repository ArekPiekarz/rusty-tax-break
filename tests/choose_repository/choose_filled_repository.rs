#![allow(non_snake_case)]

use crate::common::gui_access::findFileChooserDialog;
use crate::common::gui_assertions::{
    assertCommitLogViewContentIs, assertCommitLogViewIsEmpty, assertRepositoryPathLabelTextIs, makeCommitLogRow};
use crate::common::gui_interactions::{acceptDialog, clickChooseRepositoryFolderButton, setCurrentFolderInDialog};
use crate::common::repository_setup::{findLastCommitDateForLogView, makeCommit, makeNewStagedFile};
use crate::common::test_setup::{COMMIT_AUTHOR, COMMIT_EMAIL, makeGui, setupTest};

use gtk::glib;
use rusty_fork::rusty_fork_test;
use std::path::PathBuf;


rusty_fork_test! {
#[test]
fn chooseFilledRepository()
{
    let context = glib::MainContext::default();
    let _guard = context.acquire().unwrap();
    let (_repoDirGuard, repoDir) = setupTest();
    let repoDirStr = repoDir.to_str().unwrap();
    let filePath = PathBuf::from("some_file");
    makeNewStagedFile(&filePath, "some file content\n", &repoDir);
    makeCommit(COMMIT_MESSAGE, &repoDir);
    let commitDate = findLastCommitDateForLogView(&repoDir);
    let gui = makeGui();
    assertRepositoryPathLabelTextIs("none", &gui);
    assertCommitLogViewIsEmpty(&gui);

    clickChooseRepositoryFolderButton(&gui);
    let dialog = findFileChooserDialog();
    setCurrentFolderInDialog(&repoDir, &dialog);
    acceptDialog(&dialog);

    assertRepositoryPathLabelTextIs(repoDirStr, &gui);
    assertCommitLogViewContentIs(
        &[makeCommitLogRow(NOT_MARKED_FOR_REPORT, COMMIT_MESSAGE, &commitDate, COMMIT_AUTHOR, COMMIT_EMAIL)], &gui);
}
}

const NOT_MARKED_FOR_REPORT: bool = false;
const COMMIT_MESSAGE: &str = "initial commit";
