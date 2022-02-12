impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let mut map = vec![0; 26];
        for b in s1.as_bytes() {
            map[(b - b'a') as usize] += 1;
        }
        let len = s1.len();
        let s2 = s2.as_bytes();
        let mut tmp = 0;
        for (i, b) in s2.iter().enumerate() {
            map[(b - b'a') as usize] -= 1;
            if i >= len {
                map[(s2[i - len] - b'a') as usize] += 1;
            }
            if map.iter().all(|v| *v == 0) {
                return true;
            }
        }
        false
    }
}
