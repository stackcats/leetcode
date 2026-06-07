impl Solution {
    pub fn count_opposite_parity(nums: Vec<i32>) -> Vec<i32> {
        let mut odds = 0;
        for &n in &nums {
            if n % 2 == 1 {
                odds += 1;
            }
        }
        let mut evens = nums.len() as i32 - odds;
        let mut ans = Vec::new();
        for &n in &nums {
            if n % 2 == 1 {
                odds -= 1;
                ans.push(evens);
            } else {
                evens -= 1;
                ans.push(odds);
            }
        }
        ans
    }
}
