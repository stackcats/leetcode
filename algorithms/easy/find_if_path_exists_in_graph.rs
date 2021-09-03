struct UnionFind {
    roots: Vec<i32>,
    ranks: Vec<i32>,
}

impl UnionFind {
    fn new(n: i32) -> Self {
        Self {
            roots: (0..n).collect(),
            ranks: vec![1; n as usize],
        }
    }

    fn find(&mut self, x: i32) -> i32 {
        let ndx = x as usize;
        if self.roots[ndx] != x {
            self.roots[ndx] = self.find(self.roots[ndx]);
        }
        self.roots[ndx]
    }

    fn union(&mut self, x: i32, y: i32) {
        let rootX = self.find(x);
        let rootY = self.find(y);
        let nx = rootX as usize;
        let ny = rootY as usize;
        if self.ranks[nx] > self.ranks[ny] {
            self.roots[rootY as usize] = rootX;
        } else if self.ranks[nx] < self.ranks[ny] {
            self.roots[nx] = rootY;
        } else {
            self.roots[ny] = rootX;
            self.ranks[ny] += 1;
        }
    }
}

impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, start: i32, end: i32) -> bool {
        let mut uf = UnionFind::new(n);
        for edge in &edges {
            uf.union(edge[0], edge[1]);
        }

        uf.find(start) == uf.find(end)
    }
}
