use crate::NoteError;
use std::path::PathBuf;
use std::process::Command;

const SHELL_COMMAND: &str = "sh";
const SHELL_FLAGS: &[&str] = &["-c"];

pub trait Shell {
    fn execute(
        &self,
        command: &str,
        current_dir: &PathBuf,
    ) -> Result<(), NoteError>;
}

pub struct ShellImpl {}

impl ShellImpl {
    pub fn new() -> ShellImpl {
        ShellImpl {}
    }
}

impl Shell for ShellImpl {
    fn execute(&self, command: &str, dir: &PathBuf) -> Result<(), NoteError> {
        let mut p = Command::new(SHELL_COMMAND);
        p.args(SHELL_FLAGS);
        p.arg(command);
        p.current_dir(dir);

        let exit_status = p.spawn().unwrap().wait().unwrap();

        if !exit_status.success() {
            return Err(NoteError::Message(String::from(format!(
                "editor process returned failure exit status: {:?}", p
            ))));
        }

        Ok(())
    }
}
