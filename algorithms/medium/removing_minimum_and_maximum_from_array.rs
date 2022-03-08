impl Solution {
    pub fn minimum_deletions(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 1;
        }
        let mut min_index = 0;
        let mut max_index = 0;
        let mut max = nums[0];
        let mut min = nums[0];
        for (i, &n) in nums.iter().enumerate() {
            if max < n {
                max = n;
                max_index = i;
            }

            if min > n {
                min = n;
                min_index = i;
            }
        }

        let mut left = min_index.min(max_index);
        let mut right = min_index.max(max_index);
        let mut len = nums.len();
        let mut ans = 0;
        if left + 1 < len - right {
            ans += left + 1;
            len -= left + 1;
            right -= left + 1;
            ans += (right + 1).min(len - right);
        } else {
            ans += len - right;
            len = right;
            ans += (left + 1).min(len - left);
        }

        ans as i32
    }
}
