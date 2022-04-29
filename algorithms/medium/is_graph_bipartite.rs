struct UnionFind {
    roots: Vec<usize>,
    ranks: Vec<i32>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            roots: (0..n).collect(),
            ranks: vec![0; n],
        }
    }

    fn find(&self, p: usize) -> usize {
        if p == self.roots[p] {
            return p;
        }

        self.find(self.roots[p])
    }

    fn union(&mut self, p: usize, q: usize) {
        let rp = self.find(p);
        let rq = self.find(q);
        if self.ranks[rp] > self.ranks[rq] {
            self.roots[rq] = rp;
        } else if self.ranks[rp] < self.ranks[rq] {
            self.roots[rp] = rq;
        } else {
            self.roots[rq] = rp;
            self.ranks[rq] += 1;
        }
    }

    fn is_connected(&self, p: usize, q: usize) -> bool {
        self.find(p) == self.find(q)
    }
}

impl Solution {
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        let mut uf = UnionFind::new(graph.len());
        for (i, neighbours) in graph.into_iter().enumerate() {
            for &j in &neighbours {
                if uf.is_connected(i, j as usize) {
                    return false;
                }
                uf.union(neighbours[0] as usize, j as usize);
            }
        }

        true
    }
}
