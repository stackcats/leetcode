impl Solution {
    pub fn first_unique_even(nums: Vec<i32>) -> i32 {
        let mut mp = vec![0; 102];
        for &n in &nums {
            mp[n as usize] += 1;
        }
        for n in nums {
            if n % 2 == 0 && mp[n as usize] == 1 {
                return n;
            }
        }
        -1
    }
}
