//! houses the Repo struct
use crate::opts::Opts;
use crate::shell::{CommandOutput, Shell};
use crate::NoteError;
use std::collections::HashMap;
use std::path::PathBuf;

const DEFAULT_EDITOR: &str = "vim";

const REMOTE: &str = "origin";
const BRANCH: &str = "main";

const MARKER_FILENAME: &str = "notes-repo-marker";

pub struct Repo<'a> {
    opts: &'a Opts,
    shell: &'a dyn Shell,
    path: PathBuf,
}

// non trait functionality
impl<'a> Repo<'a> {
    pub fn try_new(
        opts: &'a Opts,
        shell: &'a dyn Shell,
    ) -> Result<Repo<'a>, NoteError> {
        let path: PathBuf = match &opts.repo_path {
            Some(path) => PathBuf::from(path),
            None => Repo::default_path(),
        };

        Ok(Repo { opts, shell, path }.init()?)
    }

    fn init(self) -> Result<Repo<'a>, NoteError> {
        if !self.path.exists() {
            std::fs::create_dir_all(&self.path).unwrap();
        }
        if self.path.join(".git").exists() {
            // already a git repo
            if !self.path.join(MARKER_FILENAME).exists() {
                // we're in a git repo that is not a notes repo
                // !!!dangerous!!!
                return Err(NoteError::Message(format!("The given repo path is a git repository, but is missing the `{}` file, meaning you may be trying to write notes in an uninteded directory. If you really want to store notes there, run `touch .notes-repo-marker` in that directory.", MARKER_FILENAME)));
            }
        } else {
            // git init
            self.shell.execute("git init", &self.path)?;
            // create marker file, so we will know in the future if this folder is intended for notes
            self.shell.execute(
                format!("touch {}", MARKER_FILENAME).as_str(),
                &self.path,
            )?;
            // create readme
            if !self.path.join("README.md").exists() {
                self.shell.execute(
                    r##"
echo "Notes Repository

See [notes_rust](https://github.com/samsonjj/notes_rust)
" > README.md
"##,
                    &self.path,
                )?;
            }
            self.execute("git add README.md")?;
            self.execute("git commit -m \"first commit\"")?;
            self.execute(&format!("git branch -M {}", BRANCH))?;
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

    pub fn try_pull(&self) {
        if let Some(_) = self.get_remote_origin_url() {
            self.execute_interactive(&format!(
                "git pull {} {}",
                REMOTE, BRANCH
            ))
            .unwrap();
        }
    }

    pub fn try_push_in_background(&self) {
        if let Some(_) = self.get_remote_origin_url() {
            self.execute_interactive(&format!(
                "git push -u {} {} > /dev/null 2>&1 &",
                REMOTE, BRANCH
            ))
            .unwrap();
        }
    }

    pub fn git_set_remote_origin(&self, url: &str) {
        if let Some(_) = self.get_remote_origin_url() {
            self.execute_interactive(&format!("git remote remove {}", REMOTE))
                .unwrap();
        }
        self.execute_interactive(&format!("git remote add {} {}", REMOTE, url))
            .unwrap();
        self.execute_interactive(&format!("git fetch")).unwrap();
        self.execute_interactive(&format!(
            "git branch --set-upstream-to={}/{} {}",
            REMOTE, BRANCH, BRANCH
        ))
        .unwrap();
        self.try_pull();
        self.try_push_in_background();
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
            .execute(
                &format!("git config --get remote.{}.url", REMOTE),
                &self.path,
            )
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
        self.create_nested_file(&self.path.join(filename));

        let editor: &str = match self.opts.editor {
            Some(ref s) => &s[..],
            None => DEFAULT_EDITOR,
        };

        self.execute_interactive(&format!("{} {:?}", editor, filename))
            .unwrap();
    }

    pub fn doctor(&self) {
        let file_names = std::fs::read_dir(&self.path).unwrap();
        let mut unique: HashMap<String, Vec<String>> = HashMap::new();

        file_names
            .into_iter()
            .map(|os_string| {
                os_string.unwrap().file_name().into_string().unwrap()
            })
            .for_each(|string| {
                let v = string
                    .split(".")
                    .into_iter()
                    .map(move |x| String::from(x))
                    .collect::<Vec<_>>();

                let vec: Vec<String> = Vec::new();
                let file_extension_list =
                    unique.entry(v[0].clone()).or_insert(vec);
                file_extension_list.push(v[1].clone());
            });

        unique.into_iter().for_each(|entry| {
            if entry.1.len() > 1 {
                println!(
                    "Duplicate file prefix: {}.{:?}",
                    entry.0,
                    entry.1.join(",")
                )
            }
        });
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_create_nested_file() {}
}
