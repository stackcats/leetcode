impl Solution {
    pub fn min_deletions(s: String) -> i32 {
        let mut map = vec![0; 26];
        
        for b in s.as_bytes() {
            map[(b - b'a') as usize] += 1;
        }

        map.sort_unstable();
        
        let mut ans = 0;
        
        for i in (0..25).rev() {
            if map[i] >= map[i+1] {
                let t = map[i];
                map[i] = 0.max(map[i+1] - 1);
                ans += t - map[i];
            }
        }
        
        ans
    }
}
