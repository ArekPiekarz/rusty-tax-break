use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write as _;
use std::path::Path;
use std::process::Command;
use std::process::Stdio;

type LocalDateTime = chrono::DateTime<chrono::Local>;


pub fn makeNewStagedFile(filePath: &Path, content: &str, repositoryDir: &Path)
{
    makeNewUnstagedFile(filePath, content, repositoryDir);
    stageFile(filePath, repositoryDir);
}

pub fn makeCommit(message: &str, repositoryDir: &Path)
{
    let status = Command::new("git").args(&["commit", "-m", message])
        .current_dir(&repositoryDir).stdout(Stdio::null()).status().unwrap();
    assert_eq!(true, status.success(),
               "Failed to create a commit with message \"{}\", command finished with {}", message, status);
}

pub fn findLastCommitDateForLogView(repoDir: &Path) -> String
{
    // for date formatting below, see https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html
    findLastCommitDate(repoDir).format("%_d %b %Y %_H:%M:%S").to_string()
}


// private

fn makeNewUnstagedFile(filePath: &Path, content: &str, repositoryDir: &Path)
{
    let mut file = makeNewWritableFile(&repositoryDir.join(filePath));
    file.write(content.as_bytes()).unwrap();
}

fn makeNewWritableFile(filePath: &Path) -> File
{
    OpenOptions::new().write(true).create_new(true).open(filePath).unwrap()
}

fn stageFile(filePath: &Path, repositoryDir: &Path)
{
    let status = Command::new("git").args(&["add", filePath.to_str().unwrap()])
        .current_dir(&repositoryDir).status().unwrap();
    assert_eq!(true, status.success(),
               "Failed to stage file \"{:?}\", command finished with {}", filePath, status);
}

fn findLastCommitDate(repoDir: &Path) -> LocalDateTime
{
    // --format=%cD means output contains only a commit date in RFC2822 format
    // see https://git-scm.com/docs/git-log#Documentation/git-log.txt-emcdem
    let output = getCommandStdoutString(&["git", "log", "-1", "--format=%cD"], repoDir).trim_end().to_owned();
    chrono::DateTime::parse_from_rfc2822(&output).unwrap().into()
}

fn getCommandStdoutString(commandParts: &[&str], repositoryDir: &Path) -> String
{
    String::from_utf8(getCommandResults(commandParts, repositoryDir).stdout).unwrap()
}

fn getCommandResults(commandParts: &[&str], repositoryDir: &Path) -> std::process::Output
{
    let mut command = Command::new(commandParts[0]);
    command.args(&commandParts[1..]).current_dir(&repositoryDir);
    command.output().unwrap()
}
