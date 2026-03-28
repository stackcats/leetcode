impl Solution {
    pub fn min_absolute_difference(nums: Vec<i32>) -> i32 {
        let mut i = 0;
        while i < nums.len() && nums[i] == 0 {
            i += 1;
        }

        if i == nums.len() {
            return -1;
        }

        let mut a = i;
        let mut ans = usize::MAX;

        while i < nums.len() {
            if nums[i] == 0 {
                i += 1;
                continue;
            }

            if nums[i] != nums[a] {
                ans = ans.min(i - a);
            }

            a = i;
            i += 1;
        }

        if ans == usize::MAX { -1 } else { ans as _ }
    }
}
