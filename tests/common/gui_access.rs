use crate::common::test_gui::TestGui;

use glib::Cast as _;


pub fn findChooseRepositoryFolderButton(gui: &TestGui) -> gtk::Button
{
    gui.findWidget::<gtk::Button>("chooseRepositoryFolderButton")
}

pub fn findRepositoryPathLabel(gui: &TestGui) -> gtk::Label
{
    gui.findWidget::<gtk::Label>("repositoryPathLabel")
}

pub fn findCommitLogView(gui: &TestGui) -> gtk::TreeView
{
    gui.findWidget::<gtk::TreeView>("commitLogView")
}

pub fn findFileChooserDialog() -> gtk::FileChooserDialog
{
    let mut topLevelWindows = gtk::Window::list_toplevels();
    topLevelWindows.remove(2).downcast::<gtk::FileChooserDialog>().unwrap()
}
