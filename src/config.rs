//! Provides some of the data needed to run the notes_rust application.
//! Intended to be loaded by confy, and then overriden with arguments
//! from the command line.
//!
//! Accepted environment variables:
//! - NOTES_STORAGE_DIRECTORY
use confy;
use std::env;
use std::path::PathBuf;
use crate::error::NoteError;

const ENV_REPO_DIR: &str = "NOTES_STORAGE_DIRECTORY";
const ENV_CONFIG_PATH: &str = "NOTES_CONFIG_PATH";
const ENV_EDITOR: &str = "NOTES_EDITOR";
const ENV_SHELL: &str = "NOTES_SHELL";

type Result<T> = std::result::Result<T, NoteError>;

/*
    Config and ConfigImpl
*/

pub trait Config<T> {
    fn load() -> Result<T>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigImpl {
    pub editor: String,
    pub repo_path: std::path::PathBuf,
    pub shell: String,
}

impl Config<ConfigImpl> for ConfigImpl {
    fn load() -> Result<Self> {
        let mut config: ConfigImpl = confy::load_path(config_path())?;

        config.editor = env::var(ENV_EDITOR)
            .unwrap_or(config.editor);
        config.repo_path = env::var(ENV_REPO_DIR).map(PathBuf::from)
            .unwrap_or(config.repo_path);
        config.shell = env::var(ENV_SHELL)
            .unwrap_or(String::from("sh"));

        Ok(config)
    }
}

impl Default for ConfigImpl {
    fn default() -> Self {
        Self {
            editor: default_editor(),
            repo_path: default_repo_directory(),
            shell: default_shell(),
        }
    }
}


/*
    helper functions
*/

const DEFAULT_CONFIG_FILE_NAME: &str    = "config.toml";
const DEFAULT_REPO_NAME: &str           = "notes_repo";
const DEFAULT_EDITOR: &str              = "vim";
const DEFAULT_SHELL: &str               = "sh";

fn default_editor() -> String {
    String::from(DEFAULT_EDITOR)
}

fn default_repo_directory() -> PathBuf {
    match dirs::home_dir() {
        Some(home) => home.join(".notes").join(DEFAULT_REPO_NAME),
        None => "/tmp".into(),
    }
}

fn default_shell() -> String {
    DEFAULT_SHELL.into()
}

fn config_path() -> PathBuf {
    if let Ok(path) = env::var(ENV_CONFIG_PATH) { 
        return path.into();
    }
    if let Some(home) = dirs::home_dir() {
        return home.join(".notes").join(DEFAULT_CONFIG_FILE_NAME);
    }

    PathBuf::from("/tmp").join(DEFAULT_CONFIG_FILE_NAME)
}

