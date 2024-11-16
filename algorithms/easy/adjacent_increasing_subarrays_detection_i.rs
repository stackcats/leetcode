impl Solution {
    pub fn has_increasing_subarrays(nums: Vec<i32>, k: i32) -> bool {
        let mut a = 0;
        let mut b = 1;
        let mut max_k = 0;

        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] {
                b += 1;
            } else {
                a = b;
                b = 1;
            }
            max_k = max_k.max(a.min(b).max(b / 2));
        }

        max_k >= k
    }
}
