use std::{
    collections::HashMap,
    io::{stdin, BufRead, Lines, StdinLock},
    mem::swap,
};

struct Tree {
    tree: Vec<Vec<(usize, usize)>>,
    size: usize,
}
impl Tree {
    fn new(mut size: usize) -> Self {
        size = (1usize << (size - 1).leading_zeros()).reverse_bits() << 1;
        Tree {
            tree: vec![vec![]; size * 2],
            size,
        }
    }
    fn add_to_tree(&mut self, mut l: usize, mut r: usize, va: (usize, usize)) {
        l += self.size;
        r += self.size;
        while l < r {
            if l & 1 == 1 {
                self.tree[l].push(va);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                self.tree[r].push(va);
            }
            l >>= 1;
            r >>= 1;
        }
    }
}
#[derive(Clone, Copy)]
struct DSUNode {
    parent: Option<usize>,
    count: usize,
    parity: bool,
}
struct DSU {
    nodes: Vec<DSUNode>,
    rb_parent: Vec<(usize, Option<usize>)>,
    rb_count: Vec<(usize, usize)>,
    rb_parity: Vec<(usize, bool)>,
}
impl DSU {
    fn new(size: usize) -> DSU {
        DSU {
            nodes: vec![
                DSUNode {
                    parent: None,
                    count: 1,
                    parity: false,
                };
                size
            ],
            rb_count: vec![],
            rb_parent: vec![],
            rb_parity: vec![],
        }
    }
    fn find(&self, mut pos: usize) -> (usize, bool) {
        let mut current_parity = true;
        loop {
            let DSUNode { parity, parent, .. } = self.nodes[pos];
            current_parity ^= parity;
            match parent {
                None => return (pos, current_parity),
                Some(parent) => pos = parent,
            }
        }
    }
    fn union(&mut self, (u, v): (usize, usize)) -> bool {
        let (mut u, mut u_parity) = self.find(u);
        let (mut v, mut v_parity) = self.find(v);
        if u == v {
            return u_parity != v_parity;
        }
        if self.nodes[u].count < self.nodes[v].count {
            swap(&mut u, &mut v);
            swap(&mut u_parity, &mut v_parity);
        }
        self.rb_count.push((u, self.nodes[u].count));
        self.rb_parent.push((v, self.nodes[v].parent));
        self.rb_parity.push((v, self.nodes[v].parity));
        self.nodes[u].count += self.nodes[v].count;
        self.nodes[v].parent = Some(u);
        self.nodes[v].parity = u_parity ^ v_parity ^ true;
        true
    }
    fn get_snapshot(&self) -> (usize, usize, usize) {
        (
            self.rb_count.len(),
            self.rb_parent.len(),
            self.rb_parity.len(),
        )
    }
    fn to_snapshot(&mut self, (count_size, parent_size, parity_size): (usize, usize, usize)) {
        while self.rb_count.len() > count_size {
            let (po, va) = self.rb_count.pop().unwrap();
            self.nodes[po].count = va;
        }
        while self.rb_parent.len() > parent_size {
            let (po, va) = self.rb_parent.pop().unwrap();
            self.nodes[po].parent = va;
        }
        while self.rb_parity.len() > parity_size {
            let (po, va) = self.rb_parity.pop().unwrap();
            self.nodes[po].parity = va;
        }
    }
}
fn solve(id: usize, tree: &Tree, dsu: &mut DSU, ans: &mut Vec<bool>) {
    for &i in &tree.tree[id] {
        if !dsu.union(i) {
            return;
        }
    }
    if id >= tree.size {
        ans[id - tree.size] = true;
        return;
    }
    let mut snap = dsu.get_snapshot();
    solve(id * 2, tree, dsu, ans);
    dsu.to_snapshot(snap);
    snap = dsu.get_snapshot();
    solve(id * 2 + 1, tree, dsu, ans);
    dsu.to_snapshot(snap);
}
fn line_to_ints(lines: &mut Lines<StdinLock>) -> Vec<usize> {
    lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}
fn main() {
    let stdin = stdin();
    let mut lines = stdin.lock().lines();
    let inp = line_to_ints(&mut lines);
    let n = inp[0];
    let m = inp[1];
    let mut mp: HashMap<(usize, usize), usize> = HashMap::new();
    let mut tree = Tree::new(m);
    let mut dsu = DSU::new(n + 1);
    for i in 0..m {
        let inp = line_to_ints(&mut lines);
        let edge = if inp[1] < inp[2] {
            (inp[1], inp[2])
        } else {
            (inp[2], inp[1])
        };
        if inp[0] == 1 {
            mp.insert(edge, i);
        } else {
            let left = mp.get(&edge).unwrap();
            tree.add_to_tree(*left, i, edge);
            mp.remove(&edge);
        }
    }
    for (edge, left) in mp.into_iter() {
        tree.add_to_tree(left, m, edge);
    }
    let mut ans = vec![false; tree.size];
    solve(1, &tree, &mut dsu, &mut ans);
    for i in ans.into_iter().take(m) {
        println!("{}", if i { "yes" } else { "no" })
    }
}
/*
5 8
1 1 5
1 4 3
1 3 2
1 1 4
1 5 2
0 4 3
0 2 5
1 5 4
*/
