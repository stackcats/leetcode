impl Solution {
    pub fn count_complete_day_pairs(hours: Vec<i32>) -> i32 {
        let mut ct = vec![0; 24];
        let mut ans = 0;
        for h in hours {
            let r = h % 24;
            if r == 0 {
                ans += ct[0];
            } else {
                ans += ct[24 - r as usize];
            }
            ct[r as usize] += 1;
        }
        ans
    }
}
