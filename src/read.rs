use std::str::FromStr;

/// A trait for objects which can read data.
///
/// Can be turned into an iterator to read all available lines as strings.
pub trait Read: IntoIterator + std::io::Read {
    /// Read and return all available data from self.
    ///
    /// Panics if there is an error reading.
    fn read_all(&mut self) -> String;

    /// Read and return a single line of data from self.
    ///
    /// Panics if there is an error reading. Note that for finite data sources (e.g., files) this
    /// function will panic when the reader is exhausted. To get all lines (if the number of lines
    /// is not known in advance), convert the reader into an iterator instead.
    ///
    /// Does not return the terminal newline character.
    fn read_line(&mut self) -> String;

    /// Read, parse, and return a single line of data from self.
    ///
    /// Panics if there is an error reading or parsing data.
    fn read_line_any<T: FromStr>(&mut self) -> T
    where
        Self: Sized,
    {
        match self.read_line().parse() {
            Ok(t) => t,
            Err(_) => panic!("Could not parse string"),
        }
    }
}

macro_rules! read_into_iter {
    ($t: ty) => {
        impl IntoIterator for $t {
            type Item = String;
            type IntoIter = crate::read::ReadIterator<$t>;

            fn into_iter(self) -> Self::IntoIter {
                crate::read::ReadIterator {
                    reader: std::io::BufReader::new(self),
                }
            }
        }
    };
}

/// An iterator created from an implementer of the `Read` trait.
///
/// Iterates over every line provided by the reader.
pub struct ReadIterator<T: std::io::Read> {
    pub(crate) reader: std::io::BufReader<T>,
}

impl<T: std::io::Read> Iterator for ReadIterator<T> {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        use std::io::BufRead;

        let mut buf = String::new();
        let len = self
            .reader
            .read_line(&mut buf)
            .expect("Failed to read line");
        match len {
            0 => None,
            _ => {
                buf.truncate(buf.len() - 1);
                Some(buf)
            }
        }
    }
}
