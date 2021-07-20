use crate::config_path::ConfigPath;
use crate::event::{Event, OutputPathInfo};
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
            Event::OutputPathChanged(pathInfo) => self.onOutputPathChanged(pathInfo),
            Event::RepositoryChanged(repo)     => self.onRepositoryChanged(repo),
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
        let config = toml::from_str(&std::fs::read_to_string(filePath).unwrap_or_default()).unwrap();
        Self{config, dirPath: dirPath.into(), filePath: filePath.into()}
    }

    pub fn getConfig(&self) -> &Config
    {
        &self.config
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
        self.config.repository = Some(repo.getPath().into());
        self.saveToFile();
    }

    fn saveToFile(&self)
    {
        std::fs::create_dir_all(&self.dirPath).unwrap();
        std::fs::write(&self.filePath, toml::to_string(&self.config).unwrap()).unwrap();
    }
}

#[derive(Deserialize, Serialize)]
pub struct Config
{
    pub repository: Option<PathBuf>,
    pub outputPathPrefix: Option<PathBuf>
}
