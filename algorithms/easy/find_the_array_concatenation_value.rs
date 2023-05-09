impl Solution {
    pub fn find_the_array_conc_val(nums: Vec<i32>) -> i64 {
        let mut ans = 0;
        let mut i = 0;
        let mut j = nums.len() - 1;
        while i <= j {
            let s = if i < j {
                format!("{}{}", nums[i], nums[j])
            } else {
                format!("{}", nums[i])
            };
            ans += s.parse::<i64>().unwrap();
            i += 1;
            j -= 1;
        }
        ans
    }
}
