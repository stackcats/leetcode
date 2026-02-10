impl Solution {
    pub fn longest_balanced(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        for i in 0..nums.len() {
            let mut even = 0;
            let mut odd = 0;
            let mut mp = vec![0; 100005];

            for j in i..nums.len() {
                if mp[nums[j] as usize] == 0 {
                    mp[nums[j] as usize] = 1;
                    if nums[j] % 2 == 0 {
                        even += 1;
                    } else {
                        odd += 1;
                    }
                }

                if even == odd {
                    ans = ans.max(j - i + 1);
                }
            }
        }
        ans as _
    }
}
