impl Solution {
    pub fn minimum_distance(nums: Vec<i32>) -> i32 {
        let mut ans = i32::MAX;
        let mut is = vec![-1; 100005];
        let mut js = vec![-1; 100005];
        for (k, n) in nums.into_iter().enumerate() {
            let n = n as usize;
            let k = k as i32;
            if is[n] == -1 {
                is[n] = k;
            } else if js[n] == -1 {
                js[n] = k;
            } else {
                ans = ans.min(2 * (k - is[n]));
                is[n] = js[n];
                js[n] = k;
            }
        }
        if ans == i32::MAX {
            -1
        } else {
            ans
        }
    }
}
