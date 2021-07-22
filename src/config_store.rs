use crate::config_path::ConfigPath;
use crate::event::{CommitAuthorFilter, Event, OutputPathInfo};
use crate::event_handling::{EventHandler, onUnknown};
use crate::source::Source;
use crate::repository::Repository;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::rc::Rc;


pub struct ConfigStore
{
    config: Config,
    dirPath: PathBuf,
    filePath: PathBuf
}

impl EventHandler for ConfigStore
{
    fn handle(&mut self, source: Source, event: &Event)
    {
        match event {
            Event::CommitAuthorFilterChanged(filter) => self.onCommitAuthorFilterChanged(filter),
            Event::OutputPathChanged(pathInfo)       => self.onOutputPathChanged(pathInfo),
            Event::RepositoryChanged(repo)           => self.onRepositoryChanged(repo),
            Event::WindowMaximized(isMaximized)      => self.onWindowMaximized(*isMaximized),
            _ => onUnknown(source, event)
        }
    }
}

impl ConfigStore
{
    pub fn new(configPath: &ConfigPath) -> Self
    {
        let dirPath = configPath.getDirPath();
        let filePath = configPath.getFilePath();
        let config = toml::from_str(&std::fs::read_to_string(filePath).unwrap_or_default()).unwrap_or_default();
        Self{config, dirPath: dirPath.into(), filePath: filePath.into()}
    }

    pub fn getConfig(&self) -> &Config
    {
        &self.config
    }

    fn onCommitAuthorFilterChanged(&mut self, filter: &CommitAuthorFilter)
    {
        if self.config.commitAuthorFilter == *filter {
            return;
        }
        self.config.commitAuthorFilter = filter.clone();
        self.saveToFile();
    }

    fn onOutputPathChanged(&mut self, pathInfo: &OutputPathInfo)
    {
        if let Some(prefix) = &self.config.outputPathPrefix {
            if *prefix == pathInfo.prefix {
                return;
            }
        }

        self.config.outputPathPrefix = Some(pathInfo.prefix.clone());
        self.saveToFile();
    }

    fn onRepositoryChanged(&mut self, repo: &Rc<Repository>)
    {
        if let Some(repoPath) = &self.config.repository {
            if repoPath == repo.getPath() {
                return;
            }
        }
        self.config.repository = Some(repo.getPath().into());
        self.saveToFile();
    }

    fn onWindowMaximized(&mut self, isMaximized: bool)
    {
        if self.config.isWindowMaximized == isMaximized {
            return;
        }

        self.config.isWindowMaximized = isMaximized;
        self.saveToFile();
    }

    fn saveToFile(&self)
    {
        std::fs::create_dir_all(&self.dirPath).unwrap();
        std::fs::write(&self.filePath, toml::to_string(&self.config).unwrap()).unwrap();
    }
}

#[derive(Default, Deserialize, Serialize)]
pub struct Config
{
    pub commitAuthorFilter: CommitAuthorFilter,
    pub isWindowMaximized: bool,
    pub outputPathPrefix: Option<PathBuf>,
    pub repository: Option<PathBuf>
}
