use crate::date_time::makeDateTime;
use crate::diff_formatter::DiffFormatter;

use time::format_description::well_known::Rfc2822;


pub fn makeCommitSummary(commit: &git2::Commit) -> String
{
    format!(
        "Commit: {}\nAuthor: {} <{}>\nDate:   {}\n\n{}\n",
        commit.id(),
        commit.author().name().unwrap(),
        commit.author().email().unwrap(),
        formatDateTime(&commit.time()),
        tabulateCommitMessage(&getMessage(commit)))
}

pub fn makeFormattedDiff(diff: &git2::Diff) -> String
{
    let mut diffFormatter = DiffFormatter::new();
    diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| diffFormatter.format(&line)).unwrap();
    diffFormatter.takeText()
}

fn getMessage(commit: &git2::Commit) -> String
{
    match commit.message() {
        Some(message) => message.into(),
        None => String::from_utf8_lossy(commit.message_bytes()).into()
    }
}

fn formatDateTime(inputTime: &git2::Time) -> String
{
    makeDateTime(inputTime).format(&Rfc2822).unwrap()
}

fn tabulateCommitMessage(message: &str) -> String
{
    let mut result = String::new();
    for line in message.lines() {
        result.push_str(&format!("    {}\n", line));
    }
    result
}
