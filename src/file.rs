use crate::prelude::*;
use std::path::Path;

pub fn reader(path: impl AsRef<Path>) -> Reader {
    Reader(std::io::BufReader::new(
        std::fs::File::open(path).expect("Couldn't open file"),
    ))
}

pub fn writer(path: impl AsRef<Path>) -> Writer {
    Writer(std::fs::File::create(path).expect("Couldn't create file"))
}

pub fn read(path: impl AsRef<Path>) -> String {
    reader(path).read_all()
}

pub fn write(path: impl AsRef<Path>, s: &str) {
    writer(path).write(s)
}

pub struct Writer(std::fs::File);
pub struct Reader(std::io::BufReader<std::fs::File>);

impl Write for Writer {
    fn write(&mut self, s: &str) {
        use std::io::Write;

        self.0
            .write_all(s.as_bytes())
            .expect("Failed to write to stdout");
    }
}

impl std::io::Write for Writer {
    fn write(&mut self, b: &[u8]) -> Result<usize, std::io::Error> {
        self.0.write(b)
    }

    fn flush(&mut self) -> Result<(), std::io::Error> {
        self.0.flush()
    }
}

impl Read for Reader {
    fn read_all(&mut self) -> String {
        use std::io::Read;

        let mut buf = String::new();
        self.0
            .read_to_string(&mut buf)
            .expect("Failed to read from stdin");
        buf
    }

    fn read_line(&mut self) -> String {
        use std::io::BufRead;

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

impl std::io::Read for Reader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(buf)
    }
}

read_into_iter!(Reader);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn write_read() {
        write("test.out", "line 1\nline 2 \nline 3\n");
        assert_eq!(read("test.out"), "line 1\nline 2 \nline 3\n");

        let mut reader = reader("test.out");
        assert_eq!(reader.read_line(), "line 1");
        assert_eq!(reader.read_line(), "line 2 ");
        assert_eq!(reader.read_line(), "line 3");
    }
}
