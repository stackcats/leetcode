impl Solution {
    pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
        let mut ans = Vec::new();
        for i in 0..nums.len() {
            let num = nums[i].abs();
            let ndx = (num - 1) as usize;
            if nums[ndx] < 0 {
                ans.push(num);
            } else {
                nums[ndx] *= -1;
            }
        }
        ans
    }
}
