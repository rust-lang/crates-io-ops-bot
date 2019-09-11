use std::error::Error;
use std::io::{self, BufRead};
use std::process::*;

pub(crate) struct HerokuCommand {
    output: Output,
}

impl HerokuCommand {
    pub(crate) fn run(
        app_name: &str,
        command: &str,
        args: &[&str],
    ) -> Result<Self, Box<dyn Error>> {
        let output = Command::new("heroku")
            .arg(command)
            .args(args)
            .arg("-a")
            .arg(app_name)
            .output()?;
        Ok(Self { output })
    }

    // fn output_lines(&self) -> impl Stream<Item = Result<String, io::Error>>
    pub(crate) fn output_lines(&self) -> impl Iterator<Item = Result<String, io::Error>> + '_ {
        let stdout_lines = self.output.stdout.lines();
        let stderr_lines = self.output.stderr.lines();
        stdout_lines.chain(stderr_lines)
    }

    // async fn status(&self) -> ExitStatus
    pub(crate) fn status(&self) -> ExitStatus {
        self.output.status
    }
}
