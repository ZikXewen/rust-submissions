use std::{
    io::{stdin, BufRead},
    process::exit,
};
#[derive(Default, Clone, Copy)]
struct Bee {
    worker: u32,
    soldier: u32,
}
impl Bee {
    fn new() -> Self {
        Self {
            worker: 1,
            soldier: 0,
        }
    }
    fn next(&mut self) -> Bee {
        Bee {
            worker: self.soldier + self.worker + 1,
            soldier: self.worker,
        }
    }
    fn count(&self) -> u32 {
        self.worker + self.soldier + 1
    }
}
fn main() {
    let stdin = stdin();
    let mut lines = stdin.lock().lines();
    let mut bees = [Bee::new(); 25];
    for i in 0..24 {
        bees[i + 1] = bees[i].next();
    }
    lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .for_each(|x| {
            let x: usize = x.parse().unwrap_or_else(|_| exit(0));
            let bees = bees[x];
            println!("{} {}", bees.worker, bees.count());
        });
}
