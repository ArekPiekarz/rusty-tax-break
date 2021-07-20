use crate::repository::Repository;

use gtk::gdk;
use std::path::PathBuf;
use std::rc::Rc;


#[derive(Debug)]
pub enum Event
{
    Clicked,
    CommitAuthorFilterChanged(CommitAuthorFilter),
    CommitLogChanged,
    CommitSelected(git2::Oid),
    CommitUnselected,
    DialogResponded(gtk::ResponseType),
    FolderChosen(PathBuf),
    GenerateReportRequested,
    MarkCommitForReportToggled(gtk::TreePath),
    MonthFilterChanged(chrono::Month),
    OpenOptionsRequested,
    OutputFileNamesPatternChanged(String),
    OutputPathChanged(OutputPathInfo),
    PartialOutputPathChanged(PathBuf),
    RepositoryChanged(Rc<Repository>),
    SelectionChanged(gtk::TreeSelection),
    YearFilterChanged(Year),
    ZoomRequested(gdk::EventScroll)
}

pub type CommitAuthorFilter = String;
pub type CommitAuthorFilterStr = str;
pub type Year = i32;

#[derive(Debug)]
pub struct OutputPathInfo{pub full: PathBuf, pub prefix: PathBuf}
