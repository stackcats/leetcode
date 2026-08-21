impl Solution {
    pub fn optimal_division(nums: Vec<i32>) -> String {
        match nums.len() {
            1 => nums[0].to_string(),
            2 => format!("{}/{}", nums[0], nums[1]),
            _ => {
                let t = nums[0];
                let rest = nums
                    .into_iter()
                    .skip(1)
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join("/");
                format!("{t}/({rest})")
            }
        }
    }
}
