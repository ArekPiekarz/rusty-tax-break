use crate::date_time::makeDateTime;
use crate::event::Event;
use crate::event_handling::{EventHandler, onUnknown, Sender};
use crate::repository::Repository;
use crate::source::Source;

use std::rc::Rc;
use time::OffsetDateTime;

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
    pub fn new(repoOpt: &Option<Rc<Repository>>, sender: Sender) -> Self
    {
        let mut newSelf = Self{commits: vec![], sender};
        if let Some(repo) = repoOpt {
            newSelf.loadCommits(repo);
        }
        newSelf
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
        self.loadCommits(repo);
        self.sender.send((Source::CommitLog, Event::CommitLogChanged)).unwrap();
    }

    fn loadCommits(&mut self, repo: &Rc<Repository>)
    {
        if repo.isEmpty() {
            return;
        }

        repo.iterateCommits(|commit| {
            let summary = getSummary(commit);
            let signature = commit.author();
            let date = makeDateTime(&commit.time());
            let author = signature.name().unwrap_or(INVALID_UTF8).into();
            let email = signature.email().unwrap_or(INVALID_UTF8).into();
            let id = commit.id();
            self.commits.push(CommitInfo {id, summary, date, author, email, markedForReport: false});
        });
    }
}

fn getSummary(commit: &git2::Commit) -> String
{
    match commit.summary() {
        Some(summary) => summary.into(),
        None => getSummaryFromRaw(commit)
    }
}

fn getSummaryFromRaw(commit: &git2::Commit) -> String
{
    match commit.summary_bytes() {
        Some(bytes) => String::from_utf8_lossy(bytes).into(),
        None => "<UNKNOWN SUMMARY>".into()
    }
}

#[derive(Debug)]
pub struct CommitInfo
{
    pub id: git2::Oid,
    pub summary: String,
    pub date: OffsetDateTime,
    pub author: String,
    pub email: String,
    pub markedForReport: bool
}
