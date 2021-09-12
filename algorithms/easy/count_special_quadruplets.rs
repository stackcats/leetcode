impl Solution {
    pub fn count_quadruplets(mut nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        for i in 0..nums.len() {
            for j in i+1..nums.len() {
                for k in j+1..nums.len() {
                    let sum = nums[i] + nums[j] + nums[k];
                    for m in k+1..nums.len() {
                        if nums[m] == sum {
                            ans += 1;
                        }
                    }
                }
            }
        }
        ans
    }
}
