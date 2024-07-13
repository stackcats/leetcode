impl Solution {
    pub fn last_visited_integers(nums: Vec<i32>) -> Vec<i32> {
        let mut seen = Vec::new();
        let mut ans = Vec::new();
        let mut k = 1usize;
        for n in nums {
            if n > 0 {
                seen.push(n);
                k = 1;
            } else if k > seen.len() {
                ans.push(-1);
            } else {
                ans.push(seen[seen.len() - k]);
                k += 1;
            }
        }
        ans
    }
}
