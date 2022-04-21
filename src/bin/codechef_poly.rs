// Grader uses an overly outdated version of compiler.
// Too lazy to rollback to old ver.

use std::{
    io::{stdin, BufRead, Lines},
    mem::swap,
};

trait Reader {
    fn next_line(&mut self) -> String;
    fn next_int(&mut self) -> usize;
    fn next_ints(&mut self) -> Vec<usize>;
}
impl<B: BufRead> Reader for Lines<B> {
    fn next_line(&mut self) -> String {
        self.next().unwrap().unwrap()
    }
    fn next_int(&mut self) -> usize {
        self.next_line().parse().unwrap()
    }
    fn next_ints(&mut self) -> Vec<usize> {
        self.next_line()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect()
    }
}

struct Line(Vec<usize>);
impl Line {
    fn calc(&self, x: usize) -> usize {
        let ar = &self.0;
        ar[0] + ar[1] * x + ar[2] * x * x + ar[3] * x * x * x
    }
}
struct LiChaoNode {
    line: Line,
    l_pos: usize,
    r_pos: usize,
    l_node: Option<usize>,
    r_node: Option<usize>,
}
impl LiChaoNode {
    fn new(l_pos: usize, r_pos: usize) -> Self {
        Self {
            line: Line(vec![usize::MAX, 0, 0, 0]),
            l_pos,
            r_pos,
            l_node: None,
            r_node: None,
        }
    }
}
struct LiChao {
    tree: Vec<LiChaoNode>,
}
impl LiChao {
    fn new() -> Self {
        Self {
            tree: vec![LiChaoNode::new(1, 100000)],
        }
    }
    fn add(&mut self, mut line: Line) {
        let mut node_id = 0;
        loop {
            let node = &mut self.tree[node_id];
            let l = node.l_pos;
            let r = node.r_pos;
            let m = (l + r) >> 1;
            let l_cmp = line.calc(l) < node.line.calc(l);
            let m_cmp = line.calc(m) < node.line.calc(m);
            if m_cmp {
                swap(&mut node.line, &mut line);
            }
            if l == r {
                return;
            }
            if l_cmp != m_cmp {
                if node.l_node.is_none() {
                    self.tree.push(LiChaoNode::new(l, m));
                    self.tree[node_id].l_node = Some(self.tree.len() - 1);
                }
                node_id = self.tree[node_id].l_node.unwrap();
            } else {
                if node.r_node.is_none() {
                    self.tree.push(LiChaoNode::new(m + 1, r));
                    self.tree[node_id].r_node = Some(self.tree.len() - 1);
                }
                node_id = self.tree[node_id].r_node.unwrap();
            }
        }
    }
    fn get(&self, pos: usize) -> usize {
        let mut ret = usize::MAX;
        let mut node_id = 0;
        loop {
            let node = &self.tree[node_id];
            let l = node.l_pos;
            let r = node.r_pos;
            ret = ret.min(node.line.calc(pos));
            let m = (l + r) >> 1;
            if m >= pos {
                if node.l_node.is_none() {
                    return ret;
                }
                node_id = node.l_node.unwrap();
            } else {
                if node.r_node.is_none() {
                    return ret;
                }
                node_id = node.r_node.unwrap();
            }
        }
    }
}
fn main() {
    let stdin = stdin();
    let mut lines = stdin.lock().lines();
    let cases: usize = lines.next_int();
    for _ in 0..cases {
        let n: usize = lines.next_int();
        let mut tree = LiChao::new();
        for _ in 0..n {
            tree.add(Line(lines.next_ints()));
        }
        let q: usize = lines.next_int();
        for _ in 0..q {
            println!("{}", tree.get(lines.next_int()));
        }
    }
}
