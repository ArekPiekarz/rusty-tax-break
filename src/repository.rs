use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::path::Path;
use std::path::PathBuf;


pub struct Repository
{
    repo: git2::Repository,
    path: PathBuf
}

impl Repository
{
    pub fn new(repo: git2::Repository, path: PathBuf) -> Self
    {
        Self{repo, path}
    }

    pub fn getPath(&self) -> &Path
    {
        &self.path
    }

    pub fn isEmpty(&self) -> bool
    {
        self.repo.is_empty().unwrap()
    }

    pub fn iterateCommits(&self, mut handler: impl FnMut(&git2::Commit))
    {
        let mut revwalk = self.repo.revwalk().unwrap();
        revwalk.push_head().unwrap();
        revwalk.simplify_first_parent().unwrap();
        revwalk.set_sorting(git2::Sort::TIME).unwrap();
        for oid in revwalk {
            let commit = self.repo.find_commit(oid.unwrap()).unwrap();
            handler(&commit);
        }
    }

    pub fn findCommit(&self, id: git2::Oid) -> Result<git2::Commit, git2::Error>
    {
        self.repo.find_commit(id)
    }

    pub fn findBlob(&self, id: git2::Oid) -> git2::Blob
    {
        self.repo.find_blob(id).unwrap()
    }

    pub fn makeDiffOfCommitAndParent(&self, commit: &git2::Commit) -> git2::Diff
    {
        let tree = commit.tree().unwrap();
        let parentTreeOpt = findTreeOfParentOfCommit(&commit);
        let mut diffOptions = makeDiffOptions();
        self.repo.diff_tree_to_tree(parentTreeOpt.as_ref(), Some(&tree), Some(&mut diffOptions)).unwrap()
    }
}

fn findTreeOfParentOfCommit<'a>(commit: &git2::Commit<'a>) -> Option<git2::Tree<'a>>
{
    match commit.parent(0) {
        Ok(parentCommit) => {
            match parentCommit.tree() {
                Ok(tree) => Some(tree),
                Err(e) => panic!("findTreeOfParentOfCommit failed: {:?}", e)
            }
        },
        Err(e) if e.class() == git2::ErrorClass::Invalid && e.code() == git2::ErrorCode::NotFound => None,
        Err(e) => panic!("findTreeOfParentOfCommit failed: {:?}", e)
    }
}

fn makeDiffOptions() -> git2::DiffOptions
{
    let mut diffOptions = git2::DiffOptions::new();
    diffOptions.indent_heuristic(true);
    diffOptions
}

impl Debug for Repository
{
    fn fmt(&self, formatter: &mut Formatter) -> FmtResult
    {
        write!(formatter, "Repository {{ path: {:?} }}", self.repo.path())
    }
}
