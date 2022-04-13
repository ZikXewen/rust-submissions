use std::{
    collections::HashMap,
    io::{stdin, BufRead},
};
fn main() {
    let stdin = stdin();
    let mut lines = stdin.lock().lines();
    let transform: [[usize; 6]; 6] = [
        [4, 1, 3, 6, 5, 2],
        [2, 6, 3, 1, 5, 4],
        [5, 2, 1, 4, 6, 3],
        [3, 2, 6, 4, 1, 5],
        [1, 5, 2, 3, 4, 6],
        [1, 3, 4, 5, 2, 6],
    ];
    let faces: HashMap<char, usize> =
        HashMap::from([('F', 0), ('B', 1), ('L', 2), ('R', 3), ('C', 4), ('D', 5)]);
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for _ in 0..n {
        print!(
            "{} ",
            lines
                .next()
                .unwrap()
                .unwrap()
                .chars()
                .fold([1, 2, 3, 5, 4, 6], |ar, c| [0, 1, 2, 3, 4, 5]
                    .map(|x| ar[transform[faces[&c]][x] - 1]))[1]
        );
    }
}
