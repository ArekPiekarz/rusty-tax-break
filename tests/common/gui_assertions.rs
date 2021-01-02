use crate::common::date_time::LocalDate;
use crate::common::gui_access::{findCommitLogView, findOutputPathLabel, findRepositoryPathLabel};
use crate::common::test_gui::TestGui;

use chrono::Datelike as _;
use gtk::LabelExt as _;
use gtk::TreeModelExt as _;
use gtk::TreeViewExt as _;
use std::path::Path;

const CONTINUE_ITERATING_MODEL: bool = false;


pub fn assertRepositoryPathLabelTextIs(expectedText: &str, gui: &TestGui)
{
    let label = findRepositoryPathLabel(gui);
    assert_eq!(label.get_text().as_str(), expectedText, "\nActual repository path label text differs from expected");
}

pub fn assertRepositoryPathLabelTextIsPlaceholder(gui: &TestGui)
{
    assertRepositoryPathLabelTextIs("none", gui);
}

pub fn assertCommitLogViewContentIs(expected: &[CommitLogRow], gui: &TestGui)
{
    let actual = collectCommitLogViewContent(&gui);
    assert_eq!(actual, expected, "\nActual commit log view content differs from expected");
}

pub fn assertCommitLogViewIsEmpty(gui: &TestGui)
{
    let expected: Vec<CommitLogRow> = vec![];
    let actual = collectCommitLogViewContent(&gui);
    assert_eq!(actual, expected, "\nActual commit log view content should be empty, but isn't");
}

pub fn makeCommitLogRow(markedForReport: bool, message: &str, date: &str, author: &str, email: &str) -> CommitLogRow
{
    CommitLogRow {
        markedForReport,
        message: message.into(),
        date: date.into(),
        author: author.into(),
        email: email.into()
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct CommitLogRow
{
    markedForReport: bool,
    message: String,
    date: String,
    author: String,
    email: String
}

pub fn assertOutputPathLabelTextIs(expectedText: &str, gui: &TestGui)
{
    let label = findOutputPathLabel(gui);
    assert_eq!(label.get_text().as_str(), expectedText, "\nActual output path label text differs from expected");
}

pub fn assertOutputPathLabelTextIsPlaceholder(date: &LocalDate, gui: &TestGui)
{
    assertOutputPathLabelTextIs(&makePlaceholderOutputPathLabelText(date), &gui);
}

pub fn makeOutputPathLabelText(outputPathPrefix: &Path, date: &LocalDate) -> String
{
    format!("{}/{}/{:02}", outputPathPrefix.to_string_lossy(), date.year(), date.month())
}

// private

fn collectCommitLogViewContent(gui: &TestGui) -> Vec<CommitLogRow>
{
    let mut content = vec![];
    let view = findCommitLogView(gui);
    view.get_model().unwrap().foreach(|model, _row, iter| {
        content.push(CommitLogRow{
            markedForReport: getMarkedForReportCell(model, iter),
            message: getMessageCell(model, iter),
            date: getDateCell(model, iter),
            author: getAuthorCell(model, iter),
            email: getEmailCell(model, iter)
        });
        CONTINUE_ITERATING_MODEL
    });
    content
}

fn getMarkedForReportCell(model: &gtk::TreeModel, iter: &gtk::TreeIter) -> bool
{
    getCellBool(model, iter, CommitLogColumn::MarkedForReport.into())
}

fn getMessageCell(model: &gtk::TreeModel, iter: &gtk::TreeIter) -> String
{
    getCellString(model, iter, CommitLogColumn::Message.into())
}

fn getDateCell(model: &gtk::TreeModel, iter: &gtk::TreeIter) -> String
{
    getCellString(model, iter, CommitLogColumn::Date.into())
}

fn getAuthorCell(model: &gtk::TreeModel, iter: &gtk::TreeIter) -> String
{
    getCellString(model, iter, CommitLogColumn::Author.into())
}

fn getEmailCell(model: &gtk::TreeModel, iter: &gtk::TreeIter) -> String
{
    getCellString(model, iter, CommitLogColumn::Email.into())
}

fn getCellBool(model: &gtk::TreeModel, iter: &gtk::TreeIter, column: i32) -> bool
{
    model.get_value(iter, column).get::<bool>().unwrap().unwrap()
}

fn getCellString(model: &gtk::TreeModel, iter: &gtk::TreeIter, column: i32) -> String
{
    model.get_value(iter, column).get::<String>().unwrap().unwrap()
}

enum CommitLogColumn
{
    MarkedForReport,
    Message,
    Date,
    Author,
    Email
}

impl From<CommitLogColumn> for i32
{
    fn from(value: CommitLogColumn) -> Self
    {
        value as Self
    }
}

fn makePlaceholderOutputPathLabelText(date: &LocalDate) -> String
{
    format!("<path>/{}/{:02}", date.year(), date.month())
}
