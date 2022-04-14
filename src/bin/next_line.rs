use std::io::{stdin, BufRead, Lines};

trait NextLine {
    fn next_line(&mut self) -> String;
}
impl<T: BufRead> NextLine for Lines<T> {
    fn next_line(&mut self) -> String {
        self.next().unwrap().unwrap()
    }
}
fn main() {
    let stdin = stdin();
    let mut lines = stdin.lock().lines();
    dbg!(lines.next_line());
}
