use std::process::Command;
use crate::NoteError;
use std::path::PathBuf;

const SHELL_COMMAND: &str = "sh";
const SHELL_FLAGS: &[&str] = &["-c"];

pub struct Shell {}

impl Shell {
    pub fn new() -> Shell {
        Shell{}
    }
}

impl Shell {
    pub fn execute_in_dir(&self, command: &str, dir: &PathBuf) -> Result<(), NoteError> {
        let mut p = Command::new(SHELL_COMMAND);
        p.args(SHELL_FLAGS);
        p.arg(command);
        p.current_dir(dir);

        let status = p.spawn().unwrap()
            .wait().unwrap();

        if !status.success() {
            return Err(NoteError::Message(
                String::from(format!(
                    "editor process returns failure exit status: {:?}", p
                ))));
        }

        Ok(())
    }
}
