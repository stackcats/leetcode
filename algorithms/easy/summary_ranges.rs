fn range_to_string(left: i32, right: i32) -> String {
    if left == right {
        return format!("{}", left);
    }
    format!("{}->{}", left, right)
}

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        if nums.len() == 0 {
            return vec![];
        }
        let mut ans = Vec::new();
        let mut left = nums[0];
        let mut right = nums[0];
        for i in 1..nums.len() {
            if nums[i] != right + 1 {
                ans.push(range_to_string(left, right));
                left = nums[i];
            }
            right = nums[i];
        }
        ans.push(range_to_string(left, right));
        ans
    }
}
