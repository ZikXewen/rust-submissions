use std::{
    collections::{HashMap, VecDeque},
    io::stdin,
};

#[derive(Default)]
struct Scanner {
    buf: VecDeque<String>,
}
impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(str) = self.buf.pop_front() {
                return str.parse().ok().unwrap();
            }
            let mut str = String::new();
            stdin().read_line(&mut str).unwrap();
            self.buf = str.split_whitespace().map(String::from).collect();
        }
    }
}
fn main() {
    let mut scan = Scanner::default();
    let n: usize = scan.next();
    let m: usize = scan.next();
    let map: HashMap<usize, usize> = HashMap::from_iter((0..m).map(|_| {
        let (a, b) = (scan.next(), scan.next());
        (b, a)
    }));
    let mut big = VecDeque::<usize>::new();
    let mut small = vec![VecDeque::<usize>::new(); n + 1];
    loop {
        let mode: char = scan.next();
        if mode == 'X' {
            println!("0");
            break;
        }
        if mode == 'D' {
            if big.is_empty() {
                println!("empty");
            }
            let &room = big.front().unwrap();
            println!("{}", small[room].pop_front().unwrap());
            if small[room].is_empty() {
                big.pop_front();
            }
        }
        if mode == 'E' {
            let bk: usize = scan.next();
            let room = map[&bk];
            if small[room].is_empty() {
                big.push_back(room);
            }
            small[room].push_back(bk);
        }
    }
}
/*
2 6
1 41
1 42
1 43
2 201
2 202
2 203
E 41
E 201
D
E 202
E 42
E 43
D
E 203
D
D
D
X
*/
