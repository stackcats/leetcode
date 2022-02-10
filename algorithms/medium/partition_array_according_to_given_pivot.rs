impl Solution {
    pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let mut ans = vec![pivot; nums.len()];
        let mut j = 0;
        for &n in &nums {
            if n < pivot {
                ans[j] = n;
                j += 1;
            }
        }
        j = nums.len() - 1;
        for &n in nums.iter().rev() {
            if n > pivot {
                ans[j] = n;
                j -= 1;
            }
        }
        ans
    }
}
