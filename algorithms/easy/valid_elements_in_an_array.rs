impl Solution {
    pub fn find_valid_elements(nums: Vec<i32>) -> Vec<i32> {
        let mut seen = vec![false; nums.len()];

        let mut ma = 0;
        for i in 0..nums.len() {
            if nums[i] > ma {
                seen[i] = true;
            }
            ma = ma.max(nums[i]);
        }

        ma = 0;
        for i in (0..nums.len()).rev() {
            if nums[i] > ma {
                seen[i] = true;
            }
            ma = ma.max(nums[i]);
        }

        (0..seen.len())
            .filter(|&i| seen[i])
            .map(|i| nums[i])
            .collect()
    }
}
