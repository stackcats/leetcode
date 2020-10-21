impl Solution {
    pub fn special_array(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        for n in 0..=(nums.len() as i32) {
            let mut i = 0;
            while i < nums.len() && nums[i] < n {
                i += 1;
            }
            if n == (nums.len() - i) as i32 {
                return n;
            }
        }
        -1
    }
}
