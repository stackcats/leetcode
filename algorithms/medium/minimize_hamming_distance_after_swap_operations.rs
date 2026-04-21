use std::collections::HashMap;

struct UnionFind {
    root: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            root: (0..n).into_iter().collect::<Vec<_>>(),
            rank: vec![0; n],
        }
    }

    fn find(&mut self, n: usize) -> usize {
        if self.root[n] != n {
            self.root[n] = self.find(self.root[n]);
        }
        self.root[n]
    }

    fn union(&mut self, a: usize, b: usize) {
        let fa = self.find(a);
        let fb = self.find(b);
        if fa == fb {
            return;
        }
        if fa < fb {
            self.root[fa] = fb;
        } else if fa > fb {
            self.root[fb] = fa;
        } else {
            self.root[fa] = fb;
            self.rank[fa] += 1;
        }
    }
}

impl Solution {
    pub fn minimum_hamming_distance(
        source: Vec<i32>,
        target: Vec<i32>,
        allowed_swaps: Vec<Vec<i32>>,
    ) -> i32 {
        let mut uf = UnionFind::new(source.len());
        for v in allowed_swaps {
            uf.union(v[0] as usize, v[1] as usize);
        }

        let mut mp = HashMap::new();
        for i in 0..source.len() {
            let f = uf.find(i);
            let tmp = mp.entry(f).or_insert(HashMap::new());
            *tmp.entry(source[i]).or_insert(0) += 1;
        }

        let mut ans = 0;
        for i in 0..source.len() {
            let f = uf.find(i);
            let tmp = mp.entry(f).or_insert(HashMap::new());
            if let Some(v) = tmp.get_mut(&target[i])
                && *v > 0
            {
                *v -= 1;
            } else {
                ans += 1;
            }
        }
        ans
    }
}
