impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        let mut i = 0;
        while i < nums.len() && nums[i] != 1 {
            i += 1;
        }
        let mut prev = i;
        for j in (i + 1)..nums.len() {
            if nums[j] == 1 {
                if j - prev - 1 < k as usize {
                    return false;
                }
                prev = j;
            }
        }
        true
    }
}
