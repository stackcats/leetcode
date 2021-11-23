impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = height.len() - 1;
        let mut ans = 0;
        while l < r {
            let h = height[l].min(height[r]);
            let w = (r - l) as i32;
            ans = ans.max(h * w);
            if height[l] <= height[r] {
                l += 1;
            } else {
                r -= 1;
            }
        }
        ans
    }
}
