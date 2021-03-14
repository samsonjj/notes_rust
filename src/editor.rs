use std::path::PathBuf;
use crate::shell::Shell;
use crate::NoteError;
use crate::cli::Opts;

pub struct Editor<'a> {
    pub opts: &'a Opts,
    pub shell: &'a Shell,
}

impl<'a> Editor<'a> {
    pub fn new(opts: &'a Opts, shell: &'a Shell) -> Editor<'a> {
        Editor{
            opts,
            shell,
        }
    }

    pub fn open_file(&self, path: &PathBuf, current_dir: &PathBuf) -> Result<(), NoteError>{
        let editor_command: &str = self.opts.editor
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("vim");

        // open file
        self.shell.execute_in_dir(
            &format!("{} {:?}", editor_command, path),
            current_dir,
        )?;

        Ok(())
    }
}
