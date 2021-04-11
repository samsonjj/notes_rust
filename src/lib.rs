mod error;
mod opts;
mod repo;
mod shell;
mod tutorial;

pub use error::NoteError;

use chrono::{DateTime, Duration, Local};

use crate::opts::Opts;
use crate::repo::Repo;
use crate::shell::ShellImpl;
use crate::tutorial::{Tutorial, TutorialImpl};

type Result<T> = std::result::Result<T, NoteError>;

/// Gets the cli options and config from file, and opens the specified
/// file in the text editor.
pub fn run() -> Result<()> {
    let opts = Opts::load();

    if opts.tutorial {
        TutorialImpl::display();
        return Ok(());
    }

    let shell: ShellImpl = ShellImpl::new()?;
    let repo: Repo = Repo::new(&opts, &shell);

    if let Some(ref remote) = opts.remote_url {
        repo.git_set_remote_origin(remote);
        repo.try_pull();
        return Ok(());
    }

    if opts.doctor {
        repo.doctor();
        return Ok(());
    }

    // filename as given in args, or the current date + offset
    let filename = match opts.note_name {
        Some(ref path) => path.clone(),
        None => {
            let offset = opts.date_offset.unwrap_or(0);
            date_filename(Local::now() + Duration::days(offset))
        }
    };

    repo.try_pull();
    repo.open_in_editor(&filename);
    repo.git_commit_all();
    repo.try_push_in_background();

    Ok(())
}

/// Returns a filename compatible date String.
fn date_filename(now: DateTime<Local>) -> String {
    now.format("%F").to_string() + ".md"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_date_filename() {
        let date = date_filename(Local::now());
        assert_eq!(date.len(), 13);
    }
}
