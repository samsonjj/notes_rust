#[macro_use]
extern crate serde_derive;
extern crate confy;
extern crate dirs;

mod cli;
mod config;
mod error;
mod shell;

pub use error::NoteError;

use chrono::{DateTime, Utc};
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

    // filename as given in args, or the current date
    let filename = match cli.path {
        Some(path) => path,
        None => file_compat_date_str(Utc::now()),
    };

    // open the file in text editor
    let full_path = config.repo_path.join(&filename);
    open_in_editor(&full_path, &config.editor)?;
    println!("finished editing {:?}", full_path);

    Ok(())
}

// Retuns a filename compatible date String.
fn file_compat_date_str(now: DateTime<Utc>) -> PathBuf {
    PathBuf::from(now.format("%F").to_string() + ".txt")
}

/// Open file in editor
/// editor must be a blocking shell command
/// editor must take path as first & only arg
fn open_in_editor(path: &std::path::PathBuf, editor: &str) -> Result<()> {
    use std::process::Command;
    let mut child = Command::new(editor).arg(path).spawn()?;

    child.wait()?;

    Ok(())
}
