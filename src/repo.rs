//! houses the Repo struct
use crate::opts::Opts;
use crate::shell::{CommandOutput, Shell};
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
            std::fs::create_dir_all(&self.path).unwrap();
        }
        if !self.path.join(".git").exists() {
            self.shell.execute("git init", &self.path).unwrap();

            // create readme
            self.shell
                .execute(
                    r##"
echo "Notes Repository

See [notes_rust](https://github.com/samsonjj/notes_rust)
" > README.md
"##,
                    &self.path,
                )
                .unwrap();
            self.shell.execute("git add README.md", &self.path).unwrap();
            self.shell
                .execute("git commit -m \"first commit\"", &self.path)
                .unwrap();
        }
        Ok(self)
    }

    pub fn git_commit_all(&self) {
        let commit_message = "update notes";

        self.execute("git add .").unwrap();
        self.execute_interactive(&format!(
            "git commit -m \"{}\"",
            commit_message
        ))
        .unwrap();
    }

    pub fn push(&self) {
        self.execute_interactive("git push").unwrap();
    }

    fn default_path() -> PathBuf {
        let home = std::env::var("HOME")
            .expect("HOME environment variable is not set");
        let path = PathBuf::from(home).join(".notes");
        path
    }

    fn execute(&self, command: &str) -> Result<CommandOutput, NoteError> {
        self.shell.execute(command, &self.path)
    }

    fn execute_interactive(
        &self,
        command: &str,
    ) -> Result<CommandOutput, NoteError> {
        self.shell.execute_interactive(command, &self.path)
    }

    pub fn get_remote_origin_url(&self) -> Option<String> {
        let result = self
            .shell
            .execute("git config --get remote.origin.url", &self.path)
            .unwrap();

        if result.output == "" {
            return None;
        }

        Some(result.output)
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
        println!("opening");
        self.create_nested_file(&self.path.join(filename));

        let editor: &str = match self.opts.editor {
            Some(ref s) => &s[..],
            None => DEFAULT_EDITOR,
        };

        self.execute_interactive(&format!("{} {:?}", editor, filename))
            .unwrap();
        println!("done");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_create_nested_file() {}
}
