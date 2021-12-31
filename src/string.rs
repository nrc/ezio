use crate::prelude::*;

pub fn reader(s: &str) -> Reader<'_> {
    Reader(s)
}

pub fn writer() -> Writer {
    Writer(String::new())
}

#[derive(Clone, Debug)]
pub struct Reader<'a>(&'a str);
#[derive(Clone, Debug)]
pub struct Writer(String);

impl Into<String> for Writer {
    fn into(self) -> String {
        self.0
    }
}

impl AsRef<str> for Writer {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Write for Writer {
    fn write(&mut self, s: &str) {
        self.0.push_str(s);
    }
}

impl std::io::Write for Writer {
    fn write(&mut self, b: &[u8]) -> Result<usize, std::io::Error> {
        let s = std::str::from_utf8(b).expect("Tried to write non-utf8 bytes to a string");
        self.0.push_str(s);
        Ok(b.len())
    }

    fn flush(&mut self) -> Result<(), std::io::Error> {
        Ok(())
    }
}

impl<'a> From<&'a str> for Reader<'a> {
    fn from(s: &'a str) -> Reader<'a> {
        Reader(s)
    }
}

impl<'a> Read for Reader<'a> {
    fn read_all(&mut self) -> String {
        self.0.to_owned()
    }

    fn read_line(&mut self) -> String {
        let (line, rest) = match self.0.find('\n') {
            Some(n) => {
                let (line, rest) = self.0.split_at(n);
                (line, &rest[1..])
            }
            None => (self.0, ""),
        };

        self.0 = rest;
        line.to_owned()
    }
}

impl<'a> std::io::Read for Reader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let len = std::cmp::min(self.0.len(), buf.len());
        let bytes = &self.0.as_bytes()[..len];
        buf.copy_from_slice(bytes);
        Ok(len)
    }
}

pub struct StrIterReader<'a>(std::str::Lines<'a>);

impl<'a> IntoIterator for Reader<'a> {
    type Item = String;
    type IntoIter = StrIterReader<'a>;

    fn into_iter(self) -> Self::IntoIter {
        StrIterReader(self.0.lines())
    }
}

impl<'a> Iterator for StrIterReader<'a> {
    // We could return &'a str, but we use String for consistency with the rest of the crate.
    type Item = String;

    fn next(&mut self) -> Option<String> {
        self.0.next().map(|s| s.to_owned())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn str_write() {
        let mut writer = writer();
        writer.write("Hello, ");
        writer.write("world!");
        assert_eq!(writer.as_ref(), "Hello, world!");
    }

    #[test]
    fn str_write_any() {
        let mut writer = writer();
        writer.write_any("Hello ");
        writer.write_any(42);
        writer.write_any(String::new());
        assert_eq!(writer.as_ref(), "Hello 42");
    }

    #[test]
    fn str_read() {
        let mut reader = reader("line 1\n line 2");
        assert_eq!(reader.read_all(), "line 1\n line 2");
        assert_eq!(reader.read_line(), "line 1");
        assert_eq!(reader.read_line(), " line 2");

        let mut reader: Reader = "".into();
        assert_eq!(reader.read_line(), "");
    }

    #[test]
    fn str_read_any() {
        let mut reader = reader("line 1\n42\n3.14");
        assert_eq!(&reader.read_line_any::<String>(), "line 1");
        assert_eq!(reader.read_line_any::<u8>(), 42);
        assert_eq!(reader.read_line_any::<f64>(), 3.14);
    }

    #[test]
    fn str_iter() {
        let reader = reader("line 0\nline 1");
        for (i, line) in reader.into_iter().enumerate() {
            assert_eq!(line, format!("line {}", i));
        }
    }
}
