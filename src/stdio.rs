use crate::prelude::*;

pub fn read_line() -> String {
    stdin().read_line()
}

pub fn print(s: &str) {
    stdout().write(s)
}

pub fn stdout() -> Stdout {
    Stdout(std::io::stdout())
}

pub fn stderr() -> Stderr {
    Stderr(std::io::stderr())
}

pub fn stdin() -> Stdin {
    Stdin(std::io::stdin())
}

pub struct Stdout(std::io::Stdout);

pub struct Stderr(std::io::Stderr);

pub struct Stdin(std::io::Stdin);

impl Write for Stdout {
    fn write(&mut self, s: &str) {
        use std::io::Write;

        self.0
            .write_all(s.as_bytes())
            .expect("Failed to write to stdout");
    }
}

impl std::io::Write for Stdout {
    fn write(&mut self, b: &[u8]) -> Result<usize, std::io::Error> {
        self.0.write(b)
    }

    fn flush(&mut self) -> Result<(), std::io::Error> {
        self.0.flush()
    }
}

impl Write for Stderr {
    fn write(&mut self, s: &str) {
        use std::io::Write;

        self.0
            .write_all(s.as_bytes())
            .expect("Failed to write to stdout");
    }
}

impl std::io::Write for Stderr {
    fn write(&mut self, b: &[u8]) -> Result<usize, std::io::Error> {
        self.0.write(b)
    }

    fn flush(&mut self) -> Result<(), std::io::Error> {
        self.0.flush()
    }
}

impl Read for Stdin {
    fn read_all(&mut self) -> String {
        use std::io::Read;

        let mut buf = String::new();
        self.0
            .read_to_string(&mut buf)
            .expect("Failed to read from stdin");
        buf
    }

    fn read_line(&mut self) -> String {
        let mut buf = String::new();
        self.0
            .read_line(&mut buf)
            .expect("Failed to read from stdin");

        if !buf.is_empty() {
            buf.truncate(buf.len() - 1);
        }

        buf
    }
}

impl std::io::Read for Stdin {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(buf)
    }
}

read_into_iter!(Stdin);

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn stdout_write() {
        stdout().write("Hello!\n");
    }

    #[test]
    fn stderr_write() {
        stderr().write("Hello!\n");
    }

    #[test]
    fn stdout_writeln() {
        use std::io::Write;

        let mut out = stdout();
        writeln!(out, "World!").unwrap();
    }
}
