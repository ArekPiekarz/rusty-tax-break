use crate::commit_log::CommitLog;
use crate::commit_log_column::{CommitLogColumn, OriginalRow};
use crate::event::{CommitAuthorFilterStr, Event, Year};
use crate::event_handling::{EventHandler, onUnknown, Sender};
use crate::gui_element_provider::GuiElementProvider;
use crate::source::Source;

use chrono::Datelike as _;
use gtk::glib;
use gtk::prelude::TreeModelExt as _;
use gtk::prelude::TreeModelFilterExt as _;
use std::cell::RefCell;
use std::convert::TryInto as _;
use std::rc::Rc;

pub struct CommitLogModelFilter
{
    modelFilter: gtk::TreeModelFilter,
    authorFilter: AuthorFilter,
    monthFilter: MonthFilter,
    yearFilter: YearFilter,
    sender: Sender
}

type AuthorFilter = Rc<RefCell<String>>;
type MonthFilter = Rc<RefCell<chrono::Month>>;
type YearFilter = Rc<RefCell<Year>>;

impl EventHandler for CommitLogModelFilter
{
    fn handle(&mut self, source: Source, event: &Event)
    {
        match event {
            Event::CommitAuthorFilterChanged(filter)    => self.onCommitAuthorFilterChanged(filter),
            Event::MarkCommitForReportToggled(treePath) => self.onReportCommitToggled(treePath),
            Event::MonthFilterChanged(month)            => self.onMonthChanged(*month),
            Event::YearFilterChanged(year)              => self.onYearChanged(*year),
            _ => onUnknown(source, event)
        }
    }
}

impl CommitLogModelFilter
{
    pub fn new(commitLog: Rc<RefCell<CommitLog>>, guiElementProvider: &GuiElementProvider, sender: Sender) -> Self
    {
        let modelFilter = guiElementProvider.get::<gtk::TreeModelFilter>("commitLogStoreFilter");
        let authorFilter = Rc::new(RefCell::new("".into()));
        let monthFilter = Rc::new(RefCell::new(chrono::Month::January));
        let yearFilter = Rc::new(RefCell::new(1));
        setupFilterFunction(
            commitLog, &modelFilter, Rc::clone(&authorFilter), Rc::clone(&monthFilter), Rc::clone(&yearFilter));
        Self{modelFilter, authorFilter, monthFilter, yearFilter, sender}
    }


    // private

    fn onCommitAuthorFilterChanged(&self, filter: &CommitAuthorFilterStr)
    {
        *self.authorFilter.borrow_mut() = filter.into();
        self.modelFilter.refilter();
    }

    fn onMonthChanged(&self, month: chrono::Month)
    {
        *self.monthFilter.borrow_mut() = month;
        self.modelFilter.refilter();
    }

    fn onReportCommitToggled(&self, treePath: &gtk::TreePath)
    {
        let childPath = self.modelFilter.convert_path_to_child_path(treePath).unwrap();
        self.sender.send((Source::CommitLogModelFilter, Event::MarkCommitForReportToggled(childPath))).unwrap();
    }

    fn onYearChanged(&self, year: Year)
    {
        *self.yearFilter.borrow_mut() = year;
        self.modelFilter.refilter();
    }
}

fn setupFilterFunction(
    commitLog: Rc<RefCell<CommitLog>>,
    modelFilter: &gtk::TreeModelFilter,
    authorFilter: AuthorFilter,
    monthFilter: MonthFilter,
    yearFilter: YearFilter)
{
    modelFilter.set_visible_func(move |model, iter| {
        if isRowEmpty(model, iter) {
            return false;
        }

        let originalRow = model.value(iter, CommitLogColumn::OriginalRow.into()).get::<OriginalRow>().unwrap()
            .try_into().unwrap();
        let date = commitLog.borrow().getCommit(originalRow).unwrap().date;
        if date.year() != *yearFilter.borrow() {
            return false;
        }
        if date.month() != monthFilter.borrow().number_from_month() {
            return false;
        }

        let authorFilter = &*authorFilter.borrow();
        if authorFilter.is_empty() {
            return true;
        }
        let author = model.value(iter, CommitLogColumn::Author.into()).get::<String>().unwrap();
        author == *authorFilter
    });
}

fn isRowEmpty(model: &gtk::TreeModel, iter: &gtk::TreeIter) -> bool
{
    match model.value(iter, CommitLogColumn::Date.into()).get::<&str>() {
        Ok(text) => text.is_empty(),
        Err(error) => match error {
            glib::value::ValueTypeMismatchOrNoneError::WrongValueType(e) => panic!("Wrong value type: {}", e),
            glib::value::ValueTypeMismatchOrNoneError::UnexpectedNone => true
        }
    }
}
