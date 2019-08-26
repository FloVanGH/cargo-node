use std::{io, process};

pub struct Command {
    program: String,
    args: Vec<String>,
    current_dir: Option<String>,
}

impl Command {
    pub fn new(program: impl Into<String>) -> Self {
        Command {
            program: program.into(),
            args: vec![],
            current_dir: None,
        }
    }

    pub fn current_dir(mut self, current_dir: impl Into<String>) -> Self {
        self.current_dir = Some(current_dir.into());
        self
    }

    pub fn arg(mut self, arg: impl Into<String>) -> Self {
        self.args.push(arg.into());
        self
    }

    pub fn exists(&mut self) -> bool {
        if let Ok(out) = self.inner_output(false) {
            if let Some(code) = out.status.code() {
                return code != 101;
            }

            return false;
        }

        false
    }

    fn inner_output(&mut self, out_put: bool) -> io::Result<process::Output> {
        let mut command = if cfg!(target_os = "windows") {
            process::Command::new("cmd")
        } else {
            process::Command::new(self.program.as_str())
        };

        let mut command_ref = {
            let mut command_ref = if cfg!(target_os = "windows") {
                command.arg("/C").arg(self.program.as_str())
            } else {
                &mut command
            };

            if let Some(current_dir) = &self.current_dir {
                command_ref = command_ref.current_dir(current_dir);
            }

            command_ref
        };

        if out_put {
            command_ref = command_ref
                .stderr(process::Stdio::inherit())
                .stdin(process::Stdio::null())
                .stdout(process::Stdio::piped());
        }

        for arg in &self.args {
            command_ref = command_ref.arg(arg)
        }

        command_ref.output()
    }

    pub fn output(&mut self) -> io::Result<process::Output> {
        self.inner_output(true)
    }
}
