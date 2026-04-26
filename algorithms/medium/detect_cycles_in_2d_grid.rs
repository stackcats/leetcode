use std::collections::{HashSet, VecDeque};

struct UnionFind {
    root: Vec<usize>,
    rank: Vec<i32>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            root: (0..n).into_iter().collect(),
            rank: vec![0; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.root[x] != x {
            self.root[x] = self.find(self.root[x]);
        }
        self.root[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let x = self.find(x);
        let y = self.find(y);
        if x == y {
            return;
        }

        if self.rank[x] < self.rank[y] {
            self.root[x] = y;
        } else if self.rank[x] > self.rank[y] {
            self.root[y] = x;
        } else {
            self.root[x] = y;
            self.rank[x] += 1;
        }
    }

    fn is_same_root(&mut self, x: usize, y: usize) -> bool {
        let x = self.find(x);
        let y = self.find(y);
        x == y
    }
}

impl Solution {
    pub fn contains_cycle(grid: Vec<Vec<char>>) -> bool {
        let mut n = grid.len();
        let mut m = grid[0].len();
        let mut uf = UnionFind::new(n * m);
        for i in 0..n {
            for j in 0..m {
                let x = i * m + j;
                if i > 0 && grid[i][j] == grid[i - 1][j] {
                    let y = (i - 1) * m + j;
                    if uf.is_same_root(x, y) {
                        return true;
                    }
                    uf.union(x, y);
                }

                if j > 0 && grid[i][j] == grid[i][j - 1] {
                    let y = i * m + j - 1;
                    if uf.is_same_root(x, y) {
                        return true;
                    }
                    uf.union(x, y);
                }
            }
        }

        false
    }
}
