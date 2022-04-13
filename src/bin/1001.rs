use std::io::{stdin, BufRead};
fn main() {
    let stdin = stdin();
    let mut lines = stdin.lock().lines();
    let mut line: Vec<usize> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let m = line.pop().unwrap();
    let n = line.pop().unwrap();
    let arr: Vec<Vec<char>> = (0..n)
        .map(|_| lines.next().unwrap().unwrap().chars().collect())
        .collect();
    let blk: Vec<usize> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let pos: Vec<usize> = (0..m)
        .map(|i| (0..n).position(|j| arr[j][i] == 'O').unwrap_or(n))
        .collect();
    for i in 0..n {
        for j in 0..m {
            if i < pos[j] && i + blk[j] >= pos[j] {
                print!("#");
            } else {
                print!("{}", arr[i][j]);
            }
        }
        println!();
    }
}
/*
8 5
.....
.....
.OO..
.....
.O...
...O.
.....
.....
1 1 3 2 0
*/
