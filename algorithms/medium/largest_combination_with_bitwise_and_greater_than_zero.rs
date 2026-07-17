impl Solution {
    pub fn largest_combination(candidates: Vec<i32>) -> i32 {
        let mut ans = 0;
        for i in 0..24 {
            let mut ct = 0;
            for &n in &candidates {
                if (1 << i) & n > 0 {
                    ct += 1;
                }
            }
            ans = ans.max(ct);
        }
        ans
    }
}
