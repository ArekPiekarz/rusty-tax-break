use crate::common::date_time::LocalDate;
use crate::common::test_gui::TestGui;

use rusty_tax_break::gui::Gui;

use color_backtrace::BacktracePrinter;
use glib::{Cast as _, ObjectExt as _};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use tempfile::{tempdir, TempDir};
use termcolor::{ColorChoice, StandardStream};

pub const COMMIT_AUTHOR: &str = "John Smith";
pub const COMMIT_EMAIL: &str = "john.smith@example.com";


pub fn setupTest() -> (TempDir, PathBuf)
{
    setupPanicHandler();
    let (repoDirGuard, repoDirPath) = makeTemporaryDir();
    initializeGitRepository(&repoDirPath);
    (repoDirGuard, repoDirPath)
}

pub fn setupTestWithoutRepo()
{
    setupPanicHandler();
}

pub fn makeTemporaryDir() -> (TempDir, PathBuf)
{
    let tempDir = tempdir().unwrap_or_else(|e| panic!("Failed to create temporary directory: {}", e));
    let path = tempDir.path().into();
    (tempDir, path)
}

pub fn getCurrentDate() -> LocalDate
{
    chrono::Local::today()
}

pub fn makeGui() -> TestGui
{
    let gui = Gui::new();
    gui.show();
    TestGui::new(getAppWindow())
}


// private

fn setupPanicHandler()
{
    BacktracePrinter::default().install(Box::new(StandardStream::stderr(ColorChoice::Always)));
}

fn initializeGitRepository(repoDir: &Path)
{
    initializeGitRepositoryWith(&["git", "init"], repoDir);
    initializeGitRepositoryWith(&["git", "config", "user.name", COMMIT_AUTHOR], repoDir);
    initializeGitRepositoryWith(&["git", "config", "user.email", COMMIT_EMAIL], repoDir);
}

fn initializeGitRepositoryWith(commandParts: &[&str], repoDir: &Path)
{
    let mut command = Command::new(commandParts[0]);
    command.args(&commandParts[1..]).current_dir(&repoDir).stdout(Stdio::null());
    let status = command.status().unwrap();
    assert_eq!(true, status.success(),
               "Failed to initialize git repository.\nPath: {:?}\nCommand: {:?}\nCommand status: {}",
               repoDir, command, status);
}

fn getAppWindow() -> gtk::ApplicationWindow
{
    let mut topLevelWindows = gtk::Window::list_toplevels();
    match topLevelWindows.len() {
        1 => topLevelWindows.remove(0).downcast::<gtk::ApplicationWindow>().unwrap(),
        2 => {
            let tooltipWindow = topLevelWindows[1].downcast_ref::<gtk::Window>().unwrap();
            assert_eq!(tooltipWindow.get_type().name(), "GtkWindow");
            topLevelWindows.remove(0).downcast::<gtk::ApplicationWindow>().unwrap()
        },
        count => panic!("Wrong number of windows, expected 1 or 2, got {}: {:?}", count, topLevelWindows)
    }
}
