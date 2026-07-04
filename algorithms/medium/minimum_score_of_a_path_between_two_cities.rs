struct UnionFind {
    root: Vec<usize>,
    rank: Vec<usize>,
    score: Vec<i32>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            root: (0..n).collect::<Vec<_>>(),
            rank: vec![0; n],
            score: vec![i32::MAX; n],
        }
    }

    fn find(&mut self, v: usize) -> usize {
        if self.root[v] != v {
            self.root[v] = self.find(self.root[v]);
        }
        let r = self.root[v];
        let s = self.score[v].min(self.score[r]);
        self.score[v] = s;
        self.score[r] = s;
        r
    }

    fn union(&mut self, u: usize, v: usize, s: i32) {
        let ru = self.find(u);
        let rv = self.find(v);
        let s = s.min(self.score[ru]).min(self.score[rv]);
        self.score[u] = s;
        self.score[v] = s;
        self.score[ru] = s;
        self.score[rv] = s;

        if ru == rv {
            return;
        }

        if self.rank[ru] < self.rank[rv] {
            self.root[ru] = rv;
        } else if self.rank[ru] > self.rank[rv] {
            self.root[rv] = ru;
        } else {
            self.root[rv] = ru;
            self.rank[rv] += 1;
        }
    }
}

impl Solution {
    pub fn min_score(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let mut uf = UnionFind::new(n as usize);
        for road in roads {
            let (a, b, s) = (road[0] - 1, road[1] - 1, road[2]);
            uf.union(a as usize, b as usize, s);
        }
        uf.score[0]
    }
}
