use crate::date_time::ZERO_NANOSECONDS;
use crate::diff_formatter::DiffFormatter;

use chrono::TimeZone as _;


pub fn makeCommitSummary(commit: &git2::Commit) -> String
{
    format!(
        "Commit: {}\nAuthor: {} <{}>\nDate:   {}\n\n{}\n",
        commit.id(),
        commit.author().name().unwrap(),
        commit.author().email().unwrap(),
        chrono::Local.timestamp(commit.time().seconds(), ZERO_NANOSECONDS).to_rfc2822(),
        tabulateCommitMessage(commit.message().unwrap()))
}

pub fn makeFormattedDiff(diff: &git2::Diff) -> String
{
    let mut diffFormatter = DiffFormatter::new();
    diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| diffFormatter.format(&line)).unwrap();
    diffFormatter.takeText()
}

fn tabulateCommitMessage(message: &str) -> String
{
    let mut result = String::new();
    for line in message.lines() {
        result.push_str(&format!("    {}\n", line));
    }
    result
}
