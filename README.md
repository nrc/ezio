# ezio - a crate for easy IO

ezio offers an easy to use IO API for reading and writing to files and stdio.
ezio includes utilities for generating random numbers and other IO-like functionality.
Performance and idiomatic error handling are explicit non-goals, so ezio is
probably not suitable for production use. It is better suited for education,
experimentation, and prototyping.

ezio wraps the standard library's IO APIs and other well-established crates, and is designed
to interoperate with them, so ezio should be compatible with most upstream libraries.

## Examples

```rust
use ezio::prelude::*;

fn main() {
    // Read a line from stdin
    let _ = stdio::read_line();

    // Iterate lines in a file
    for line in file::reader("path/to/file.txt") {
        // ...
    }

    // Read a whole file
    let _ = file::read("path/to/file.txt");

    // Write to a file
    file::write("path/to/file.txt", "Some text");

    // Write multiple things to a file
    let mut w = file::writer("path/to/file.txt");
    w.write("Some text\n");
    w.write("Some more text");

    // Generates a random u32
    let _ = random::u32();
}
```

## Design principals

(ezio is work in progress, so these may still be aspirational)

* Easy to use!
* Easy to import - provide a prelude and most users will not need anything else
* Simple module hierarchy
* String-based, not byte-based by default
* Panic-happy: panic by default, `try_` versions of functions where you really need an error
* Allocation-happy: returns Strings, etc rather than taking buffers
* Compatible and interoperable with std IO so programs can gradually migrate from ezio to std::io
* Just because we're doing unsophisticated IO, doesn't mean the rest of the program is unsophisticated. Therefore:
  - should be idiomatic Rust
  - should support generics and trait objects, etc
