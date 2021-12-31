use std::str::FromStr;

pub trait Read: IntoIterator + std::io::Read {
    fn read_all(&mut self) -> String;

    fn read_line(&mut self) -> String;

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
