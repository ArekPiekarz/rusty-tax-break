use crate::commit_diff::{makeCommitSummary, makeFormattedDiff};
use crate::commit_log::{CommitLog, CommitInfo};
use crate::event::Event;
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
use zip::write::FileOptions as ZipFileOptions;
use zip::write::ZipWriter;

const DIFF_FILE_PATH: &str = "changes.diff";


pub struct ReportGenerator
{
    commitLog: Rc<RefCell<CommitLog>>,
    repo: Option<Rc<Repository>>,
    outputPath: Option<PathBuf>
}

impl EventHandler for ReportGenerator
{
    fn handle(&mut self, source: Source, event: &Event)
    {
        match event {
            Event::GenerateReportRequested => self.generateReport(),
            Event::OutputPathChanged(path) => self.onOutputPathChanged(path),
            Event::RepositoryChanged(repo) => self.onRepositoryChanged(repo),
            _ => onUnknown(source, event)
        }
    }
}

impl ReportGenerator
{
    pub fn new(commitLog: Rc<RefCell<CommitLog>>) -> Self
    {
        Self{commitLog, repo: None, outputPath: None}
    }


    // private

    fn onOutputPathChanged(&mut self, path: &Path)
    {
        self.outputPath = Some(path.into());
    }

    fn generateReport(&self)
    {
        let repo = match &self.repo {
            Some(repo) => repo,
            None => { return; }
        };

        let outputPath = match &self.outputPath {
            Some(path) => path,
            None => { return; }
        };

        std::fs::create_dir_all(&outputPath).unwrap();

        for commitInfo in self.commitLog.borrow().getCommits() {
            if !commitInfo.markedForReport {
                continue;
            }
            reportCommit(commitInfo, repo, outputPath);
        }
    }

    fn onRepositoryChanged(&mut self, repo: &Rc<Repository>)
    {
        self.repo = Some(Rc::clone(repo));
    }
}

fn reportCommit(commitInfo: &CommitInfo, repo: &Repository, outputPath: &Path)
{
    let commitId = commitInfo.id;
    let commit = repo.findCommit(commitId).unwrap();
    let commitsDiff = repo.makeDiffOfCommitAndParent(&commit);

    let fullFilesZipName = makeFullFilesZipName(outputPath, commitId);
    let fullFilesZipFile = OpenOptions::new().write(true).create_new(true).open(&fullFilesZipName).unwrap();
    let mut fullFilesZipWriter = ZipWriter::new(fullFilesZipFile);

    let diffAndFullFilesZipName = makeDiffAndFullFilesZipName(outputPath, commitId);
    let diffAndFullFilesZipFile = OpenOptions::new().write(true).create_new(true).open(diffAndFullFilesZipName).unwrap();
    let mut diffAndFullFilesZipWriter = ZipWriter::new(diffAndFullFilesZipFile);

    let zipOptions = ZipFileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    reportDiffFile(&commit, &commitsDiff, &mut diffAndFullFilesZipWriter, &zipOptions);
    reportFullFiles(&commitsDiff, repo, &mut fullFilesZipWriter, &mut diffAndFullFilesZipWriter, &zipOptions);

    fullFilesZipWriter.finish().unwrap();
    diffAndFullFilesZipWriter.finish().unwrap();
}

fn makeFullFilesZipName(outputPath: &Path, commitId: git2::Oid) -> PathBuf
{
    let mut fileName = outputPath.to_owned();
    fileName.push(format!("{}.zip", commitId));
    fileName
}

fn makeDiffAndFullFilesZipName(outputPath: &Path, commitId: git2::Oid) -> PathBuf
{
    let mut fileName = outputPath.to_owned();
    fileName.push(format!("{}-diff.zip", commitId));
    fileName
}

fn reportDiffFile(
    commit: &git2::Commit,
    commitsDiff: &git2::Diff,
    zipWriter: &mut ZipWriter<File>,
    zipOptions: &ZipFileOptions)
{
    zipWriter.start_file(DIFF_FILE_PATH, *zipOptions).unwrap();
    let textDiff = makeCommitSummary(&commit) + &makeFormattedDiff(&commitsDiff);
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
