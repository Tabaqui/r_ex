use std::{
    error::Error,
    fmt::Display,
    process::{Child, ChildStdout, Command, Output, Stdio},
};

const CMD_ERR_MSG: &str = "Command error!";

pub struct CmdGroup {
    cmd: String,
    args: Option<String>,
}

impl CmdGroup {
    pub fn new(value: String) -> Result<Self, CmdError> {
        let mut cmd_array = value.split_ascii_whitespace();
        let cmd = cmd_array.next().ok_or(CmdError::NotRecognized)?;
        let args = cmd_array.next().map(|s| s.to_string());
        Ok(Self {
            cmd: cmd.to_string(),
            args,
        })
    }

    fn spawn(cmd: String, args: Option<String>, stdin: Stdio) -> Result<Child, CmdError> {
        Command::new(cmd)
            .args(args)
            .stdout(Stdio::piped())
            .stdin(stdin)
            .spawn()
            .map_err(|_| CmdError::Command)

    }

    pub fn run_out(self, stdin: Stdio) -> Result<ChildStdout, CmdError> {
        CmdGroup::spawn(self.cmd, self.args, stdin)?
            .stdout
            .ok_or(CmdError::Out)
    }

    pub fn run_wait(self, stdin: Stdio) -> Result<Output, CmdError> {
        CmdGroup::spawn(self.cmd, self.args, stdin)?
            .wait_with_output()
            .map_err(|_| CmdError::Wait)
    }
}

#[derive(Debug)]
pub enum CmdError {
    NotRecognized,
    Wait,
    Out,
    Command
}

impl Display for CmdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{CMD_ERR_MSG}")
    }
}

impl Error for CmdError {}

