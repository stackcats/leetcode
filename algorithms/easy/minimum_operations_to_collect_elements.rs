impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut ct = 0;
        let mut ans = 0;
        let mut arr = vec![0; k as usize];
        for i in (0..nums.len()).rev() {
            ans += 1;

            if nums[i] > k {
                continue;
            }

            if arr[nums[i] as usize - 1] == 1 {
                continue;
            }

            arr[nums[i] as usize - 1] = 1;

            ct += 1;
            if ct == k {
                return ans;
            }
        }
        ans
    }
}
