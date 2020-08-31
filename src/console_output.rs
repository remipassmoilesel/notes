use crate::shell::CommandOutput;

#[derive(Debug)]
pub struct ConsoleOutput {
    pub stdout: String,
    pub stderr: String,
}

impl ConsoleOutput {
    pub fn new(out: String, err: String) -> Self {
        ConsoleOutput { stdout: out, stderr: err }
    }

    pub fn empty() -> Self {
        ConsoleOutput {
            stdout: String::from(""),
            stderr: String::from(""),
        }
    }

    pub fn from_stdout(out: &str) -> Self {
        ConsoleOutput {
            stdout: out.to_string(),
            stderr: String::from(""),
        }
    }

    pub fn from_stderr(out: &str) -> Self {
        ConsoleOutput {
            stdout: String::from(""),
            stderr: out.to_string(),
        }
    }

    pub fn append(&mut self, out: ConsoleOutput) {
        self.append_stdout(out.stdout.as_str());
        self.append_stderr(out.stderr.as_str());
    }

    pub fn append_command_output(&mut self, out: CommandOutput) {
        self.append_stdout(out.stdout.as_str());
        self.append_stderr(out.stderr.as_str());
    }

    pub fn append_stdout(&mut self, out: &str) {
        self.stdout.push_str(out);
    }

    pub fn append_stderr(&mut self, out: &str) {
        self.stderr.push_str(out);
    }
}

impl Default for ConsoleOutput {
    fn default() -> Self {
        ConsoleOutput::empty()
    }
}

impl From<CommandOutput> for ConsoleOutput {
    fn from(o: CommandOutput) -> Self {
        ConsoleOutput {
            stderr: o.stderr,
            stdout: o.stdout,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let res = ConsoleOutput::new("out".to_string(), "err".to_string());
        assert_eq!(res.stdout, "out");
        assert_eq!(res.stderr, "err");
    }

    #[test]
    fn empty() {
        let res = ConsoleOutput::empty();
        assert_eq!(res.stdout, "");
        assert_eq!(res.stderr, "");
    }

    #[test]
    fn from_stdout() {
        let res = ConsoleOutput::from_stdout("test");
        assert_eq!(res.stdout, "test");
        assert_eq!(res.stderr, "");
    }

    #[test]
    fn from_stderr() {
        let res = ConsoleOutput::from_stderr("test");
        assert_eq!(res.stdout, "");
        assert_eq!(res.stderr, "test");
    }

    #[test]
    pub fn append() {
        let mut out_a = ConsoleOutput::new("out_a\n".to_string(), "err_a\n".to_string());
        let out_b = ConsoleOutput::new("out_b\n".to_string(), "err_b\n".to_string());
        out_a.append(out_b);

        assert_eq!(out_a.stdout, "out_a\nout_b\n");
        assert_eq!(out_a.stderr, "err_a\nerr_b\n");
    }

    #[test]
    pub fn append_command_output() {
        let mut out_a = ConsoleOutput::new("out_a\n".to_string(), "err_a\n".to_string());
        let out_b = CommandOutput::new(2, "out_b\n".to_string(), "err_b\n".to_string());
        out_a.append_command_output(out_b);

        assert_eq!(out_a.stdout, "out_a\nout_b\n");
        assert_eq!(out_a.stderr, "err_a\nerr_b\n");
    }

    #[test]
    pub fn append_stdout() {
        let mut out_a = ConsoleOutput::new("out_a\n".to_string(), "err_a\n".to_string());
        out_a.append_stdout("test\n");

        assert_eq!(out_a.stdout, "out_a\ntest\n");
        assert_eq!(out_a.stderr, "err_a\n");
    }

    #[test]
    pub fn append_stderr() {
        let mut out_a = ConsoleOutput::new("out_a\n".to_string(), "err_a\n".to_string());
        out_a.append_stderr("test\n");

        assert_eq!(out_a.stdout, "out_a\n");
        assert_eq!(out_a.stderr, "err_a\ntest\n");
    }

    #[test]
    fn from_command_ouput() {
        let out_b = CommandOutput::new(2, "out_b\n".to_string(), "err_b\n".to_string());
        let out = ConsoleOutput::from(out_b);

        assert_eq!(out.stdout, "out_b\n");
        assert_eq!(out.stderr, "err_b\n");
    }
}
