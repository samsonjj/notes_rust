//! houses the Repo struct
use crate::cli::Opts;
use crate::shell::Shell;
use crate::NoteError;
use std::path::PathBuf;

const DEFAULT_EDITOR: &str = "vim";

pub struct Repo<'a> {
    opts: &'a Opts,
    shell: &'a dyn Shell,
    path: PathBuf,
}

// non trait functionality
impl<'a> Repo<'a> {
    pub fn new(opts: &'a Opts, shell: &'a dyn Shell) -> Repo<'a> {
        let path: PathBuf = match &opts.repo_path {
            Some(path) => PathBuf::from(path),
            None => Repo::default_path(),
        };
        Repo { opts, shell, path }
    }

    pub fn init(self) -> Result<Repo<'a>, NoteError> {
        if !self.path.exists() {
            std::fs::create_dir_all(&self.path)?;
        }
        Ok(self)
    }

    fn default_path() -> PathBuf {
        let home = std::env::var("HOME")
            .expect("HOME environment variable is not set");
        let path = PathBuf::from(home).join(".notes");
        path
    }

    fn execute_in_repo(&self, command: &str) -> Result<(), NoteError> {
        self.shell.execute(command, &self.path)
    }

    pub fn create_nested_file(&self, path: &PathBuf) {
        let mut path = path.clone();
        let file_path = path.clone();

        // create any directories, if incoming path is nested
        path.pop();

        std::fs::create_dir_all(self.path.join(path)).unwrap();

        // create file, only if it does not already exist
        // 🌵 otherwise File::create will truncate it
        if !file_path.exists() {
            std::fs::File::create(file_path).unwrap();
        }
    }

    pub fn open_in_editor(&self, filename: &str) {
        self.create_nested_file(&self.path.join(filename));

        let editor: &str = match self.opts.editor {
            Some(ref s) => &s[..],
            None => DEFAULT_EDITOR,
        };

        self.execute_in_repo(&format!("{} {:?}", editor, filename)).unwrap();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_create_nested_file() {
    }
}
