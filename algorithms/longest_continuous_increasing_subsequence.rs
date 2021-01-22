impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let mut ct = 1;
        let mut ans = 1;
        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] {
                ct += 1;
                if ans < ct {
                    ans = ct;
                }
            } else {
                ct = 1;
            }
        }
        ans
    }
}
