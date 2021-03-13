mod cli;
mod error;
mod shell;
mod editor;
mod repo;

pub use error::NoteError;

use chrono::{DateTime, Local};
use std::path::PathBuf;

use cli::{Cli, CliImpl};
use crate::shell::Shell;
use crate::editor::Editor;

type Result<T> = std::result::Result<T, NoteError>;

/// Gets the cli options and config from file, and opens the specified
/// file in the text editor.
pub fn run() -> Result<()> {
    let cli = CliImpl::load();
    let preferred_editor: &str = cli.editor.as_ref().map(|s| s.as_str()).unwrap_or("vim");
    let shell: Shell = Shell::new();
    let editor: Editor = Editor::new(preferred_editor, shell);

    // filename as given in args, or the current date
    let filename = match cli.note_name {
        Some(path) =>  path,
        None => file_compat_date_str(Local::now()),
    };

    editor.open_file_in_repo(&PathBuf::from(&filename))?;

    Ok(())
}

/// Returns a filename compatible date String.
fn file_compat_date_str(now: DateTime<Local>) -> String {
    now.format("%F").to_string() + ".txt"
}
