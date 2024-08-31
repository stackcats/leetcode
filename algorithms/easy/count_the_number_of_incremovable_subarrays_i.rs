fn is_incremovable(v: Vec<i32>) -> bool {
    for i in 1..v.len() {
        if v[i] <= v[i - 1] {
            return false;
        }
    }
    true
}

impl Solution {
    pub fn incremovable_subarray_count(nums: Vec<i32>) -> i32 {
        let mut ct = 0;
        for i in 0..nums.len() {
            for j in i..nums.len() {
                let v = [&nums[0..i], &nums[j + 1..]].concat();
                if is_incremovable(v) {
                    ct += 1;
                }
            }
        }
        ct
    }
}

