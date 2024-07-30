impl Solution {
    pub fn find_permutation_difference(s: String, t: String) -> i32 {
        let mut srr = vec![0; 26];
        let mut trr = vec![0; 26];
        let s = s.as_bytes();
        let t = t.as_bytes();
        for i in 0..s.len() {
            srr[(s[i] - b'a') as usize] = i as i32;
            trr[(t[i] - b'a') as usize] = i as i32;
        }
        let mut diff = 0;
        for i in 0..26 {
            diff += (srr[i] - trr[i]).abs();
        }
        diff
    }
}
