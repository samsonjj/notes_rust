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
    ) -> Result<CommandOutput, NoteError>;

    fn execute_interactive(
        &self,
        command: &str,
        current_dir: &PathBuf,
    ) -> Result<CommandOutput, NoteError>;
}

pub struct ShellImpl {}

pub struct CommandOutput {
    pub status: i32,
    pub output: String,
    pub error: String,
}

impl CommandOutput {
    pub fn new(code: i32, output: String, error: String) -> CommandOutput {
        CommandOutput {
            status: code,
            output,
            error,
        }
    }
}

impl ShellImpl {
    pub fn new() -> ShellImpl {
        ShellImpl {}
    }
}

impl Shell for ShellImpl {
    fn execute(
        &self,
        command: &str,
        dir: &PathBuf,
    ) -> Result<CommandOutput, NoteError> {
        let child = Command::new(SHELL_COMMAND)
            .args(SHELL_FLAGS)
            .arg(command)
            .current_dir(dir)
            .output()
            .unwrap();

        println!("done?");

        Ok(CommandOutput::new(
            child.status.code().unwrap_or(-1),
            std::str::from_utf8(&child.stdout[..]).unwrap().to_string(),
            std::str::from_utf8(&child.stderr[..]).unwrap().to_string(),
        ))
    }

    fn execute_interactive(
        &self,
        command: &str,
        dir: &PathBuf,
    ) -> Result<CommandOutput, NoteError> {
        let mut p = Command::new(SHELL_COMMAND);
        p.args(SHELL_FLAGS);
        p.arg(command);
        p.current_dir(dir);

        let exit_status = p.spawn().unwrap().wait().unwrap();

        if !exit_status.success() {
            return Err(NoteError::Message(String::from(format!(
                "editor process returned failure exit status: {:?}",
                p
            ))));
        }

        Ok(CommandOutput::new(
            exit_status.code().unwrap_or(-1),
            String::new(),
            String::new(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute() -> Result<(), NoteError> {
        let shell = ShellImpl::new();
        let command_output =
            shell.execute("echo hello", &std::env::current_dir()?)?;

        assert_eq!(command_output.output, "hello\n");
        assert_eq!(command_output.error, "");
        assert_eq!(command_output.status, 0);

        Ok(())
    }

    #[test]
    fn test_execute_interactive() -> Result<(), NoteError> {
        let shell = ShellImpl::new();
        let command_output = shell
            .execute_interactive("echo hello", &std::env::current_dir()?)?;

        assert_eq!(command_output.output, "");
        assert_eq!(command_output.error, "");
        assert_eq!(command_output.status, 0);

        Ok(())
    }
}
