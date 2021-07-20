use std::path::Path;

pub struct TestResources
{
    configFileGuard: Option<ConfigFileGuard>,
    repoDirGuard: Option<RepoDirGuard>
}

impl TestResources
{
    pub fn new() -> Self
    {
        Self{configFileGuard: None, repoDirGuard: None}
    }

    pub fn withConfig(mut self, guard: ConfigFileGuard) -> Self
    {
        self.configFileGuard = Some(guard);
        self
    }

    pub fn withRepo(mut self, guard: RepoDirGuard) -> Self
    {
        self.repoDirGuard = Some(guard);
        self
    }

    pub fn getConfigFilePath(&self) -> &Path
    {
        self.configFileGuard.as_ref().unwrap().path()
    }

    pub fn getRepoDir(&self) -> &Path
    {
        self.repoDirGuard.as_ref().unwrap().path()
    }
}

type ConfigFileGuard = tempfile::NamedTempFile;
type RepoDirGuard = tempfile::TempDir;
