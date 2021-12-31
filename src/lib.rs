pub mod prelude {
    pub use super::{
        file,
        read::Read,
        stdio::{self, stderr, stdin, stdout, Stderr, Stdin, Stdout},
        string,
        write::Write,
    };
}

#[macro_use]
mod read;

mod write {
    pub trait Write: std::io::Write {
        fn write(&mut self, s: &str);
    }
}

pub mod file;
pub mod stdio;
pub mod string;
