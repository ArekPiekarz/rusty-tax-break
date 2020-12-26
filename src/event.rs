use crate::repository::Repository;

use std::path::PathBuf;
use std::rc::Rc;


#[derive(Debug)]
pub enum Event
{
    Clicked,
    CommitAuthorFilterChanged(CommitAuthorFilter),
    CommitLogFilled,
    CommitSelected(git2::Oid),
    CommitUnselected,
    FolderChosen(FolderUri),
    GenerateReportRequested,
    MarkCommitForReportToggled(gtk::TreePath),
    MonthFilterChanged(chrono::Month),
    OutputPathChanged(PathBuf),
    PartialOutputPathChanged(PathBuf),
    RepositoryChanged(Rc<Repository>),
    SelectionChanged(gtk::TreeSelection),
    YearFilterChanged(Year),
    ZoomRequested(gdk::EventScroll)
}

pub type CommitAuthorFilter = String;
pub type CommitAuthorFilterStr = str;
pub type FolderUri = String;
pub type FolderUriStr = str;
pub type Year = i32;
