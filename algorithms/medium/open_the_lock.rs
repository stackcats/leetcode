use std::collections::{HashSet, VecDeque};

fn to_int(s: String) -> i32 {
    let s = s.as_bytes();
    let mut n = 0;
    for i in 0..s.len() {
        n = n * 10 + (s[i] - b'0') as i32;
    }
    n
}

fn rotate(n: i32) -> Vec<i32> {
    let mut ans = Vec::new();
    let mut v = [n / 1000, n / 100 % 10, n / 10 % 10, n % 10];
    for i in 0..4 {
        let t = v[i];
        for d in [-1, 1] {
            v[i] = (v[i] + d + 10) % 10;
            ans.push(v.iter().fold(0, |acc, &d| acc * 10 + d));
            v[i] = t;
        }
    }
    ans
}

impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        let mut st: HashSet<i32> = deadends.into_iter().map(to_int).collect();
        if st.contains(&0) {
            return -1;
        }
        st.insert(0);

        let mut q = VecDeque::new();
        q.push_back(0);
        let t = to_int(target);
        let mut ans = 0;

        while !q.is_empty() {
            for _ in 0..q.len() {
                let s = q.pop_front().unwrap();
                if s == t {
                    return ans;
                }

                for v in rotate(s) {
                    if !st.contains(&v) {
                        st.insert(v);
                        q.push_back(v);
                    }
                }
            }
            ans += 1;
        }

        -1
    }
}
