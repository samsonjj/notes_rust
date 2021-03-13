use std::path::PathBuf;
use crate::shell::Shell;
use crate::NoteError;

pub struct Editor<'a> {
    pub command: &'a str,
    pub shell: Shell,
}

impl<'a> Editor<'a> {
    pub fn new(command: &str, shell: Shell) -> Editor {
        Editor{ command, shell }
    }

    pub fn open_file_in_repo(&self, path: &PathBuf) -> Result<(), NoteError>{
        let mut path = path.clone();
        let filename = path.clone();

        // create any directories, if incoming path is nested
        path.pop();
        let directory_path = path;
        if directory_path.as_os_str().len() != 0 {
            self.shell.execute_in_repo(&*format!("mkdir -p {:?}", directory_path))?;
        }

        // open file
        self.shell.execute_in_repo(
            &format!("{} {:?}", self.command, filename)
        )?;

        Ok(())
    }
}
