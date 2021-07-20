use std::path::{Path, PathBuf};


pub struct ConfigPath
{
    dirPath: PathBuf,
    filePath: PathBuf
}

impl ConfigPath
{
    pub fn new(filePath: &Path) -> Self
    {
        if filePath.is_dir() {
            panic!("The provided config file path leads to a directory: {:?}", filePath);
        }
        Self{dirPath: filePath.parent().unwrap().into(), filePath: filePath.into()}
    }

    pub fn getDirPath(&self) -> &Path
    {
        &self.dirPath
    }

    pub fn getFilePath(&self) -> &Path
    {
        &self.filePath
    }
}

impl Default for ConfigPath
{
    fn default() -> Self
    {
        let mut dirPath = dirs::config_dir().unwrap();
        dirPath.push("rusty-tax-break");
        let mut filePath = dirPath.clone();
        filePath.push("config.toml");
        Self{dirPath, filePath}
    }
}
