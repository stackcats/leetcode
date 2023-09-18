impl Solution {
    pub fn number_of_points(nums: Vec<Vec<i32>>) -> i32 {
        let mut arr = [0; 102];
        for n in nums {
            arr[n[0] as usize] += 1;
            arr[(n[1] + 1) as usize] -= 1;
        }
        let mut ct = 0;
        let mut ans = 0;
        for n in arr {
            ct += n;
            if (ct > 0) {
                ans += 1;
            }
        }

        ans
    }
}
