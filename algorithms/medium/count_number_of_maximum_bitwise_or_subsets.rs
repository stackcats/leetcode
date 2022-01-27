fn dfs(nums: &[i32], cur: i32, max: &mut i32, ans: &mut i32) {
    if cur > *max {
        *max = cur;
        *ans = 1;
    } else if cur == *max {
        *ans += 1;
    }

    for (i, &n) in nums.iter().enumerate() {
        dfs(&nums[i + 1..], cur | n, max, ans)
    }
}

impl Solution {
    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        let mut max = -1;
        let mut ans = 0;
        dfs(&nums, 0, &mut max, &mut ans);
        ans
    }
}
