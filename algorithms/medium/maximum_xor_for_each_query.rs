impl Solution {
    pub fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
        let mut xor = nums.iter().fold(0, |acc, x| acc ^ x);
        let mut ans = Vec::new();
        let mask = (1 << maximum_bit) - 1;
        for i in 0..nums.len() {
            ans.push(xor ^ mask);
            xor ^= nums[nums.len() - i - 1];
        }
        ans
    }
}
