use crate::date_time::{LocalDateTime, ZERO_NANOSECONDS};
use crate::event::Event;
use crate::event_handling::{EventHandler, onUnknown, Sender};
use crate::repository::Repository;
use crate::source::Source;

use chrono::TimeZone as _;
use std::rc::Rc;

const INVALID_UTF8: &str = "<invalid UTF-8>";


pub struct CommitLog
{
    commits: Vec<CommitInfo>,
    sender: Sender
}

impl EventHandler for CommitLog
{
    fn handle(&mut self, source: Source, event: &Event)
    {
        match event {
            Event::RepositoryChanged(repo) => self.onRepositoryChanged(repo),
            _ => onUnknown(source, event)
        }
    }
}

impl CommitLog
{
    pub fn new(sender: Sender) -> Self
    {
        Self{commits: vec![], sender}
    }

    pub fn getCommits(&self) -> &[CommitInfo]
    {
        &self.commits
    }

    pub fn getCommit(&self, row: usize) -> Option<&CommitInfo>
    {
        self.commits.get(row)
    }

    pub fn setMarkedForReport(&mut self, row: usize, markedForReport: bool)
    {
        self.commits.get_mut(row).unwrap().markedForReport = markedForReport;
    }

    // private

    fn onRepositoryChanged(&mut self, repo: &Rc<Repository>)
    {
        self.commits.clear();
        if repo.isEmpty() {
            eprintln!("Cannot load commits, because repository is empty.");
            return;
        }

        repo.iterateCommits(|commit| {
            let summary = commit.summary().unwrap().into();
            let signature = commit.author();
            let date = chrono::Local.timestamp(signature.when().seconds(), ZERO_NANOSECONDS);
            let author = signature.name().unwrap_or(INVALID_UTF8).into();
            let email = signature.email().unwrap_or(INVALID_UTF8).into();
            let id = commit.id();
            self.commits.push(CommitInfo {id, summary, date, author, email, markedForReport: false});
        });

        self.sender.send((Source::CommitLog, Event::CommitLogFilled)).unwrap();
    }
}

pub struct CommitInfo
{
    pub id: git2::Oid,
    pub summary: String,
    pub date: LocalDateTime,
    pub author: String,
    pub email: String,
    pub markedForReport: bool
}
