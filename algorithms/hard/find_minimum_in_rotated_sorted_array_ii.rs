fn find(nums: &[i32], l: usize, r: usize) -> i32 {
    if l == r {
        return nums[l];
    }
    if nums[l] < nums[r] {
        return nums[l];
    }
    let m = (l + r) / 2;
    let lft = find(nums, l, m);
    let rht = find(nums, m + 1, r);
    lft.min(rht)
}

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        find(&nums, 0, nums.len() - 1)
    }
}
