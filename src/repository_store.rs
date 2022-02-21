use crate::config_store::Config;
use crate::event::Event;
use crate::event_handling::{EventHandler, onUnknown, Sender};
use crate::repository::Repository;
use crate::source::Source;

use std::path::Path;
use std::rc::Rc;


pub struct RepositoryStore
{
    repo: Option<Rc<Repository>>,
    sender: Sender
}

impl EventHandler for RepositoryStore
{
    fn handle(&mut self, source: Source, event: &Event)
    {
        match event {
            Event::FolderChosen(path) => self.onFolderChosen(path),
            _ => onUnknown(source, event)
        }
    }
}

impl RepositoryStore
{
    pub fn new(config: &Config, sender: Sender) -> Self
    {
        match &config.repository {
            Some(path) => Self::newFromPath(path, sender),
            None => Self{repo: None, sender}
        }
    }

    pub fn getRepository(&self) -> &Option<Rc<Repository>>
    {
        &self.repo
    }

    pub fn getRepositoryPath(&self) -> Option<&Path>
    {
        match &self.repo {
            Some(repo) => Some(repo.getPath()),
            None => None
        }
    }

    // private

    fn newFromPath(path: &Path, sender: Sender) -> Self
    {
        match git2::Repository::open(path) {
            Ok(gitRepo) => {
                Self{repo: Some(Rc::new(Repository::new(gitRepo, path.into()))), sender}
            },
            Err(error) => {
                warnRepoFailedToOpen(path, error);
                Self{repo: None, sender}
            }
        }
    }

    fn onFolderChosen(&mut self, path: &Path)
    {
        if let Some(repo) = &self.repo {
            if repo.getPath() == path {
                eprintln!("Repository path was already known: {:?}", path);
                return;
            }
        }

        match git2::Repository::open(path) {
            Ok(gitRepo) => {
                let repository = Rc::new(Repository::new(gitRepo, path.into()));
                self.repo = Some(Rc::clone(&repository));
                self.sender.send((Source::RepositoryStore, Event::RepositoryChanged(repository))).unwrap();
            },
            Err(error) => {
                warnRepoFailedToOpen(path, error);
            }
        }
    }
}

fn warnRepoFailedToOpen(path: &Path, error: git2::Error)
{
    eprintln!("Failed to open repository at {:?}, cause: {}", path, error);
}
