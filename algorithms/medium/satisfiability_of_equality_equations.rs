struct DSet {
    roots: Vec<u8>,
    ranks: Vec<i32>,
}

impl DSet {
    fn new() -> Self {
        let mut roots = vec![0; 26];
        for c in b'a'..=b'z' {
            roots[(c - b'a') as usize] = c;
        }
        Self {
            roots,
            ranks: vec![1; 26],
        }
    }

    fn find(&mut self, x: u8) -> u8 {
        let ndx = (x - b'a') as usize;
        if self.roots[ndx] == x {
            return x;
        }

        let r = self.find(self.roots[ndx]);
        self.roots[ndx] = r;
        r
    }

    fn union(&mut self, x: u8, y: u8) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x == root_y {
            return;
        }

        let x_ndx = (root_x - b'a') as usize;
        let y_ndx = (root_y - b'a') as usize;
        if self.ranks[x_ndx] > self.ranks[y_ndx] {
            self.roots[y_ndx] = root_x;
        } else if self.ranks[x_ndx] < self.ranks[y_ndx] {
            self.roots[x_ndx] = root_y
        } else {
            self.roots[x_ndx] = root_y;
            self.ranks[x_ndx] += 1;
        }
    }
}

impl Solution {
    pub fn equations_possible(equations: Vec<String>) -> bool {
        let (eqs, nes): (Vec<_>, Vec<_>) = equations.into_iter().partition(|s| s.contains("=="));
        let mut set = DSet::new();
        eqs.into_iter().for_each(|eq| {
            let bs = eq.as_bytes();
            set.union(bs[0], bs[3]);
        });
        nes.into_iter().all(|eq| {
            let bs = eq.as_bytes();
            set.find(bs[0]) != set.find(bs[3])
        })
    }
}
