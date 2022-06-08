fn rev(arr: &mut Vec<i32>, mut i: usize, mut j: usize) {
    while i < j {
        arr.swap(i, j);
        i += 1;
        j -= 1;
    }
}

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let k = (k as usize) % nums.len();
        if k == 0 {
            return;
        }
        rev(nums, 0, nums.len() - 1);
        rev(nums, 0, k - 1);
        rev(nums, k, nums.len() - 1);
    }
}
