// https://leetcode.com/problems/largest-number/

impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut snums = Vec::new();

        for i in 0..nums.len() {
            snums.push(format!("{}", nums[i]));
        }

        snums.sort_by(|a, b| format!("{}{}", b, a).cmp(&format!("{}{}", a, b)));

        let mut i = 0;
        while i < snums.len() - 1 {
            // 过滤掉多余的"0"
            if snums[i] != "0" {
                break;
            }
            i += 1;
        }

        let mut ans = String::new();
        while i < snums.len() {
            ans.push_str(snums[i].as_str());
            i += 1;
        }

        ans
    }
}
