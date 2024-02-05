impl Solution {
    pub fn return_to_boundary_count(nums: Vec<i32>) -> i32 {
        let mut acc = 0;
        let mut ct = 0;
        for n in nums {
            acc += n;
            if acc == 0 {
                ct += 1;
            }
        }
        ct
    }
}
