impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut arr = heights.clone();
        arr.sort();
        let mut ans = 0;
        for i in 0..arr.len() {
           if heights[i] != arr[i] {
               ans += 1;
            }
        }
        ans
    }
}
