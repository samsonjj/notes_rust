mod cli;
mod error;
mod repo;
mod shell;

pub use error::NoteError;

use chrono::{DateTime, Local};
use std::path::PathBuf;

use crate::cli::Opts;
use crate::repo::Repo;
use crate::shell::ShellImpl;

type Result<T> = std::result::Result<T, NoteError>;

/// Gets the cli options and config from file, and opens the specified
/// file in the text editor.
pub fn run() -> Result<()> {
    let opts = Opts::load();
    let shell: ShellImpl = ShellImpl::new();
    let repo: Repo = Repo::new(&opts, &shell).init().unwrap();

    // filename as given in args, or the current date
    let filename = match opts.note_name {
        Some(ref path) => path.clone(),
        None => date_filename(Local::now()),
    };

    repo.open_in_editor(&PathBuf::from(&filename));

    Ok(())
}

/// Returns a filename compatible date String.
fn date_filename(now: DateTime<Local>) -> String {
    now.format("%F").to_string() + ".txt"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_date_filename() {
        let date = date_filename(Local::now());
        assert_eq!(date.len(), 14);
    }
}
