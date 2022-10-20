impl Solution {
    pub fn check_distances(s: String, distance: Vec<i32>) -> bool {
        let mut mp = vec![0; 26];
        for (i, b) in s.as_bytes().into_iter().enumerate() {
            let ndx = (b - b'a') as usize;
            if mp[ndx] > 0 && distance[ndx] != (i as i32) - mp[ndx] {
                return false;
            }
            mp[ndx] = i as i32 + 1;
        }
        true
    }
}
