#[macro_use]
extern crate serde_derive;
extern crate confy;
extern crate dirs;

mod cli;
mod config;
mod error;
mod shell;
mod command;

pub use error::NoteError;

use chrono::{DateTime, Local};
use std::fs::{self, File};
use std::path::PathBuf;

use cli::{Cli, CliImpl};
use config::{Config, ConfigImpl};

type Result<T> = std::result::Result<T, NoteError>;

/// Gets the cli options and config from file, and opens the specified
/// file in the text editor.
pub fn run() -> Result<()> {
    // load cli and config
    let cli = CliImpl::load();
    let config = ConfigImpl::load()?;

    init_notes(&config)?;

    // filename as given in args, or the current date
    let filename = match cli.path {
        Some(path) => path,
        None => file_compat_date_str(Local::now()),
    };

    // open the file in text editor
    let full_path = config.repo_path.join(&filename);
    open_in_editor(
        &full_path,
        &config.shell,
        &cli.editor.unwrap_or(config.editor),
    )?;

    Ok(())
}

// Retuns a filename compatible date String.
fn file_compat_date_str(now: DateTime<Local>) -> PathBuf {
    PathBuf::from(now.format("%F").to_string() + ".txt")
}

fn init_notes(config: &ConfigImpl) -> Result<()> {
    fs::create_dir_all(&config.repo_path)?;
    Ok(())
}

/// Open file in editor
/// editor must be a blocking shell command
/// editor must take path as first & only arg
fn open_in_editor(
    path: &std::path::PathBuf,
    shell: &str,
    editor: &str,
) -> Result<()> {
    use std::process::Command;

    File::create(path)?;
    let mut child = Command::new(shell)
        .arg("-c")
        .arg(&format!("{} {:?}", editor, path))
        .spawn()?;

    child.wait()?;

    Ok(())
}
