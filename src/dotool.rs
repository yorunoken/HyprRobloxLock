use std::io::Write;
use std::process::{Command, Stdio};

pub struct DoTool {
    pub child: std::process::Child,
}

impl DoTool {
    pub fn new() -> Result<Self, std::io::Error> {
        let child = Command::new("dotool")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        Ok(DoTool { child })
    }

    pub fn write(&mut self, command: &str) -> Result<(), std::io::Error> {
        println!("{}", self.child.stdin.as_mut().is_none());
        if let Some(childin) = self.child.stdin.as_mut() {
            childin.write_all(format!("{}\n", command).as_bytes())?;
        }

        Ok(())
    }
}
