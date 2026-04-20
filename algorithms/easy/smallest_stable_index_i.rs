impl Solution {
    pub fn first_stable_index(nums: Vec<i32>, k: i32) -> i32 {
        let mut arr = vec![0; nums.len()];
        let mut mi = i32::MAX;
        for i in (0..nums.len()).rev() {
            mi = mi.min(nums[i]);
            arr[i] = mi;
        }

        let mut ma = i32::MIN;
        for i in 0..nums.len() {
            ma = ma.max(nums[i]);
            if ma - arr[i] <= k {
                return i as _;
            }
        }

        -1
    }
}
