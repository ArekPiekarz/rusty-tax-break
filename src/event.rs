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
    DialogResponded(gtk::ResponseType),
    FolderChosen(PathBuf),
    GenerateReportRequested,
    MarkCommitForReportToggled(gtk::TreePath),
    MonthFilterChanged(chrono::Month),
    OpenOptionsRequested,
    OutputFileNamesPatternChanged(String),
    OutputPathChanged(PathBuf),
    PartialOutputPathChanged(PathBuf),
    RepositoryChanged(Rc<Repository>),
    SelectionChanged(gtk::TreeSelection),
    YearFilterChanged(Year),
    ZoomRequested(gdk::EventScroll)
}

pub type CommitAuthorFilter = String;
pub type CommitAuthorFilterStr = str;
pub type Year = i32;
