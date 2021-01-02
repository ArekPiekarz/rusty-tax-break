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
            Event::FolderChosen(folderUri) => self.handleFolderChosen(&folderUri),
            _ => onUnknown(source, event)
        }
    }
}

impl RepositoryStore
{
    pub fn new(sender: Sender) -> Self
    {
        Self{repo: None, sender}
    }


    // private

    fn handleFolderChosen(&mut self, path: &Path)
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
            Err(e) => {
                eprintln!("Failed to open repository at {:?}, cause: {}", path, e);
            }
        }
    }
}
