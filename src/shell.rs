pub struct CommandOutput {
    pub status: i32,
    pub stdout: String,
    pub stderr: String,
}

impl CommandOutput {
    pub fn new(code: i32, stdout: String, stderr: String) -> Self {
        CommandOutput {
            status: code,
            stdout,
            stderr,
        }
    }
}

impl Default for CommandOutput {
    fn default() -> Self {
        CommandOutput::new(0, "".to_string(), "".to_string())
    }
}

/// Execute specified command in user shell, and capture outputs
/// If command succeed, return a CommandOutput
/// If command fail, return a CommandOutput
/// If command cannot be run, return an error
// pub fn command(
//     command: &str,
//     current_dir: &PathBuf,
// ) -> Result<CommandOutput, DefaultError> {
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn new_command_output() {
        let res: CommandOutput =
            CommandOutput::new(5, "out".to_string(), "err".to_string());
        assert_eq!(res.status, 5);
        assert_eq!(res.stdout, "out");
        assert_eq!(res.stderr, "out");
    }

    #[test]
    pub fn correct_command() {}
}
