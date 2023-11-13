use crate::common::{dfxvm_path, Settings};
use directories::ProjectDirs;
use std::env;
use std::fs::create_dir_all;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Mutex;
use tempdir::TempDir;

static HOME_LOCK: Mutex<()> = Mutex::new(());

fn with_home<T, F>(temp_home: &Path, func: F) -> T
where
    F: FnOnce() -> T,
{
    let _lock = HOME_LOCK.lock().unwrap();
    let prev_home = env::var("HOME").unwrap();
    env::set_var("HOME", temp_home);
    let result = func();
    env::set_var("HOME", prev_home);

    result
}

fn project_dirs(home: &Path) -> ProjectDirs {
    with_home(home, || ProjectDirs::from("org", "dfinity", "dfx").unwrap())
}

pub struct TempHomeDir {
    tempdir: TempDir,
    versions_dir: PathBuf,
    config_dir: PathBuf,
}

impl TempHomeDir {
    pub fn new() -> Self {
        let tempdir = TempDir::new("dfxvm-integration-tests-home").unwrap();
        let project_dirs = project_dirs(tempdir.path());
        let versions_dir = project_dirs.data_local_dir().join("versions");
        let config_dir = tempdir.path().join(".config").join("dfx");
        Self {
            tempdir,
            versions_dir,
            config_dir,
        }
    }

    pub fn dfx(&self) -> Command {
        self.command("dfx")
    }

    pub fn dfxvm(&self) -> Command {
        self.command("dfxvm")
    }

    pub fn dfxvm_init(&self) -> Command {
        self.command("dfxvm-init")
    }

    pub fn command(&self, filename: &str) -> Command {
        let path = self.tempdir.path().join(filename);
        std::fs::copy(dfxvm_path(), &path).unwrap();
        let mut command = Command::new(path);
        command.env("HOME", self.tempdir.path());
        command
    }

    pub fn create_executable_dfx_script(&self, version: &str, snippet: &str) -> PathBuf {
        let version = self.versions_dir.join(version);
        create_dir_all(&version).unwrap();
        let bin_path = version.join("dfx");
        let script = format_bash_script(snippet);
        std::fs::write(&bin_path, script).unwrap();
        set_executable(&bin_path);
        bin_path
    }

    pub fn settings(&self) -> Settings {
        Settings::new(self.config_dir.join("version-manager.json"))
    }
}

fn format_bash_script(snippet: &str) -> String {
    format!(
        r#"
#!/usr/bin/env bash

set -e

{snippet}
"#
    )
}

fn set_executable(bin_path: &Path) {
    let mut perms = std::fs::metadata(bin_path).unwrap().permissions();
    perms.set_mode(perms.mode() | 0o500);
    std::fs::set_permissions(bin_path, perms).unwrap();
}