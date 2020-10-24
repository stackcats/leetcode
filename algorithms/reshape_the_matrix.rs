impl Solution {
    pub fn matrix_reshape(nums: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        if nums.len() * nums[0].len() != (r * c) as usize {
            return nums;
        }
        let mut ans = vec![vec![0; c as usize]; r as usize];
        let mut m = 0;
        let mut n = 0;
        for i in 0..nums.len() {
            for j in 0..nums[i].len() {
                ans[m][n] = nums[i][j];
                n += 1;
                if n == c as usize {
                    n = 0;
                    m += 1;
                }
            }
        }
        ans
    }
}
