use crate::commit_diff::{makeCommitSummary, makeFormattedDiff};
use crate::commit_log::{CommitLog, CommitInfo};
use crate::date_time::makeDateTime;
use crate::event::{Event, OutputPathInfo};
use crate::event_handling::{EventHandler, onUnknown};
use crate::repository::Repository;
use crate::source::Source;

use std::cell::RefCell;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write as _;
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;
use time::OffsetDateTime;
use zip::write::FileOptions as ZipFileOptions;
use zip::write::ZipWriter;

const DIFF_FILE_PATH: &str = "changes.diff";


pub struct ReportGenerator
{
    commitLog: Rc<RefCell<CommitLog>>,
    repo: Option<Rc<Repository>>,
    outputPath: Option<PathBuf>,
    outputFileNamesPattern: String
}

impl EventHandler for ReportGenerator
{
    fn handle(&mut self, source: Source, event: &Event)
    {
        match event {
            Event::GenerateReportRequested                => self.generateReport(),
            Event::OutputFileNamesPatternChanged(pattern) => self.onOutputFileNamesPatternChanged(pattern),
            Event::OutputPathChanged(pathInfo)            => self.onOutputPathChanged(pathInfo),
            Event::RepositoryChanged(repo)                => self.onRepositoryChanged(repo),
            _ => onUnknown(source, event)
        }
    }
}

impl ReportGenerator
{
    pub fn new(
        commitLog: Rc<RefCell<CommitLog>>,
        repo: Option<Rc<Repository>>,
        outputPath: Option<PathBuf>,
        outputFileNamesPattern: &str)
        -> Self
    {
        Self{commitLog, repo, outputPath, outputFileNamesPattern: outputFileNamesPattern.into()}
    }


    // private

    fn onOutputFileNamesPatternChanged(&mut self, pattern: &str)
    {
        self.outputFileNamesPattern = pattern.into();
    }

    fn onOutputPathChanged(&mut self, pathInfo: &OutputPathInfo)
    {
        self.outputPath = Some(pathInfo.full.clone());
    }

    fn generateReport(&self)
    {
        let repo = match &self.repo {
            Some(repo) => repo,
            None => return
        };

        let outputPath = match &self.outputPath {
            Some(path) => path,
            None => return
        };

        std::fs::create_dir_all(outputPath).unwrap();

        for commitInfo in self.commitLog.borrow().getCommits() {
            if !commitInfo.markedForReport {
                continue;
            }
            self.reportCommit(commitInfo, repo, outputPath);
        }
    }

    fn onRepositoryChanged(&mut self, repo: &Rc<Repository>)
    {
        self.repo = Some(Rc::clone(repo));
    }

    fn reportCommit(&self, commitInfo: &CommitInfo, repo: &Repository, outputPath: &Path)
    {
        let commitId = commitInfo.id;
        let commit = repo.findCommit(commitId).unwrap();
        let commitsDiff = repo.makeDiffOfCommitAndParent(&commit);
        let commitDateTime = toZipDateTime(&makeDateTime(&commit.time()));

        let zipFileNameStem = self.formatFileName(commitInfo, repo);
        let fullFilesZipPath = makeFullFilesZipPath(outputPath, &zipFileNameStem);
        let fullFilesZipFile = OpenOptions::new().write(true).create_new(true).open(fullFilesZipPath).unwrap();
        let mut fullFilesZipWriter = ZipWriter::new(fullFilesZipFile);

        let diffAndFullFilesZipPath = makeDiffAndFullFilesZipPath(outputPath, &zipFileNameStem);
        let diffAndFullFilesZipFile = OpenOptions::new().write(true).create_new(true).open(diffAndFullFilesZipPath).unwrap();
        let mut diffAndFullFilesZipWriter = ZipWriter::new(diffAndFullFilesZipFile);

        let zipOptions = ZipFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated)
            .last_modified_time(commitDateTime);

        reportDiffFile(&commit, &commitsDiff, &mut diffAndFullFilesZipWriter, &zipOptions);
        reportFullFiles(&commitsDiff, repo, &mut fullFilesZipWriter, &mut diffAndFullFilesZipWriter, &zipOptions);

        fullFilesZipWriter.finish().unwrap();
        diffAndFullFilesZipWriter.finish().unwrap();
    }

    fn formatFileName(&self, commitInfo: &CommitInfo, repo: &Repository) -> String
    {
        let mut fileName = self.outputFileNamesPattern.clone();
        if fileName.contains("<commit_id>") {
            fileName = fileName.replace("<commit_id>", &commitInfo.id.to_string());
        }
        if fileName.contains("<commit_short_id>") {
            fileName = fileName.replace("<commit_short_id>", &makeCommitShortId(commitInfo.id, repo));
        }
        if fileName.contains("<commit_summary>") {
            fileName = fileName.replace("<commit_summary>", &commitInfo.summary);
        }
        let sanitizingOptions = sanitize_filename::Options{windows: true, truncate: true, replacement: "_"};
        sanitize_filename::sanitize_with_options(fileName, sanitizingOptions)
    }
}

fn makeCommitShortId(commitId: git2::Oid, repo: &Repository) -> String
{
    repo.findCommit(commitId).unwrap().as_object().short_id().unwrap().as_str().unwrap().into()
}

fn makeFullFilesZipPath(outputPath: &Path, fileNameStem: &str) -> PathBuf
{
    let mut filePath = outputPath.to_owned();
    filePath.push(format!("{}.zip", fileNameStem));
    filePath
}

fn makeDiffAndFullFilesZipPath(outputPath: &Path, fileNameStem: &str) -> PathBuf
{
    let mut filePath = outputPath.to_owned();
    filePath.push(format!("{}-diff.zip", fileNameStem));
    filePath
}

fn reportDiffFile(
    commit: &git2::Commit,
    commitsDiff: &git2::Diff,
    zipWriter: &mut ZipWriter<File>,
    zipOptions: &ZipFileOptions)
{
    zipWriter.start_file(DIFF_FILE_PATH, *zipOptions).unwrap();
    let textDiff = makeCommitSummary(commit) + &makeFormattedDiff(commitsDiff);
    zipWriter.write_all(textDiff.as_bytes()).unwrap();
}

fn reportFullFiles(
    commitsDiff: &git2::Diff,
    repo: &Repository,
    fullFilesZipWriter: &mut ZipWriter<File>,
    diffAndFullFilesZipWriter: &mut ZipWriter<File>,
    zipOptions: &ZipFileOptions)
{
    for delta in commitsDiff.deltas() {
        if delta.status() == git2::Delta::Deleted {
            continue;
        }
        let file = delta.new_file();
        let blob = repo.findBlob(file.id());
        let fileContent = blob.content();

        let fullFilesFilePath = file.path().unwrap();
        fullFilesZipWriter.start_file(fullFilesFilePath.to_str().unwrap(), *zipOptions).unwrap();
        fullFilesZipWriter.write_all(fileContent).unwrap();

        let diffAndFullFilesFilePath = PathBuf::from("full_files").join(file.path().unwrap());
        diffAndFullFilesZipWriter.start_file(diffAndFullFilesFilePath.to_str().unwrap(), *zipOptions).unwrap();
        diffAndFullFilesZipWriter.write_all(fileContent).unwrap();
    }
}

fn toZipDateTime(dateTime: &OffsetDateTime) -> zip::DateTime
{
    zip::DateTime::from_date_and_time(
        dateTime.year().try_into().unwrap(),
        dateTime.month().into(),
        dateTime.day(),
        dateTime.hour(),
        dateTime.minute(),
        dateTime.second())
        .unwrap()
}
