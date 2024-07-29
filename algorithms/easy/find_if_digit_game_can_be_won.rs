impl Solution {
    pub fn can_alice_win(nums: Vec<i32>) -> bool {
        let mut sum = 0;
        for n in nums {
            if n < 10 {
                sum -= n;
            } else {
                sum += n;
            }
        }
        sum != 0
    }
}
