impl Solution {
    pub fn minimum_swaps(mut nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut i = 0;
        let mut j = nums.len() - 1;
        while i < j {
            while i < j && nums[i] != 0 {
                i += 1;
            }
            while j > i && nums[j] == 0 {
                j -= 1;
            }
            if i < j {
                ans += 1;
                nums.swap(i, j);
                i += 1;
                j -= 1;
            }
        }
        ans
    }
}
