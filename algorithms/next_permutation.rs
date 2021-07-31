impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        for i in (1..nums.len()).rev() {
            if nums[i] > nums[i - 1] {
                for j in (i..nums.len()).rev() {
                    if nums[j] > nums[i - 1] {
                        let t = nums[i - 1];
                        nums[i - 1] = nums[j];
                        nums[j] = t;
                        break;
                    }
                }
                &nums[i..].sort();
                return;
            }
        }

        nums.sort();
    }
}
