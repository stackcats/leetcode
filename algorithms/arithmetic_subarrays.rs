impl Solution {
    pub fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
        let mut ans = Vec::new();
        for i in 0..l.len() {
            let start = l[i] as usize;
            let end = r[i] as usize;
            if end - start <= 1 {
                ans.push(true);
                continue;
            }
            let mut v = nums[start..=end].to_vec();
            v.sort();
            let diff = v[1] - v[0];
            let mut is_arithmetic = true;
            for j in 2..v.len() {
                if v[j] - v[j - 1] != diff {
                    is_arithmetic = false;
                    break;
                }
            }
            ans.push(is_arithmetic);
        }
        ans
    }
}
