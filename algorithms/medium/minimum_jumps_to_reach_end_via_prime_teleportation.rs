use std::collections::{HashMap, HashSet};

const N: usize = 1000001;

lazy_static::lazy_static! {
    static ref PRIMES: Vec<bool> = {
        let mut v = vec![true; N];
        v[0] = false;
        v[1] = false;
        for i in 2..N {
            if !v[i] {
                continue;
            }
            for j in ((i + i)..N).step_by(i) {
                v[j] = false;
            }
        }

        v
    };
}

impl Solution {
    pub fn min_jumps(nums: Vec<i32>) -> i32 {
        let mut mp = HashMap::new();
        for i in 0..nums.len() {
            mp.entry(nums[i]).or_insert(vec![]).push(i);
        }

        let ma = nums.iter().max().unwrap();

        let mut ans = 0;
        let mut q = vec![0];
        let mut seen = vec![false; nums.len()];
        let mut used = HashSet::new();
        loop {
            let mut t = Vec::new();
            for i in q {
                if i == nums.len() - 1 {
                    return ans;
                }

                if seen[i] {
                    continue;
                }

                seen[i] = true;

                for j in [i - 1, i + 1] {
                    if j < nums.len() && !seen[j] {
                        t.push(j);
                    }
                }

                if PRIMES[nums[i] as usize] && !used.contains(&nums[i]) {
                    for n in (nums[i]..=*ma).step_by(nums[i] as usize) {
                        if let Some(v) = mp.get(&n) {
                            for j in 0..v.len() {
                                if !seen[v[j]] {
                                    t.push(v[j]);
                                }
                            }
                        }
                    }
                }

                used.insert(nums[i]);
            }
            q = t;
            ans += 1;
        }
        -1
    }
}
