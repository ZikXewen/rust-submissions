use std::io::{stdin, stdout, BufWriter, Write};

fn main() {
    let out = &mut BufWriter::new(stdout());

    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut buf = String::new();
    stdin().read_line(&mut buf).ok();
    let mut it = buf.chars();
    while let Some(c) = it.next() {
        write!(out, "{}", c).ok();
        if vowels.contains(&c) {
            it.nth(1);
        }
    }
}
