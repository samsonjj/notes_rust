use std::process::Command;
use crate::NoteError;
use crate::repo::Repo;

pub struct Shell {

}

impl Shell {
    pub fn new() -> Shell { Shell{} }
}

impl Shell {
    pub fn execute_in_repo(&self, command: &str) -> Result<(), NoteError> {
        let mut repo = Repo::new();

        let mut p = Command::new("sh");
        p.args(&["-c", command]);
        p.current_dir(repo.get_path());

        let status = p.spawn().expect("failed to spawn process")
            .wait().expect("error opening in editor");

        if !status.success() {
            return Err(NoteError::Message(
                String::from(format!(
                    "editor process returns failure exit status: {:?}", p
                ))));
        }

        Ok(())
    }
}
