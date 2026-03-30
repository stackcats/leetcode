fn split(s: String) -> (Vec<i32>, Vec<i32>) {
    let mut t1 = vec![0; 26];
    let mut t2 = vec![0; 26];
    for (i, b) in s.as_bytes().into_iter().enumerate() {
        if i % 2 == 0 {
            t1[(*b - b'a') as usize] += 1;
        } else {
            t2[(*b - b'a') as usize] += 1;
        }
    }

    (t1, t2)
}

impl Solution {
    pub fn check_strings(s1: String, s2: String) -> bool {
        if s1.len() != s2.len() {
            return false;
        }

        split(s1) == split(s2)
    }
}
