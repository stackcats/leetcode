use std::collections::{HashMap, HashSet, VecDeque};

impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let mut mp = HashMap::new();
        for (i, &n) in arr.iter().enumerate() {
            mp.entry(n).or_insert(vec![]).push(i);
        }

        let mut q = VecDeque::new();
        q.push_back(0);
        let mut ans = 0;
        let mut seen = vec![false; arr.len()];
        let mut extended = HashSet::new();

        loop {
            let mut p = VecDeque::new();
            while let Some(i) = q.pop_front() {
                if i == arr.len() - 1 {
                    return ans;
                }
                seen[i] = true;
                for j in [i - 1, i + 1] {
                    if j < arr.len() && !seen[j] {
                        p.push_back(j);
                    }
                }
                if extended.contains(&arr[i]) {
                    continue;
                }
                extended.insert(arr[i]);
                for &j in mp.get(&arr[i]).unwrap() {
                    if j != i && !seen[j] {
                        p.push_back(j);
                    }
                }
            }
            ans += 1;
            q = p;
        }

        -1
    }
}
