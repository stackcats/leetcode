impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut l = 0;
        let mut prod = 1;
        let mut ans = 0;
        for r in 0..nums.len() {
            prod *= nums[r];
            while prod >= k && l <= r {
                prod /= nums[l];
                l += 1;
            }
            ans += r - l + 1;
        }
        ans as _
    }
}
