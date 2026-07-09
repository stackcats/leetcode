struct UnionFind {
    root: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            root: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    fn find(&mut self, u: usize) -> usize {
        let r = self.root[u];
        if r == u {
            u
        } else {
            self.root[r] = self.find(r);
            self.root[r]
        }
    }

    fn union(&mut self, u: usize, v: usize) {
        let ru = self.find(u);
        let rv = self.find(v);
        if ru == rv {
            return;
        }

        if ru < rv {
            self.root[ru] = rv;
        } else if ru > rv {
            self.root[rv] = ru;
        } else {
            self.root[ru] = rv;
            self.rank[rv] += 1;
        }
    }

    fn is_connected(&mut self, u: usize, v: usize) -> bool {
        self.find(u) == self.find(v)
    }
}

impl Solution {
    pub fn path_existence_queries(
        n: i32,
        nums: Vec<i32>,
        max_diff: i32,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        let mut uf = UnionFind::new(n as usize);
        for i in 1..nums.len() {
            if (nums[i] - nums[i - 1]).abs() <= max_diff {
                uf.union(i, i - 1);
            }
        }

        let mut ans = Vec::new();
        for q in queries {
            let (u, v) = (q[0] as usize, q[1] as usize);
            ans.push(uf.is_connected(u, v));
        }
        ans
    }
}
