impl Solution {
    pub fn dominant_index(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 0;
        }
        let mut a = nums[0];
        let mut b = nums[1];
        let mut ans = 0;
        if a < b {
            let t = a;
            a = b;
            b = t;
            ans = 1;
        }

        for i in 2..nums.len() {
            if a < nums[i] {
                b = a;
                a = nums[i];
                ans = i;
            } else if b < nums[i] {
                b = nums[i];
            }
        }
        if a >= 2 * b {
            return ans as i32;
        }
        -1
    }
}
