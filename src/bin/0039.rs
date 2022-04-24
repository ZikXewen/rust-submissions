use std::io::{stdin, stdout, BufRead, BufWriter, Write};
fn next_perm(ar: &mut [u8]) -> bool {
    let po = match ar.windows(2).rposition(|w| w[0] < w[1]) {
        Some(i) => i,
        None => return false,
    };
    let se = ar[po + 1..]
        .binary_search_by(|n| ar[po].cmp(n).then(std::cmp::Ordering::Less))
        .unwrap_err();
    ar.swap(po, po + se);
    ar[po + 1..].reverse();
    true
}
fn main() {
    let stdin = stdin();
    let mut stdout = BufWriter::new(stdout());
    let mut lines = stdin.lock().lines();
    let n = lines.next().unwrap().unwrap().parse::<u8>().unwrap();
    lines.next();
    let ar: Vec<u8> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u8>().unwrap())
        .collect();
    for i in 1..=n {
        if !ar.contains(&i) {
            let mut arr: Vec<u8> = (1..=n).filter(|x| x != &i).collect();
            write!(stdout, "{}", i).unwrap();
            for j in &arr {
                write!(stdout, " {}", j).unwrap();
            }
            writeln!(stdout).unwrap();
            while next_perm(&mut arr) {
                write!(stdout, "{}", i).unwrap();
                for j in &arr {
                    write!(stdout, " {}", j).unwrap();
                }
                writeln!(stdout).unwrap();
            }
        }
    }
}
