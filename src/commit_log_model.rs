use crate::commit_log::CommitLog;
use crate::commit_log_column::{CommitLogColumn, OriginalRow};
use crate::event::Event;
use crate::event_handling::{EventHandler, onUnknown};
use crate::gui_element_provider::GuiElementProvider;
use crate::source::Source;

use gtk::GtkListStoreExt as _;
use gtk::prelude::GtkListStoreExtManual as _;
use gtk::TreeModelExt as _;
use std::cell::RefCell;
use std::convert::TryInto as _;
use std::rc::Rc;
use to_trait::To as _;

const DO_NOT_REPORT_COMMIT: bool = false;


pub struct CommitLogModel
{
    commitLog: Rc<RefCell<CommitLog>>,
    store: gtk::ListStore
}

impl EventHandler for CommitLogModel
{
    fn handle(&mut self, source: Source, event: &Event)
    {
        match event {
            Event::CommitLogFilled                      => self.onCommitLogFilled(),
            Event::MarkCommitForReportToggled(treePath) => self.onMarkCommitForReportToggled(treePath),
            _ => onUnknown(source, event)
        }
    }
}

impl CommitLogModel
{
    pub fn new(commitLog: Rc<RefCell<CommitLog>>, guiElementProvider: &GuiElementProvider) -> Self
    {
        Self{
            commitLog,
            store: guiElementProvider.get::<gtk::ListStore>("commitLogStore")
        }
    }


    // private

    fn onCommitLogFilled(&self)
    {
        self.store.clear();
        // for date formatting below, see https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html
        for (row, commit) in self.commitLog.borrow().getCommits().iter().enumerate() {
            self.store.set(
                &self.store.append(),
                CommitLogColumn::allAsArrayOfU32(),
                &[&DO_NOT_REPORT_COMMIT,
                  &commit.message,
                  &commit.date.format("%_d %b %Y %_H:%M:%S").to_string(),
                  &commit.author,
                  &commit.email,
                  &(row.try_to::<OriginalRow>().unwrap())]);
        }
    }

    fn onMarkCommitForReportToggled(&self, treePath: &gtk::TreePath)
    {
        let iter = self.store.get_iter(treePath).unwrap();
        let report = !self.store.get_value(&iter, CommitLogColumn::Report.into()).get_some::<bool>().unwrap();
        self.store.set(&iter, &[CommitLogColumn::Report.into()], &[&report]);
        let row = self.store.get_value(&iter, CommitLogColumn::OriginalRow.into()).get_some::<OriginalRow>().unwrap()
            .try_into().unwrap();
        self.commitLog.borrow_mut().setMarkedForReport(row, report);
    }
}
