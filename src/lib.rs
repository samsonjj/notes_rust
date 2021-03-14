mod cli;
mod error;
mod shell;
mod editor;
mod repo;

pub use error::NoteError;

use chrono::{DateTime, Local};
use std::path::PathBuf;

use crate::cli::Opts;
use crate::shell::Shell;
use crate::editor::Editor;
use crate::repo::Repo;

type Result<T> = std::result::Result<T, NoteError>;

/// Gets the cli options and config from file, and opens the specified
/// file in the text editor.
pub fn run() -> Result<()> {
    let opts = Opts::load();
    let shell: Shell = Shell::new();
    let editor: Editor = Editor::new(&opts, &shell);
    let repo: Repo = Repo::new(&editor).init().unwrap();

    // filename as given in args, or the current date
    let filename = match opts.note_name {
        Some(ref path) => path.clone(),
        None => file_compat_date_str(Local::now()),
    };

    repo.open_in_editor(&PathBuf::from(&filename));

    Ok(())
}

/// Returns a filename compatible date String.
fn file_compat_date_str(now: DateTime<Local>) -> String {
    now.format("%F").to_string() + ".txt"
}
