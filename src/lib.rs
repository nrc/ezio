//! ezio - an easy IO library for Rust
//!
//! ezio offers an easy to use IO API for reading and writing to files and stdio.
//! Performance and idiomatic error handling are explicit non-goals, so ezio is
//! probably not suitable for production use. It is better suited for education,
//! experimentation, and prototyping.
//!
//! ezio wraps the standard library's IO APIs and is designed to interoperate with
//! them, so ezio should be compatible with most upstream libraries.
//!
//! The easiest way to use ezio is to include the prelude:
//!
//! ```
//! use ezio::prelude::*;
//! ```
//!
//! You can then either use reader and writer objects, or read/write free functions,
//! each are defined in the modules for specific IO kinds.
//!
//! ezio has its own [`Read`] and [`Write`] traits which you can use for generic
//! programming.
//!
//! ## Examples
//!
//! ```no_run
//! use ezio::prelude::*;
//!
//! fn main() {
//!     // Read a line from stdin
//!     let _ = stdio::read_line();
//!
//!     // Iterate lines in a file
//!     for line in file::reader("path/to/file.txt") {
//!         // ...
//!     }
//!
//!     // Read a while file
//!     let _ = file::read("path/to/file.txt");
//!
//!     // Write to a file
//!     file::write("path/to/file.txt", "Some text");
//!
//!     // Write multiple things to a file
//!     let mut w = file::writer("path/to/file.txt");
//!     w.write("Some text\n");
//!     w.write("Some more text");
//! }
//! ```

/// Re-exports of ezio's modules, traits, and some functions and types.
///
/// Import using:
///
/// ```
/// use ezio::prelude::*;
/// ```
pub mod prelude {
    pub use super::{
        file,
        read::Read,
        stdio::{self, stderr, stdin, stdout, Stderr, Stdin, Stdout},
        string,
        write::Write,
    };
}

pub use read::Read;
pub use write::Write;

/// Defines ezio's `Read` trait and iterators for reading.
#[macro_use]
mod read;

/// Defines ezio's `Write` trait.
mod write {
    /// A trait for objects which can write out data.
    pub trait Write: std::io::Write {
        /// Write a string to self.
        ///
        /// How the string is written will depend on the implementation.
        ///
        /// Panics if the string cannot be written.
        fn write(&mut self, s: &str);

        /// Write any data which implements `ToString` to self.
        ///
        /// Panics if the string cannot be written.
        fn write_any(&mut self, o: impl ToString)
        where
            Self: Sized,
        {
            <Self as Write>::write(self, &o.to_string())
        }
    }
}

/// File IO.
pub mod file;
/// IO using stdin, stdout, and stderr. Used for terminal IO, etc.
pub mod stdio;
/// Implementation of ezio traits for strings. Useful for mocking and other
/// test usage of ezio in tests.
pub mod string;
