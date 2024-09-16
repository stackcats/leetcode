impl Solution {
    pub fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut ct = [0; 101];
        let mut ans = Vec::new();
        for n in nums {
            if ct[n as usize] == 1 {
                ans.push(n);
            }
            ct[n as usize] += 1;
        }
        ans
    }
}
