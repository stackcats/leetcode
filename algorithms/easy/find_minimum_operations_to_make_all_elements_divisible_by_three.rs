impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut ct = 0;
        for n in nums {
            let m = n % 3;
            if m == 0 {
                continue;
            }
            ct += 1;
        }
        ct
    }
}
