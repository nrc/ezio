use ezio::prelude::*;

fn main() {
    let mut stdin = stdin();
    println!("`{}`", stdin.read_line());

    for l in stdin {
        println!("`{}`", l);
    }
}
