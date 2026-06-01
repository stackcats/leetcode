impl Solution {
    pub fn min_cost(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut prev = vec![0; nums.len()];
        for i in 1..nums.len() {
            let lft = if i == 1 {
                i32::MAX
            } else {
                nums[i - 1] - nums[i - 2]
            };
            let rht = nums[i] - nums[i - 1];
            prev[i] = prev[i - 1] + if lft <= rht { rht } else { 1 };
        }

        let mut suff = vec![0; nums.len()];
        for i in (0..nums.len() - 1).rev() {
            let lft = nums[i + 1] - nums[i];
            let rht = if i == nums.len() - 2 {
                i32::MAX
            } else {
                nums[i + 2] - nums[i + 1]
            };
            suff[i] = suff[i + 1] + if lft <= rht { 1 } else { lft };
        }

        queries
            .iter()
            .map(|q| {
                let (l, r) = (q[0] as usize, q[1] as usize);
                if l < r {
                    prev[r] - prev[l]
                } else {
                    suff[r] - suff[l]
                }
            })
            .collect()
    }
}
