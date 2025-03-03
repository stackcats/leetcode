impl Solution {
    pub fn largest_integer(nums: Vec<i32>, k: i32) -> i32 {
        let mut ct = vec![0; 51];
        for &n in &nums {
            ct[n as usize] += 1;
        }

        match k {
            1 => {
                for i in (0..51).rev() {
                    if ct[i] == 1 {
                        return i as i32;
                    }
                }
                -1
            }
            _ if k == nums.len() as i32 => {
                for i in (0..51).rev() {
                    if ct[i] > 0 {
                        return i as i32;
                    }
                }
                -1
            }
            _ => {
                let mut ans = -1;
                if ct[nums[0] as usize] == 1 {
                    ans = ans.max(nums[0]);
                }
                if ct[nums[nums.len() - 1] as usize] == 1 {
                    ans = ans.max(nums[nums.len() - 1]);
                }
                ans
            }
        }
    }
}
