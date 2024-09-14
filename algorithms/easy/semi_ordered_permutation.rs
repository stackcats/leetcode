impl Solution {
    pub fn semi_ordered_permutation(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let mut first = 0;
        let mut last = 0;
        for i in 0..nums.len() {
            if nums[i] == 1 {
                first = i;
            } else if nums[i] == n {
                last = i;
            }
        }
        let mut ans = (first + nums.len() - last - 1) as i32;
        if first > last {
            ans -= 1;
        }
        ans
    }
}
