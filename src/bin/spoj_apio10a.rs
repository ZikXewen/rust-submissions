use std::{
    io::{stdin, BufRead, Lines},
    mem::swap,
};

trait Reader {
    fn next_line(&mut self) -> String;
    fn next_int(&mut self) -> isize;
    fn next_ints(&mut self) -> Vec<isize>;
}
impl<B: BufRead> Reader for Lines<B> {
    fn next_line(&mut self) -> String {
        self.next().unwrap().unwrap()
    }
    fn next_int(&mut self) -> isize {
        self.next_line().parse().unwrap()
    }
    fn next_ints(&mut self) -> Vec<isize> {
        self.next_line()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect()
    }
}
struct Line(isize, isize);
struct LiChaoNode {
    line: Line,
    left: Option<Box<LiChaoNode>>,
    right: Option<Box<LiChaoNode>>,
}
struct LiChao {
    tree: Box<LiChaoNode>,
}
impl Line {
    fn calc(&self, x: isize) -> isize {
        self.0 * x + self.1
    }
}
impl LiChaoNode {
    fn new() -> Self {
        Self {
            line: Line(0, 0),
            left: None,
            right: None,
        }
    }
    fn from(line: Line) -> Self {
        Self {
            line,
            left: None,
            right: None,
        }
    }
}
impl LiChao {
    fn query(&self, pos: isize) -> isize {
        let mut node = &self.tree;
        let mut ret = isize::MIN;
        let mut l = 1;
        let mut r = 100_000_000;
        loop {
            ret = ret.max(node.line.calc(pos));
            let m = (l + r) >> 1;
            if pos <= m {
                if node.left.is_none() {
                    break;
                }
                node = node.left.as_ref().unwrap();
                r = m;
            } else {
                if node.right.is_none() {
                    break;
                }
                node = node.right.as_ref().unwrap();
                l = m + 1;
            }
        }
        ret
    }
    fn add(&mut self, mut line: Line) {
        let mut node = &mut self.tree;
        let mut l = 1;
        let mut r = 100_000_000;
        loop {
            let m = (l + r) >> 1;
            let l_cmp = line.calc(l) > node.line.calc(l);
            let m_cmp = line.calc(m) > node.line.calc(m);
            if m_cmp {
                swap(&mut line, &mut node.line);
            }
            if l == r {
                return;
            }
            if l_cmp != m_cmp {
                if node.left.is_none() {
                    node.left = Some(Box::new(LiChaoNode::from(line)));
                    return;
                }
                node = node.left.as_mut().unwrap();
                r = m;
            } else {
                if node.right.is_none() {
                    node.right = Some(Box::new(LiChaoNode::from(line)));
                    return;
                }
                node = node.right.as_mut().unwrap();
                l = m + 1;
            }
        }
    }
}
fn main() {
    let stdin = stdin();
    let mut lines = stdin.lock().lines();
    for _ in 0..lines.next_int() {
        lines.next();
        let poly = lines.next_ints();
        let mut sm = 0;
        let mut tree = LiChao {
            tree: Box::new(LiChaoNode::new()),
        };
        let mut dp = 0;
        for i in lines.next_ints() {
            sm += i;
            dp = tree.query(sm) + poly[0] * sm * sm + poly[1] * sm + poly[2];
            tree.add(Line(
                -2 * poly[0] * sm,
                dp + poly[0] * sm * sm - poly[1] * sm,
            ));
        }
        println!("{}", dp);
    }
}
