use std::collections::HashMap;

struct UnionFind {
    root: Vec<usize>,
    rank: Vec<i32>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            root: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    fn find(&mut self, t: usize) -> usize {
        let r = self.root[t];
        if r == t {
            return t;
        }

        self.root[t] = self.find(r);
        self.root[t]
    }

    fn union(&mut self, a: usize, b: usize) {
        let ra = self.find(a);
        let rb = self.find(b);
        if ra == rb {
            return;
        }

        if self.rank[ra] < self.rank[rb] {
            self.root[ra] = rb;
        } else if self.rank[ra] > self.rank[rb] {
            self.root[rb] = ra;
        } else {
            self.root[ra] = rb;
            self.rank[rb] += 1;
        }
    }
}

impl Solution {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut uf = UnionFind::new(accounts.len());
        let mut mp = HashMap::new();

        for i in 0..accounts.len() {
            for j in 1..accounts[i].len() {
                if let Some(u) = mp.get(&accounts[i][j]) {
                    uf.union(i, *u);
                } else {
                    mp.insert(accounts[i][j].clone(), i);
                }
            }
        }

        let mut users = vec![vec![]; accounts.len()];

        for (k, v) in mp {
            let r = uf.find(v);
            users[r].push(k);
        }

        let mut ans = Vec::new();
        for (i, mut emails) in users.into_iter().enumerate() {
            if emails.is_empty() {
                continue;
            }
            let mut v = vec![accounts[uf.find(i)][0].clone()];
            emails.sort();
            v.extend(emails);
            ans.push(v);
        }

        ans
    }
}
